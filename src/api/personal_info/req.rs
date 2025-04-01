use serde::Deserialize;

use super::Sex;

#[derive(Deserialize)]
pub struct Schedule {
    pub start_time: u8,
    pub label: String,
}

#[derive(Deserialize)]
pub struct PostRequestBody {
    pub occupation: String,
    pub sex: Sex,
    pub age: u8,
    pub schedule: Vec<Schedule>,
}
