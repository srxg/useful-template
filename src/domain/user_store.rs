// Define Repository traits in the domain

pub trait UserStore {
    async fn get_user(&self, user_id: &String) -> User<String>;
}