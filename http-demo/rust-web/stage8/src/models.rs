use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TeacherRegisterForm {
    pub name: String,
    pub image_url: String,
    pub profile: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TeacherResponse {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}
