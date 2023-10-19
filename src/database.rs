///this place is where the query 
use sqlx::{Pool, Sqlite};

pub mod requests;
pub mod responses;



use responses::Task;
use responses::User;

pub type DBResult<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

pub async fn create_task(
    pool: &Pool<Sqlite>,
    name: &String,
    description: &String,
    rewardsat: i64,
    link: &String,
    creator_name: &String
) -> DBResult<i64> {
    let mut connection = pool
        .acquire()
        .await?;
    let id = sqlx::query_as!(
            Task,
            r#"
        INSERT INTO tasks (name, description, rewardsat, link, creator_name) VALUES (?, ?,?,?,?);
        "#,
            name,
            description,
            rewardsat,
            link,
            creator_name
    )
        .execute(&mut connection)
        .await?
        .last_insert_rowid();
        Ok(id)
}

pub async fn get_task(pool: &Pool<Sqlite>, id: i64) -> DBResult<Task> {
    let mut connection = pool.acquire()
        .await?;
    let task = sqlx::query_as!(
        Task,
        r#"
        SELECT id, name, description, rewardsat, link, creator_name from tasks
        WHERE id = ?;
        "#,
            id
    )
        .fetch_one(&mut connection)
        .await?;
        Ok(task)
}


pub async fn get_task_by_name(pool: &Pool<Sqlite>, name: String) -> DBResult<Task> {
    let mut connection = pool.acquire()
        .await?;
    let task = sqlx::query_as!(
        Task,
        r#"
        SELECT id, name, description, rewardsat, link, creator_name from tasks
        WHERE name = ?;
        "#,
            name
    )
        .fetch_one(&mut connection)
        .await?;
        Ok(task)

}       
pub async fn get_tasks(pool: &Pool<Sqlite>) -> DBResult<Vec<Task>> {
    let mut connection = pool.acquire()
        .await
        .unwrap();
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        select id, name, description, rewardsat, link, creator_name from tasks;
        "#
    )
        .fetch_all(&mut connection)
        .await?;
        Ok(tasks)

}
// create a new user into the database
pub async fn create_user(
    pool: &Pool<Sqlite>,
    name: &String,
    address: &String,
    balance: i64,
    badge: String,
) -> DBResult<i64> {
    let mut connection = pool
        .acquire()
        .await?;
    let id = sqlx::query_as!(
            User,
            r#"
        INSERT INTO users (name, address, balance, badge) VALUES (?, ?, ?, ?);
        "#,

            name,
            address,
            balance,
            badge
    )
        .execute(&mut connection)
        .await?
        .last_insert_rowid();
        Ok(id)
}
// }

pub async fn get_user(pool: &Pool<Sqlite>, id: i64) -> DBResult<User> {
    let mut connection = pool.acquire()
        .await?;
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, address, balance, badge from users
        WHERE id = ?;
        "#,
            id
    )
        .fetch_one(&mut connection)
        .await?;
        Ok(user)
}

pub async fn get_user_by_name(pool: &Pool<Sqlite>, name: String) -> DBResult<User> {
    let mut connection = pool.acquire()
        .await?;
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, name, address, balance, badge from users
        WHERE name = ?;
        "#,
            name
    )
        .fetch_one(&mut connection)
        .await?;
        Ok(user)
}