use serde::Serialize;

use super::Sex;

#[derive(Serialize)]
pub struct Schedule {
    pub id: i32,
    pub start_time: i32,
    pub label: String,
}

#[derive(Serialize)]
pub struct PersonalInfo {
    pub id: i32,
    pub occupation: String,
    pub sex: Sex,
    pub age: i32,
    pub schedule: Vec<Schedule>,
}

#[derive(Serialize)]
pub struct Response {
    pub personal_info_list: Vec<PersonalInfo>,
}
