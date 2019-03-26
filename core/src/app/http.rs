
pub struct HttpResponse {
    pub error_code: Option<i32>,
    pub url: String,
    pub data: String,
}

pub trait HttpListener {
    fn on_completed(&mut self, response: HttpResponse);
}

pub trait HttpClient {
    fn fetch<T>(&mut self, url: &str, completion: &mut T) where T: HttpListener;
}