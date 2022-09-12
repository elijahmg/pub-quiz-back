use crate::error_handler::CustomError;
use crate::teams::{NewTeam, Team};
use actix_web::{get, post, web, HttpResponse};

#[get("/team/{id}")]
async fn find(path: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let id = path.into_inner().parse::<i32>().unwrap();

    let team = Team::find(id)?;

    Ok(HttpResponse::Ok().json(team))
}

#[post("/team")]
async fn create(team: web::Json<NewTeam>) -> Result<HttpResponse, CustomError> {
    let team = Team::create(team.into_inner())?;
    Ok(HttpResponse::Ok().json(team))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(create);
}
