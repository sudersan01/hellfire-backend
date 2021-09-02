use crate::models::schemas::{error_schema::HFError, response_schema::HFResponse};

pub type HFResult<T> = Result<HFResponse<T>, HFError>;