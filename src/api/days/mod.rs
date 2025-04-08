use actix_web::{get, post, web, HttpResponse};
use entity::{activity, day};
use log::error;
use req::PostRequestBody;
use res::{Activity, Day, Response};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};

use crate::AppState;

mod req;
mod res;

#[post("/days")]
pub async fn create_day(
    state: web::Data<AppState>,
    request: web::Json<PostRequestBody>,
) -> HttpResponse {
    // TODO: Validate request
    let day = day::ActiveModel {
        occupation: Set(request.occupation.clone()),
        title: Set(request.title.clone()),
        description: Set(request.description.clone()),
        country: Set(request.country.clone()),
        ..Default::default()
    };
    let conn = &state.conn;
    let info: day::Model = match day.insert(conn).await {
        Ok(info) => info,
        Err(e) => {
            error!("Error inserting personal info: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let activity_active_models: Vec<activity::ActiveModel> = request
        .activities
        .iter()
        .map(|activity| activity::ActiveModel {
            hours: Set(activity.hours as i32),
            name: Set(activity.name.clone()),
            color: Set(activity.color.clone()),
            day_id: Set(info.id),
            ..Default::default()
        })
        .collect();
    activity::Entity::insert_many(activity_active_models)
        .exec(conn)
        .await
        .unwrap();
    HttpResponse::Created().finish()
}

#[get("/days")]
pub async fn list_days(state: web::Data<AppState>) -> HttpResponse {
    let conn = &state.conn;
    let days: Vec<Day> = match day::Entity::find()
        .find_with_related(activity::Entity)
        .all(conn)
        .await
    {
        Ok(list) => list,
        Err(e) => {
            error!("Error fetching personal info: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    .iter_mut()
    .map(|(day, activities)| Day {
        id: day.id,
        title: day.title.clone(),
        description: day.description.clone(),
        country: day.country.clone(),
        created_at: day.created_at.to_string(),
        occupation: day.occupation.clone(),
        activities: activities
            .iter()
            .map(|schedule| Activity {
                id: schedule.id,
                hours: schedule.hours,
                name: schedule.name.clone(),
                color: schedule.color.clone(),
            })
            .collect(),
    })
    .collect();
    HttpResponse::Ok().json(Response { days })
}
