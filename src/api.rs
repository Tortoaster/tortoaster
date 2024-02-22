use crate::template::{Index, Projects, TemplateResponse};

pub async fn index() -> TemplateResponse<Index> {
    TemplateResponse(Index)
}

pub async fn projects() -> TemplateResponse<Projects> {
    TemplateResponse(Projects)
}
