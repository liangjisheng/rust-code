use crate::error::MyError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use sqlx::mysql::MySqlPool;

// 之后我们开始编写 db_access 相关的模块，因为实现了 From trait 和 TryFrom trait
// 所以我们可以直接把查询出来的数据转化成刚刚定义的数据结构。
// 在查询的时候使用了 query_as 这个宏，然后指定一个数据结构，当我们查询出内容时，会包装成指定的数据结构

pub async fn get_courses_for_teacher_db(
    pool: &MySqlPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        "SELECT * FROM course
        WHERE teacher_id = ?",
        teacher_id
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_course_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        "SELECT * FROM course
            WHERE teacher_id = ? and id = ?",
        teacher_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course didn't founded".into()))
    }
}

pub async fn post_new_course_db(
    pool: &MySqlPool,
    new_course: CreateCourse,
) -> Result<Course, MyError> {
    let data = sqlx::query_as!(
        Course,
        "INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        new_course.teacher_id, new_course.name, new_course.description,
        new_course.format, new_course.structure, new_course.duration,
        new_course.price, new_course.language, new_course.level
        )
        .execute(pool)
        .await?;

    let row = sqlx::query_as!(
        Course,
        "SELECT * FROM course
                WHERE id = ?",
        data.last_insert_id(),
    )
    .fetch_optional(pool)
    .await?;

    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course didn't founded".into()))
    }
}

pub async fn delete_course_db(
    pool: &MySqlPool,
    teacher_id: i32,
    id: i32,
) -> Result<String, MyError> {
    let course_row = sqlx::query!(
        "DELETE FROM course where teacher_id = ? and id=?",
        teacher_id,
        id,
    )
    .execute(pool)
    .await?;

    Ok(format!("Deleted {:?} record", course_row))
}

pub async fn update_course_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse,
) -> Result<Course, MyError> {
    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM course where teacher_id=? and id=?",
        teacher_id,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Course Id not found".into()))?;

    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name
    };
    let description: String = if let Some(description) = update_course.description {
        description
    } else {
        current_course_row.description.unwrap_or_default()
    };
    let format: String = if let Some(format) = update_course.format {
        format
    } else {
        current_course_row.format.unwrap_or_default()
    };
    let structure: String = if let Some(structure) = update_course.structure {
        structure
    } else {
        current_course_row.structure.unwrap_or_default()
    };
    let duration: String = if let Some(duration) = update_course.duration {
        duration
    } else {
        current_course_row.duration.unwrap_or_default()
    };
    let level: String = if let Some(level) = update_course.level {
        level
    } else {
        current_course_row.level.unwrap_or_default()
    };
    let language: String = if let Some(language) = update_course.language {
        language
    } else {
        current_course_row.language.unwrap_or_default()
    };
    let price: i32 = if let Some(price) = update_course.price {
        price
    } else {
        current_course_row.price.unwrap_or_default()
    };

    let course_row = sqlx::query_as!(
        Course,
        "UPDATE course SET name = ?, description = ?, format = ?,
            structure = ?, duration = ?, price = ?, language = ?,
            level = ? where teacher_id = ? and id = ?",
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        teacher_id,
        id
    )
    .execute(pool)
    .await;

    if let Ok(course) = course_row {
        let row = sqlx::query_as!(
            Course,
            "SELECT * FROM course
                WHERE id = ?",
            course.last_insert_id(),
        )
        .fetch_optional(pool)
        .await?;

        if let Some(course) = row {
            Ok(course)
        } else {
            Err(MyError::NotFound("Course didn't founded".into()))
        }
    } else {
        Err(MyError::NotFound("Course id not found".into()))
    }
}
