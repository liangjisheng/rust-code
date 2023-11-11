use crate::handlers::{course::*, general::health_check_handler, teacher::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(post_new_course))
            .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
            .route(
                "/{teacher_id}/{course_id}",
                web::get().to(get_course_detail),
            )
            .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
            .route(
                "/{teacher_id}/{course_id}",
                web::put().to(update_course_details),
            ),
    );
}

pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/teachers")
            .route("/", web::post().to(post_new_teacher))
            .route("/", web::get().to(get_all_teacher))
            .route("/{teacher_id}", web::get().to(get_teacher_detail))
            .route("/{teacher_id}", web::delete().to(delete_teacher))
            .route("/{teacher_id}", web::put().to(update_teacher_details)),
    );
}

// curl "http://127.0.0.1:8080/health"
// curl -X POST "http://127.0.0.1:8080/courses/" -d '{ "teacher_id":1, "id":4, "name":"class" }' -H "Content-Type: application/json"
// curl "http://127.0.0.1:8080/courses/1"
// curl "http://127.0.0.1:8080/courses/1/1"

// curl "http://127.0.0.1:8080/teachers/"
// curl "http://127.0.0.1:8080/teachers/1"
