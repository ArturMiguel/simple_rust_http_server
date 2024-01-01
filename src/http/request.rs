use super::method::Method;

pub struct Request {
  method: String,
  path: Option<String>,
  query_params: Method
}
