use actix_web::{get, post, web, HttpResponse};
use entity::{personal_info, schedule, sea_orm_active_enums};
use log::error;
use req::PostRequestBody;
use res::{PersonalInfo, Response, Schedule};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::AppState;

mod req;
mod res;

#[derive(Serialize, Deserialize, Clone)]
pub enum Sex {
    Male,
    Female,
}

impl From<sea_orm_active_enums::Sex> for Sex {
    fn from(value: sea_orm_active_enums::Sex) -> Self {
        match value {
            sea_orm_active_enums::Sex::Male => Sex::Male,
            sea_orm_active_enums::Sex::Female => Sex::Female,
        }
    }
}

impl From<Sex> for sea_orm_active_enums::Sex {
    fn from(value: Sex) -> Self {
        match value {
            Sex::Male => sea_orm_active_enums::Sex::Male,
            Sex::Female => sea_orm_active_enums::Sex::Female,
        }
    }
}

#[post("/personal_info")]
pub async fn create_personal_info(
    state: web::Data<AppState>,
    request: web::Json<PostRequestBody>,
) -> HttpResponse {
    let info = personal_info::ActiveModel {
        sex: Set(request.sex.clone().into()),
        occupation: Set(request.occupation.clone()),
        age: Set(request.age.into()),
        ..Default::default()
    };
    let conn = &state.conn;
    let info: personal_info::Model = match info.insert(conn).await {
        Ok(info) => info,
        Err(e) => {
            error!("Error inserting personal info: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let schedule_active_models: Vec<schedule::ActiveModel> = request
        .schedule
        .iter()
        .enumerate()
        .map(|(i, schedule)| schedule::ActiveModel {
            start_time: Set(schedule.start_time.into()),
            label: Set(schedule.label.clone()),
            personal_info_id: Set(info.id.clone()),
            order: Set(i as i32),
            ..Default::default()
        })
        .collect();
    schedule::Entity::insert_many(schedule_active_models)
        .exec(conn)
        .await
        .unwrap();
    HttpResponse::Created().finish()
}

#[get("/personal_info")]
pub async fn list_personal_info(state: web::Data<AppState>) -> HttpResponse {
    let conn = &state.conn;
    let personal_info_list: Vec<PersonalInfo> = match personal_info::Entity::find()
        .find_with_related(schedule::Entity)
        .all(conn)
        .await
    {
        Ok(list) => list,
        Err(e) => {
            error!("Error fetching personal info: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    .iter()
    .map(|(info, schedule_list)| PersonalInfo {
        id: info.id,
        sex: info.sex.clone().into(),
        occupation: info.occupation.clone(),
        age: info.age,
        schedule: schedule_list
            .iter()
            .map(|schedule| Schedule {
                id: schedule.id,
                start_time: schedule.start_time,
                label: schedule.label.clone(),
            })
            .collect(),
    })
    .collect();
    HttpResponse::Ok().json(Response { personal_info_list })
}
