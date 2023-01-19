use actix_web::{Error, HttpResponse};
mod di;
pub mod user_find;

pub async fn healthcheck() -> actix_web::Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("ok".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::ServiceResponse;
    use actix_web::{http, test};

    #[actix_web::test]
    async fn health_check_work() {
        let req = test::TestRequest::default().to_http_request();
        let res = healthcheck().await.unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);
        let srv_resp = ServiceResponse::new(req, res);
        let bytes = test::read_body(srv_resp).await;
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body.contains("ok"));
    }
}
