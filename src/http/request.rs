use super::method::Method;

pub struct Request {
    path: String,
    query_method: Option<String>,
    method: Method,
}