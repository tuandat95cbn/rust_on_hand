use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user_login)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserLogin {
	pub user_login_id : String,
}