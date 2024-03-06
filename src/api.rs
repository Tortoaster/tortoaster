use crate::template::{Index, TemplateResponse};

pub async fn index() -> TemplateResponse<Index> {
    TemplateResponse(Index)
}
