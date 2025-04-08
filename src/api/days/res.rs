use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Activity {
    pub id: i32,
    pub hours: i32,
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Debug)]
pub struct Day {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub occupation: String,
    pub country: String,
    pub created_at: String,
    pub activities: Vec<Activity>,
}

#[derive(Serialize)]
pub struct Response {
    pub days: Vec<Day>,
}
