use serde::Deserialize;

#[derive(Deserialize)]
pub struct Activity {
    pub hours: u8,
    pub name: String,
    pub color: String,
}

#[derive(Deserialize)]
pub struct PostRequestBody {
    pub occupation: String,
    pub title: String,
    pub description: Option<String>,
    pub country: String,
    pub activities: Vec<Activity>,
}
