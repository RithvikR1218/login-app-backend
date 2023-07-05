use diesel::{pg::PgConnection};
use diesel::prelude::*;
use crate::DbError;
use crate::services::db::models::user::models::{NewUser,User, CreateUser, UpdateUser,Login,Response};

pub fn create_user(conn: &mut PgConnection, info: &CreateUser) -> Result<User,DbError> {
    use crate::schema::users;
    let new_user = NewUser { 
        user_name: &info.user_name
        , user_email: &info.user_email
        ,user_password: &info.user_password 
    };
    let create = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)?;
    Ok(create)
}

pub fn get_all_users(conn: &mut PgConnection) -> Result<Vec<User>,DbError> {
    use crate::schema::users::dsl::*;
    let items = users.load::<User>(conn)?;
    Ok(items)
}

pub fn get_users(conn: &mut PgConnection,email: &str) -> Result<User,DbError> {
    use crate::schema::users::dsl::*;

    let user_result = users.filter(user_email.eq(email)).first::<User>(conn);
    match user_result {
        Ok(user) => Ok(user),
        Err(diesel::result::Error::NotFound) => {
            let error_message = format!("No users found with email: {}", email);
            // let custom_error = Response { message: error_message };
            let test: DbError = String::from(error_message).into();
            Err(test)
        }
        Err(err) => Err(Box::new(err)),
    }
}

pub fn update_user(conn: &mut PgConnection,email: &str,update_details: &UpdateUser) -> Result<User,DbError> {
    use crate::schema::users::dsl::*;
    let person = get_users(conn,email)?;
  
  let mut final_password = person.user_password;
  match &update_details.user_password {
    Some(x) => final_password = x.to_string(),
    None => {},
  }

  let mut final_name = person.user_name;
  match &update_details.user_name {
    Some(x) => final_name = x.to_string(),
    None => {},
  }

    let item = diesel::update(users.filter(id.eq(person.id)))
    .set((user_name.eq(final_name),user_password.eq(final_password)))
    .get_result::<User>(conn)?;
    Ok(item)
}

pub fn delete_user(conn: &mut PgConnection,email: &str) -> Result<Response,DbError> {
    use crate::schema::users::dsl::*;
    let person = get_users(conn,email)?;
    diesel::delete(users.find(person.id)).execute(conn)?;
    let message = Response {message: String::from("Delete Successful")};
    Ok(message)
}

pub fn check_login(conn: &mut PgConnection,login: &Login) -> Result<Response,DbError> {

    let person = get_users(conn,&login.user_email)?;
    
    if person.user_password == login.user_password {
        let final_message = Response {message: String::from("Login successful")};
        return Ok(final_message);
    } else {
        let test: DbError = String::from("Password Incorrect").into();
        return Err(test);
    }
}