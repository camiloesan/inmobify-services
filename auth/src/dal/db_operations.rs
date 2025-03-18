use crate::dal::repository::AuthRepository;
use crate::dto::login::LoginRequest;
use crate::dto::login_response::LoginResponse;
use crate::dal::sch_models::User;
use diesel::prelude::*;

pub struct PgAuth {}

impl AuthRepository for PgAuth {
    fn login(login_data: LoginRequest, conn: &mut PgConnection) -> Option<(LoginResponse, String)> {
        use crate::dal::schema::users::dsl::*;

        let user = users
            .filter(email.eq(login_data.email))
            .filter(password.eq(login_data.password))
            .first::<User>(conn)
            .optional()
            .ok()?;

        user.map(|user| {
            let response = LoginResponse {
                email: user.email,
                user_type_id: user.user_type_id,
                name: user.name,
                last_name: user.last_name,
                phone: user.phone,
            };
            (response, user.id.to_string())
        })
    }
}