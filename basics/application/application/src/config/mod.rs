use std::convert::TryFrom;

use actix_web::http::header::IntoHeaderValue;
use actix_web::http::header::{IntoHeaderPair, InvalidHeaderName, InvalidHeaderValue};
use actix_web::http::Error as HttpError;
use actix_web::http::{HeaderName, HeaderValue};
use actix_web::{get, web, HttpResponse, Responder};

#[derive(Debug)]
pub enum InvalidHeaderPart {
    Name(InvalidHeaderName),
    Value(InvalidHeaderValue),
}

impl From<InvalidHeaderPart> for HttpError {
    fn from(part_err: InvalidHeaderPart) -> Self {
        match part_err {
            InvalidHeaderPart::Name(err) => err.into(),
            InvalidHeaderPart::Value(err) => err.into(),
        }
    }
}

struct StringHeader<'a> {
    raw: &'a str,
}

impl IntoHeaderPair for StringHeader<'_> {
    fn try_into_header_pair(self) -> Result<(HeaderName, HeaderValue), Self::Error> {
        let hdr = self.raw.to_lowercase();
        let (name, value) = hdr.split_once(":").expect("Invalid Header String");
        let name = HeaderName::try_from(name).map_err(InvalidHeaderPart::Name)?;
        let value = value
            .try_into_value()
            .map_err(|err| InvalidHeaderPart::Value(err.into()))?;
        Ok((name, value))
    }

    type Error = InvalidHeaderPart;
}

async fn hello() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("plain/text")
        .append_header(StringHeader {
            raw: "X-Hdr:Sample",
        })
        .body("Hello, world!")
}

#[get("/")]
async fn idx_service() -> impl Responder {
    HttpResponse::Ok().body("Hello Rust!")
}

pub fn app_config(service_config: &mut web::ServiceConfig) {
    service_config.service(
        web::scope("/app")
            .service(idx_service)
            .route("/hello", web::get().to(hello)),
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{
        http::{self, header, HeaderMap},
        test,
    };

    #[actix_rt::test]
    async fn test_hello_ok() {
        let _req = test::TestRequest::get().to_http_request();
        let resp = hello().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("plain/text"));
        headers.insert(
            HeaderName::from_static("x-hdr"),
            HeaderValue::from_static("sample"),
        );
        assert_eq!(resp.headers().len(), headers.len());
        println!("{:?}", headers);
        println!("{:?}", resp.headers());
        for hdr in resp.headers() {
            let name = hdr.0.to_owned();
            assert!(headers.contains_key(name))
        }
    }
}
