use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TaskRequest {
    pub name: String,
    pub description: String,
    pub rewardsat: i64,
    pub link: String,
    pub creator_name: String
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserRequest {

    pub name: String,
    pub address: String,
    pub balance: i64,
    pub badge: String
    
}


