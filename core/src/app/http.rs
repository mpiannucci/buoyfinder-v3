
pub struct HttpResponse {
    error_code: Option<i32>,
    url: String,
    data: String,
}

pub trait HttpClient {
    fn fetch<F>(url: String, completion: F) where F: Fn(HttpResponse);
}