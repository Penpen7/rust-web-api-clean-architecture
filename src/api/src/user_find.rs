use actix_web::{Error, HttpRequest, HttpResponse};
use usecase::UserFindUsecase;

pub async fn user_find(req: HttpRequest) -> actix_web::Result<HttpResponse, Error> {
    let container = crate::di::create_di_conntainer();
    let req = usecase::UserFindUsecaseRequest::new("hoge@example.com".to_string()).unwrap();
    let res = container.user_find_usecase.handle(req);
    match res {
        Ok(res) => Ok(HttpResponse::Ok().body(res.get_user_name().to_string())),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
