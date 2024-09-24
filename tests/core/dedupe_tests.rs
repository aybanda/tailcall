

#[cfg(test)]
mod tests {
    use crate::core::http::{RequestTemplate, handle_http_request};

    #[test]
    fn test_dedupe_http_request() {
        let template = RequestTemplate::new(/* existing parameters */, true);
        let context = template.to_context();
        let response1 = handle_http_request(&context);
        let response2 = handle_http_request(&context);
        assert_eq!(response1, response2);
    }


}