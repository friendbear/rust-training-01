

pub struct RequestBuilder<'a> {
    base_url: &'a str,
    method: Method,
    params: Option<ParamList>
    query: Option<String>,
    body: Option<(Body, &'static str)>,
    addon: OAuthAddOn,
}

impl<'a> RequestBuilder<'a> {
    pub fn new(method: Method, base_url: &'a str) -> Self {
        RequestBuilder {
            base_url,
            method,
            params: None,
            query: None,
            body: None,
            addon: OAuthAddOn::None,
        }
    }
}