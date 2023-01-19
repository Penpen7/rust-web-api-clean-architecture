pub struct UserFindUsecase<T>
where
    T: crate::repository::UserRepository,
{
    user_repository: T,
}

impl<T> UserFindUsecase<T>
where
    T: crate::repository::UserRepository,
{
    pub fn new(user_repository: T) -> Self {
        UserFindUsecase { user_repository }
    }
}

impl<T> usecase::UserFindUsecase for UserFindUsecase<T>
where
    T: crate::repository::UserRepository,
{
    fn handle(
        &self,
        req: usecase::UserFindUsecaseRequest,
    ) -> Result<usecase::UserFindUsecaseResponse, String> {
        let user = self
            .user_repository
            .find_by_mail_address(req.get_mail_address());
        Ok(usecase::UserFindUsecaseResponse::new(&user))
    }
}
