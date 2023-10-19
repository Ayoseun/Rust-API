#[macro_use]

extern crate rocket;
mod database;
use database::get_task_by_name;
use database::get_user_by_name;
use database::requests::TaskRequest;
use database::requests::UserRequest;
use database::responses::Task;
use database::{create_task, get_task, create_user, get_user,get_tasks,DBResult };
use database::responses::User;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{Pool, Sqlite, SqlitePool};
use rocket::Request;
use rocket::response::status;
use rocket::http::Status;

#[post("/tasks", format = "json", data = "<task>")]
async fn create(task: Json<TaskRequest>, pool: &State<Pool<Sqlite>>) -> DBResult<Json<Task>> {
    let id = create_task(pool, &task.name, &task.description, task.rewardsat, &task.link, &task.creator_name).await?;
    let task = get_task(pool, id).await?;
    Ok(Json(task))
}

#[get("/tasks")]
async fn index(pool: &State<Pool<Sqlite>>) -> DBResult<Json<Vec<Task>>> {
    let tasks = get_tasks(pool).await?;
    Ok(Json(tasks))
}

#[get("/tasks/<id>")]
async fn detail(id: i64, pool: &State<Pool<Sqlite>>) -> DBResult<Json<Task>> {
    let task = get_task(pool, id).await?;
    Ok(Json(task))

}

#[get("/tasks/<name>", rank =2)]
async fn detail_name(name: String, pool: &State<Pool<Sqlite>>) -> DBResult<Json<Task>> {
    let task = get_task_by_name(pool, name).await?;
    Ok(Json(task))

}

#[post("/users", format = "json", data = "<user>")]
async fn create_(user:  Json<UserRequest>, pool: &State<Pool<Sqlite>>) -> DBResult<Json<User>> {
    let id = create_user(pool, &user.name, &user.address, user.balance, user.badge.to_owned()).await?;
    let task = get_user(pool, id).await?;
    Ok(Json(task))
}
#[get("/users/<id>")]
async fn fetch_user_by_id(pool: &State<Pool<Sqlite>>, id: i64) -> DBResult<Json<User>> {
    let tasks = get_user(pool, id).await?;
    Ok(Json(tasks))
}

#[get("/users/<name>", rank = 3)]
async fn fetch_user_by_name(pool: &State<Pool<Sqlite>>, name: String) -> DBResult<Json<User>> {
    let tasks = get_user_by_name(pool, name).await?;
    Ok(Json(tasks))
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let pool = SqlitePool::connect("sqlite://data.db")
        .await
        .expect("Couldn't connect to sqlite database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Couldn't migrate the database tables");

    let _rocket = rocket::build()
        .mount("/", routes![index, create, detail, create_, detail_name, fetch_user_by_id,fetch_user_by_name ])
        .register("/", catchers![ default_catcher])
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}