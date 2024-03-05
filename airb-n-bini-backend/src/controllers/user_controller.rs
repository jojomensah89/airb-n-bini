use crate::{models::user_model::CreateUserModel, utils::api_error::APIError};
use axum::{http::StatusCode, Extension, Json};
use entity::user::{self as User, ActiveModel};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user): Json<CreateUserModel>,
) -> Result<(), APIError> {
    let user_exists = User::Entity::find()
        .filter(entity::user::Column::Email.eq(user.email.clone()))
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?;

    if user_exists != None {
        return Err(APIError {
            message: "User already exists".to_owned(),
            status_code: StatusCode::CONFLICT,
            error_code: Some(40),
        });
    }

    // let mut model = User::ActiveModel{
    //     id:ActiveValue::NotSet,
    //     email:ActiveValue::NotSet,
    //     first_name:ActiveValue::NotSet,
    //     last_name:ActiveValue::NotSet,
    //     profile_image:ActiveValue::NotSet,
    // };
    let user_model = ActiveModel {
        id: Set(String::from(user.id)),
        email: Set(String::from(user.email)),
        first_name: Set(String::from(user.first_name)),
        last_name: Set(String::from(user.last_name)),
        profile_image: Set(user.profile_image.as_ref().map(|image| {
            // Assuming the image is always valid (handle potential None if necessary)
            String::from(image)
        })),
    };
    let user = user_model.insert(&db).await.map_err(|err| APIError {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(50),
    })?;
    println!("User {} inserted ", user.first_name);
    Ok(())
}
