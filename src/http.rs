use minihttp::request::Request;
use std::collections::HashMap;

//Basic http methods
pub enum HttpMethod {
    Get,
    Post,
}

//Represents Request object
pub struct HttpRequest {
    pub url: String,
    pub method: HttpMethod,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn make_request(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut requests_obj = Request::new(&self.url)?;

        let method = match self.method {
            HttpMethod::Get => requests_obj.get(),

            HttpMethod::Post => {
                let body = &self.body.clone().unwrap();
                requests_obj.body_str(&body);

                //set content type to application/json
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                requests_obj.headers(headers);

                requests_obj.post()
            }
        };

        let data = method.send()?;
        Ok(data.text())
    }
}
