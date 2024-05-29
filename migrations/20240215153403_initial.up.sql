CREATE TABLE projects
(
    id           VARCHAR(128) PRIMARY KEY,
    name         VARCHAR(32)              NOT NULL,
    preview      VARCHAR(300)             NOT NULL,
    thumbnail_id UUID                     NOT NULL,
    project_url  VARCHAR(2000),
    date_posted  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    deleted      BOOLEAN                  NOT NULL DEFAULT FALSE
);

CREATE TABLE comments
(
    id          SERIAL PRIMARY KEY,
    user_id     VARCHAR(36) REFERENCES keycloak.user_entity ON DELETE CASCADE NOT NULL,
    project_id  VARCHAR(128) REFERENCES projects ON DELETE CASCADE            NOT NULL,
    message     TEXT                                                          NOT NULL,
    date_posted TIMESTAMP WITH TIME ZONE                                      NOT NULL DEFAULT now(),
    deleted     BOOLEAN                                                       NOT NULL DEFAULT FALSE
);
