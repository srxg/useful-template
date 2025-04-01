pub struct User<Id = ()> {
    id: Id,
    details: UserDetails
}

pub struct UserDetails {
    name: String,
}