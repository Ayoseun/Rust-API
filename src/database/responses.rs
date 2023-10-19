use rocket::serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize,FromRow, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub rewardsat: i64,
    pub link: String,
    pub creator_name: String
}
#[derive(Serialize, FromRow, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub address: String,
    pub balance: i64,
    pub badge: String
   
}