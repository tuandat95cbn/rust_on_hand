use diesel::dsl::count;
use diesel::prelude::*;
use diesel::{dsl::count_star, PgConnection, QueryDsl};

use crate::schema::user_login;

use super::{common::Page, models::UserLogin};
use crate::schema::user_login::dsl::*;

pub fn users(
    conn: &mut PgConnection,
    size: &usize,
    page: &usize,
) -> Result<Page<UserLogin>, diesel::result::Error> {
    let users = user_login
        .limit(*size as i64)
        .offset(*page as i64)
        .select(UserLogin::as_select())
        .load(conn)?;
    let total: i64 = user_login.select(count(user_login_id)).first(conn)?;
    return Ok(Page {
        content: users,
        total: total as usize,
        page: page.to_owned(),
        size: size.to_owned(),
    });
}
