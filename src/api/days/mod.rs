use actix_web::{get, post, web, HttpResponse};
use entity::{activity, day};
use log::error;
use req::PostRequestBody;
use res::{Activity, Day, Response};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, ModelTrait, QueryOrder};
use serde::Deserialize;

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
            hours: Set(activity.hours),
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
        .order_by_desc(day::Column::CreatedAt)
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

#[derive(Deserialize)]
struct PathParams {
    id: i32,
}

#[get("/days/{id}")]
pub async fn get_day(path: web::Path<PathParams>, state: web::Data<AppState>) -> HttpResponse {
    let conn = &state.conn;
    let day_model: Option<day::Model> = match day::Entity::find_by_id(path.id).one(conn).await {
        Ok(day) => day,
        Err(e) => {
            error!("Error fetching day: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    let day_model: day::Model = match day_model {
        Some(day) => day,
        None => {
            return HttpResponse::NotFound().finish();
        }
    };
    let activities: Vec<activity::Model> =
        match day_model.find_related(activity::Entity).all(conn).await {
            Ok(activites) => activites,
            Err(e) => {
                error!("Error fetching activites: {:?}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };
    let day = Day {
        id: day_model.id,
        title: day_model.title.clone(),
        description: day_model.description.clone(),
        country: day_model.country.clone(),
        created_at: day_model.created_at.to_string(),
        occupation: day_model.occupation.clone(),
        activities: activities
            .iter()
            .map(|schedule| Activity {
                id: schedule.id,
                hours: schedule.hours,
                name: schedule.name.clone(),
                color: schedule.color.clone(),
            })
            .collect(),
    };
    HttpResponse::Ok().json(day)
}
