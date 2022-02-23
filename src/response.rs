use worker::*;

pub trait ResponseWithContentType {
    fn content_type(self, content_type: &str) -> Result<Response>;
}

impl ResponseWithContentType for Response {
    fn content_type(self, content_type: &str) -> Result<Response> {
        let mut headers = self.headers().clone();
        headers.set("Content-Type", content_type)?;

        Ok(self.with_headers(headers))
    }
}
