use chrono::DateTime;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HomeModel {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub guests: Option<String>,
    pub bed_rooms: Option<String>,
    pub bath_rooms: Option<String>,
    pub country: Option<String>,
    pub photo: Option<String>,
    pub price: Option<i32>,
    pub category_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateHomeModel {
    pub title: Option<String>,
    pub description: Option<String>,
    pub guests: Option<String>,
    pub bed_rooms: Option<String>,
    pub bath_rooms: Option<String>,
    pub country: Option<String>,
    pub photo: Option<String>,
    pub price: Option<i32>,
    pub category_name: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug,PartialEq,Clone)]
pub struct UpdateHomeModel {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub guests: Option<String>,
    pub bed_rooms: Option<String>,
    pub bath_rooms: Option<String>,
    pub country: Option<String>,
    pub photo: Option<String>,
    pub price: Option<i32>,
    pub category_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub user_id: Option<String>,
}

