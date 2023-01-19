pub struct DIContainer<T>
where
    T: usecase::UserFindUsecase,
{
    pub user_find_usecase: T,
}

impl<T: usecase::UserFindUsecase> DIContainer<T> {
    fn new(user_find_usecase: T) -> DIContainer<T> {
        DIContainer { user_find_usecase }
    }
}

pub fn create_di_conntainer() -> DIContainer<impl usecase::UserFindUsecase> {
    let user_repository = repository::mock_user::MockUserRepository::new();
    let user_find = application::UserFindUsecase::new(user_repository);
    DIContainer::new(user_find)
}
