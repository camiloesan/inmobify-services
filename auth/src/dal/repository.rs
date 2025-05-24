use crate::dto::login::LoginRequest;
use crate::dto::login_response::LoginResponse;
use diesel::PgConnection;

pub trait AuthRepository {
    fn login(login_data: LoginRequest, conn: &mut PgConnection) -> Option<(LoginResponse, String)>;
}