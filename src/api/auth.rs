use askama::Template;
use axum::{
    extract::{rejection::FormRejection, State},
    Form,
};
use axum_valid::{Valid, ValidRejection};
use scrypt::{
    password_hash,
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use serde::Deserialize;
use validator::{Validate, ValidationErrors};

use crate::{
    model::auth::{NewUser, User},
    render::Render,
    repository::auth::AuthRepository,
};

#[derive(Default, Template)]
#[template(path = "auth/register_form.html")]
pub struct RegisterForm {
    errors: ValidationErrors,
}

impl RegisterForm {
    fn validation_errors(_: ValidRejection<FormRejection>) -> Self {
        let errors = todo!();
        Self { errors }
    }

    /// Creates a form displaying errors encountered when adding a user to the
    /// database.
    ///
    /// Some possible causes:
    ///
    /// * Username already exists
    /// * Email address already exists
    /// * Database cannot be reached
    fn database_error(_: sqlx::Error) -> Self {
        let errors = todo!();
        Self { errors }
    }
}

#[derive(Template)]
#[template(path = "auth/register_confirmation.html")]
pub struct RegisterConfirmation;

#[derive(Deserialize, Validate)]
pub struct RegisterData {
    #[validate(length(min = 1, max = 32))]
    username: String,
    #[validate(email)]
    email_address: String,
    #[validate(length(min = 8), must_match(other = "confirm_password"))]
    password: String,
    confirm_password: String,
}

pub async fn register(
    State(repo): State<AuthRepository>,
    optional_form_data: Option<Result<Valid<Form<RegisterData>>, ValidRejection<FormRejection>>>,
) -> Result<Render<RegisterConfirmation>, Render<RegisterForm>> {
    // Return empty form if no data is provided
    let form_data = optional_form_data.ok_or(RegisterForm::default())?;

    // Return simple validation errors if the structure of the data has problems
    let Valid(Form(register_data)) = form_data.map_err(RegisterForm::validation_errors)?;

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Scrypt
        .hash_password(register_data.password.as_bytes(), &salt)
        .expect("failed to create PHC string")
        .to_string();

    let user = NewUser {
        username: register_data.username,
        email_address: register_data.email_address,
        password_hash,
    };

    // Return error if the username or email address already exists
    repo.add_user(&user)
        .await
        .map_err(RegisterForm::database_error)?;

    // TODO: Send verification email

    Ok(Render(RegisterConfirmation))
}

#[derive(Default, Template)]
#[template(path = "auth/login_form.html")]
pub struct LoginForm {
    errors: ValidationErrors,
}

impl LoginForm {
    fn validation_errors(_: ValidRejection<FormRejection>) -> Self {
        let errors = todo!();
        Self { errors }
    }

    fn database_error(_: sqlx::Error) -> Self {
        let errors = todo!();
        Self { errors }
    }

    fn username_not_found_error() -> Self {
        let errors = todo!();
        Self { errors }
    }

    fn wrong_password_error(_: password_hash::Error) -> Self {
        let errors = todo!();
        Self { errors }
    }

    fn user_not_verified_error() -> Self {
        let errors = todo!();
        Self { errors }
    }
}

#[derive(Template)]
#[template(path = "auth/login_confirmation.html")]
pub struct LoginConfirmation {
    user: User,
}

#[derive(Deserialize, Validate)]
pub struct LoginData {
    #[validate(length(min = 1, max = 32))]
    username: String,
    #[validate(length(min = 8))]
    password: String,
}

pub async fn login(
    State(repo): State<AuthRepository>,
    optional_form_data: Option<Result<Valid<Form<LoginData>>, ValidRejection<FormRejection>>>,
) -> Result<Render<LoginConfirmation>, Render<LoginForm>> {
    // Return empty form if no data is provided
    let form_data = optional_form_data.ok_or(LoginForm::default())?;

    // Return simple validation errors if the structure of the data has problems
    let Valid(Form(login_data)) = form_data.map_err(LoginForm::validation_errors)?;

    // Return error if the user doesn't exist
    // TODO: If this returns the same error as an incorrect password, the next
    //  section should still execute on error to prevent timing attacks
    let user = repo
        .get_user_by_username(&login_data.username)
        .await
        .map_err(LoginForm::database_error)?
        .ok_or(LoginForm::username_not_found_error())?;

    // Return error if the password is incorrect
    let hash = PasswordHash::new(&user.password_hash).expect("invalid PHC string");
    Scrypt
        .verify_password(login_data.password.as_bytes(), &hash)
        .map_err(LoginForm::wrong_password_error)?;

    // Return error if user hasn't verified their email address
    user.email_verified
        .then_some(())
        .ok_or_else(LoginForm::user_not_verified_error)?;

    let response = LoginConfirmation { user };
    Ok(Render(response))
}
