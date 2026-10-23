use std::marker::PhantomData;

// State marker types which replaces the 'S' type
struct NoUrl;
struct HasUrl;

struct RequestBuilder<S> {
    url: Option<String>,
    _marker: PhantomData<S>,
}

impl RequestBuilder<NoUrl> {
    pub fn new() -> Self {
        RequestBuilder {
            url: None,
            _marker: PhantomData,
        }
    }

    pub fn url(self, url: String) -> RequestBuilder<HasUrl> {
        RequestBuilder {
            url: Some(url),
            _marker: PhantomData,
        }
    }
}


fn main() {
    let request_without_url = RequestBuilder::new();
    let request_with_url = RequestBuilder::new().url("https://google.com".to_string());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn assert_no_url(){
        let request_without_url = RequestBuilder::new();
        assert_eq!(request_without_url.url, None);

    }

    #[test]
    fn assert_url(){
        let request_without_url = RequestBuilder::new().url("https://google.com".to_string());
        assert_eq!(request_without_url.url, Some("https://google.com".to_string()));
    }
}
