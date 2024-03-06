use crate::{
    models::home_model::{CreateHomeModel, HomeModel},
    utils::api_error::APIError,
};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use chrono::Utc;
use entity::home::{self as Home, ActiveModel};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use uuid::Uuid;

pub async fn create_home(
    Extension(db): Extension<DatabaseConnection>,
    Json(home): Json<CreateHomeModel>,
) -> Result<(), APIError> {
    println!("incoming home is: {:?}", home);

    // let mut model = User::ActiveModel{
    //     id:ActiveValue::NotSet,
    //     email:ActiveValue::NotSet,
    //     first_name:ActiveValue::NotSet,
    //     last_name:ActiveValue::NotSet,
    //     profile_image:ActiveValue::NotSet,
    // };

    let home_model = ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(home.title.as_ref().map(|title| String::from(title))),
        description: Set(home
            .description
            .as_ref()
            .map(|description| String::from(description))),
        guests: Set(home.guests.as_ref().map(|guests| String::from(guests))),
        bed_rooms: Set(home
            .bed_rooms
            .as_ref()
            .map(|bed_rooms| String::from(bed_rooms))),
        bath_rooms: Set(home
            .bath_rooms
            .as_ref()
            .map(|bath_rooms| String::from(bath_rooms))),
        country: Set(home.country.as_ref().map(|country| String::from(country))),
        photo: Set(home.photo.as_ref().map(|photo| String::from(photo))),
        price: Set(home.price.as_ref().map(|price| !price)),
        category_name: Set(home
            .category_name
            .as_ref()
            .map(|category_name| String::from(category_name))),
        created_at: Set(Utc::now().naive_utc()),

        user_id: Set(home.user_id),
    };
    let _home = home_model.insert(&db).await.map_err(|err| APIError {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(50),
    })?;
    println!("Home  inserted ");
    Ok(())
}

pub async fn get_homes(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<HomeModel>>, APIError> {
    let homes: Vec<HomeModel> = Home::Entity::find()
        .all(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .into_iter()
        .map(|item| HomeModel {
            id: item.id,
            title: item.title,
            description: item.description,
            guests: item.guests,
            bed_rooms: item.bed_rooms,
            bath_rooms: item.bath_rooms,
            country: item.country,
            photo: item.photo,
            price: item.price,
            category_name: item.category_name,
            created_at: item.created_at,
            user_id: item.user_id,
        })
        .collect();

    Ok(Json(homes))
}

// pub async fn get_home_by_id(
//     Extension(db): Extension<DatabaseConnection>,
//     Json(home): Json<CreateHomeModel>,
// ) -> Result<(), APIError> {
//     Ok(())
// }

// pub async fn update_home(
//     Extension(db): Extension<DatabaseConnection>,
//     Json(home): Json<UpdateHomeModel>,
// ) -> Result<(), APIError> {
//     let mut home_exists:ActiveModel = Home::Entity::find()
//         .filter(entity::home::Column::Id.eq(home.id.clone()))
//         .one(&db)
//         .await
//         .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?
//     .ok_or(APIError { message: "Not Found".to_owned(), status_code: StatusCode::NOT_FOUND, error_code: Some(44) })?
//     .into();

//     // if home_exists == None {
//     //     return Err(APIError {
//     //         message: "Home doesn't exists".to_owned(),
//     //         status_code: StatusCode::CONFLICT,
//     //         error_code: Some(40),
//     //     });
//     // }

//     //must use Set here
//     home_exists = home.clone();

//      home_exists.update(&db).await
//     .map_err(|err| APIError { message: err.to_string(), status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)})?;

//     Ok(())
// }

pub async fn delete_home(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<(), APIError> {
    let _home = entity::home::Entity::find()
        .filter(entity::home::Column::Id.eq(id))
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "Not Found".to_owned(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        })?;

    entity::home::Entity::delete_by_id(id)
        .exec(&db)
        .await
        .map_err(|err| APIError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?;

    Ok(())
}
