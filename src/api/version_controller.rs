use crate::services::version_service::get_cargo_version;
use actix_web::{web::Bytes, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    version: String,
}

pub async fn get_version() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(Version {
            version: get_cargo_version(),
        })
}
trait BodyTest {
    fn as_str(&self) -> &str;
}

impl BodyTest for Bytes {
    fn as_str(&self) -> &str {
        std::str::from_utf8(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{body::to_bytes, http::header, test, web, App};
    use serde_json::Value;

    #[actix_web::test]
    async fn test_greet_get() {
        let mut app =
            test::init_service(App::new().route("/api/v1/version", web::get().to(get_version)))
                .await;

        let req = test::TestRequest::get()
            .insert_header(header::ContentType::json())
            .uri("/api/v1/version")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        let headers = resp.headers().get("content-type").unwrap();

        assert_eq!(headers, "application/json");

        let body = to_bytes(resp.into_body()).await.unwrap();
        let done: Value = serde_json::from_str(body.as_str()).unwrap();

        assert_eq!(done["version"], env!("CARGO_PKG_VERSION"));
    }
}
