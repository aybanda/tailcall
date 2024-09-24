pub use cache::*;
pub use data_loader::*;
pub use data_loader_request::*;
use headers::HeaderValue;
pub use method::Method;
pub use query_encoder::QueryEncoder;
pub use request_context::RequestContext;
pub use request_handler::{handle_request, API_URL_PREFIX};
pub use request_template::RequestTemplate;
pub use response::*;

mod cache;
mod data_loader;
mod data_loader_request;
mod method;
mod query_encoder;
mod request_context;
mod request_handler;
mod request_template;
mod response;
pub mod showcase;
mod telemetry;

pub static TAILCALL_HTTPS_ORIGIN: HeaderValue = HeaderValue::from_static("https://tailcall.run");
pub static TAILCALL_HTTP_ORIGIN: HeaderValue = HeaderValue::from_static("http://tailcall.run");

#[derive(Default, Clone, Debug)]
/// User can configure the filter/interceptor
/// for the http requests.
pub struct HttpFilter {
    pub on_request: String,
}

impl HttpFilter {
    pub fn new(on_request: &str) -> Self {
        HttpFilter { on_request: on_request.to_owned() }
    }
}

lazy_static! {
    static ref CACHE: Arc<Mutex<HashMap<String, HTTPResponse>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn handle_http_request(request: &HTTPRequest) -> HTTPResponse {
    if request.dedupe && is_duplicate_request(request) {
        return get_cached_response(request);
    }
    let response = make_http_request(request);
    if request.dedupe {
        cache_response(request, &response);
    }
    response
}

fn is_duplicate_request(request: &HTTPRequest) -> bool {
    let cache = CACHE.lock().unwrap();
    cache.contains_key(&request.cache_key())
}

fn get_cached_response(request: &HTTPRequest) -> HTTPResponse {
    let cache = CACHE.lock().unwrap();
    cache.get(&request.cache_key()).unwrap().clone()
}

fn cache_response(request: &HTTPRequest, response: &HTTPResponse) {
    let mut cache = CACHE.lock().unwrap();
    cache.insert(request.cache_key(), response.clone());
}

pub use request_context::RequestContext;
pub use request_handler::handle_http_request;
pub use request_template::RequestTemplate;
