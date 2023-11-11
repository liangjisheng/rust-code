// use super::handlers::*;
use super::handlers_db::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(new_course))
            .route("/{user_id}", web::get().to(get_courses_for_teacher))
            .route("/{user_id}/{course_id}", web::get().to(get_course_detail)),
    );
}

// curl "http://127.0.0.1:8080/health"
// curl -X POST "http://127.0.0.1:8080/courses/" -d '{ "teacher_id":1, "id":4, "name":"class" }' -H "Content-Type: application/json"
// curl "http://127.0.0.1:8080/courses/1"
// curl "http://127.0.0.1:8080/courses/1/1"
