use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct User{
    pub username:String,
    pub password:String,
}