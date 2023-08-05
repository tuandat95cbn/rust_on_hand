use diesel::{Insertable, PgConnection};

use crate::schema::user_login;
use diesel::SelectableHelper;
use diesel::RunQueryDsl;

use super::models::UserLogin;
#[derive(Insertable)]
#[diesel(table_name = crate::schema::user_login)]
pub struct NewUser<'a> {
    pub user_login_id: &'a str,
}
pub fn create_user(conn: &mut PgConnection,user_name: &str)-> Result<UserLogin, diesel::result::Error>{
    let new_user = NewUser{user_login_id:user_name};
    return diesel::insert_into(user_login::table)
        .values(&new_user)
        .returning(UserLogin::as_returning())
        .get_result(conn)
    
}

