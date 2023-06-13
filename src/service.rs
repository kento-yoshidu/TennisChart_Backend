use actix_web::{
    get, post, web,
    web::{Data, Json, Path},
    Responder, HttpResponse
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use crate::AppState;

#[derive(Serialize, FromRow, Debug)]
struct User {
    id: i32,
    name: String,
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => {
            println!("{:?}", users);
            HttpResponse::Ok().json(users)
        },
        Err(_) => HttpResponse::NotFound().json("No users found"),
    }
}

#[get("/test")]
pub async fn test() -> impl Responder {
    let obj = User {
        id: 1,
        name: String::from("Roger Federer")
    };

    let obj2 = User {
        id: 2,
        name: String::from("Rafael Nadal")
    };

    let obj3 = User {
        id: 3,
        name: String::from("Novak Dokovic")
    };

    let obj4 = User {
        id: 4,
        name: String::from("Andy Murray")
    };

    let vec = vec![obj, obj2, obj3, obj4];

    return web::Json(vec);
}


/*
#[get("/test")]
pub async fn fetch_test(state: Data<AppState>) -> Result<impl Responder> {
    let user = User { id: 1, name: String::from("Roger") };
    // let json = serde_json::to_string(&user).unwrap();

    let user = User {
        id: 1,
        name: String::from("Roger")
    };

    Ok(web::Json(user))
}
*/
