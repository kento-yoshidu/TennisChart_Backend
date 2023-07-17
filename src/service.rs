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

#[derive(Serialize, FromRow, Debug)]
struct DataA {
    year: u16,
    roger_federer: u8,
    rafael_nadal: u8,
    novak_dokovic: u8,
    andy_murray: u8,
    other: u8,
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

#[get("/gs1")]
pub async fn gs1() -> impl Responder {
    let year2003 = DataA {
        year: 2003,
        roger_federer: 1,
        rafael_nadal: 0,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 3
    };

    let year2004 = DataA {
        year: 2004,
        roger_federer: 3,
        rafael_nadal: 0,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 1
    };

    let year2005 = DataA {
        year: 2005,
        roger_federer: 2,
        rafael_nadal: 1,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 1
    };

    let year2006 = DataA {
        year: 2006,
        roger_federer: 3,
        rafael_nadal: 1,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 0
    };

    let year2007 = DataA {
        year: 2007,
        roger_federer: 3,
        rafael_nadal: 1,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 0
    };

    let year2008 = DataA {
        year: 2008,
        roger_federer: 1,
        rafael_nadal: 2,
        novak_dokovic: 1,
        andy_murray: 0,
        other: 0
    };

    let year2009 = DataA {
        year: 2009,
        roger_federer: 2,
        rafael_nadal: 1,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 1
    };

    let year2010 = DataA {
        year: 2010,
        roger_federer: 1,
        rafael_nadal: 3,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 0
    };

    let year2011 = DataA {
        year: 2011,
        roger_federer: 0,
        rafael_nadal: 1,
        novak_dokovic: 3,
        andy_murray: 0,
        other: 0
    };

    let year2012 = DataA {
        year: 2012,
        roger_federer: 1,
        rafael_nadal: 1,
        novak_dokovic: 1,
        andy_murray: 1,
        other: 0
    };

    let year2013 = DataA {
        year: 2013,
        roger_federer: 0,
        rafael_nadal: 2,
        novak_dokovic: 1,
        andy_murray: 1,
        other: 0
    };

    let year2014 = DataA {
        year: 2014,
        roger_federer: 0,
        rafael_nadal: 1,
        novak_dokovic: 1,
        andy_murray: 1,
        other: 1
    };

    let year2015 = DataA {
        year: 2015,
        roger_federer: 0,
        rafael_nadal: 0,
        novak_dokovic: 3,
        andy_murray: 0,
        other: 1
    };

    let year2016 = DataA {
        year: 2016,
        roger_federer: 0,
        rafael_nadal: 0,
        novak_dokovic: 2,
        andy_murray: 1,
        other: 1
    };

    let year2017 = DataA {
        year: 2017,
        roger_federer: 2,
        rafael_nadal: 2,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 0
    };

    let year2018 = DataA {
        year: 2018,
        roger_federer: 1,
        rafael_nadal: 1,
        novak_dokovic: 2,
        andy_murray: 0,
        other: 0
    };

    let year2019 = DataA {
        year: 2019,
        roger_federer: 0,
        rafael_nadal: 2,
        novak_dokovic: 2,
        andy_murray: 0,
        other: 0
    };

    let year2020 = DataA {
        year: 2020,
        roger_federer: 0,
        rafael_nadal: 1,
        novak_dokovic: 1,
        andy_murray: 0,
        other: 1
    };

    let year2021 = DataA {
        year: 2021,
        roger_federer: 0,
        rafael_nadal: 0,
        novak_dokovic: 3,
        andy_murray: 0,
        other: 1
    };

    let year2022 = DataA {
        year: 2022,
        roger_federer: 0,
        rafael_nadal: 1,
        novak_dokovic: 2,
        andy_murray: 0,
        other: 1
    };

    let year2023 = DataA {
        year: 2023,
        roger_federer: 0,
        rafael_nadal: 0,
        novak_dokovic: 2,
        andy_murray: 0,
        other: 1
    };

    let vec = vec![year2003, year2004, year2005, year2006, year2007, year2008, year2009, year2010, year2011, year2012, year2013, year2014, year2015, year2016, year2017, year2018, year2019, year2020, year2021, year2022, year2023];

    web::Json(vec)
}

#[get("/gs2")]
pub async fn gs2() -> impl Responder {
    let year2003 = DataA {
        year: 2003,
        roger_federer: 1,
        rafael_nadal: 0,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 7
    };

    let year2004 = DataA {
        year: 2004,
        roger_federer: 3,
        rafael_nadal: 0,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 5
    };

    let year2005 = DataA {
        year: 2005,
        roger_federer: 2,
        rafael_nadal: 1,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 5
    };

    let year2006 = DataA {
        year: 2006,
        roger_federer: 4,
        rafael_nadal: 2,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 2
    };

    let year2007 = DataA {
        year: 2007,
        roger_federer: 4,
        rafael_nadal: 2,
        novak_dokovic: 1,
        andy_murray: 0,
        other: 1
    };

    let year2008 = DataA {
        year: 2008,
        roger_federer: 3,
        rafael_nadal: 2,
        novak_dokovic: 1,
        andy_murray: 1,
        other: 1
    };

    let year2009 = DataA {
        year: 2009,
        roger_federer: 4,
        rafael_nadal: 1,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 3
    };

    let year2010 = DataA {
        year: 2010,
        roger_federer: 1,
        rafael_nadal: 3,
        novak_dokovic: 2,
        andy_murray: 1,
        other: 1
    };

    let year2011 = DataA {
        year: 2011,
        roger_federer: 1,
        rafael_nadal: 3,
        novak_dokovic: 3,
        andy_murray: 1,
        other: 0
    };

    let year2012 = DataA {
        year: 2012,
        roger_federer: 1,
        rafael_nadal: 2,
        novak_dokovic: 3,
        andy_murray: 2,
        other: 0
    };

    let year2013 = DataA {
        year: 2013,
        roger_federer: 0,
        rafael_nadal: 2,
        novak_dokovic: 3,
        andy_murray: 2,
        other: 1
    };

    let year2014 = DataA {
        year: 2014,
        roger_federer: 1,
        rafael_nadal: 2,
        novak_dokovic: 2,
        andy_murray: 0,
        other: 3
    };

    let year2015 = DataA {
        year: 2015,
        roger_federer: 2,
        rafael_nadal: 0,
        novak_dokovic: 3,
        andy_murray: 1,
        other: 2
    };

    let year2016 = DataA {
        year: 2016,
        roger_federer: 0,
        rafael_nadal: 0,
        novak_dokovic: 3,
        andy_murray: 3,
        other: 2
    };

    let year2017 = DataA {
        year: 2017,
        roger_federer: 2,
        rafael_nadal: 3,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 3
    };

    let year2018 = DataA {
        year: 2018,
        roger_federer: 1,
        rafael_nadal: 2,
        novak_dokovic: 2,
        andy_murray: 0,
        other: 3
    };

    let year2019 = DataA {
        year: 2019,
        roger_federer: 1,
        rafael_nadal: 3,
        novak_dokovic: 2,
        andy_murray: 0,
        other: 2
    };

    let year2020 = DataA {
        year: 2020,
        roger_federer: 0,
        rafael_nadal: 1,
        novak_dokovic: 2,
        andy_murray: 0,
        other: 3
    };

    let year2021 = DataA {
        year: 2021,
        roger_federer: 0,
        rafael_nadal: 0,
        novak_dokovic: 3,
        andy_murray: 0,
        other: 5
    };

    let year2022 = DataA {
        year: 2022,
        roger_federer: 0,
        rafael_nadal: 2,
        novak_dokovic: 1,
        andy_murray: 0,
        other: 5
    };

    let year2023 = DataA {
        year: 2023,
        roger_federer: 0,
        rafael_nadal: 0,
        novak_dokovic: 3,
        andy_murray: 0,
        other: 3
    };

    let vec = vec![year2003, year2004, year2005, year2006, year2007, year2008, year2009, year2010, year2011, year2012, year2013, year2014, year2015, year2016, year2017, year2018, year2019, year2020, year2021, year2022, year2023];

    web::Json(vec)
}

#[get("/gsms")]
pub async fn gsms() -> impl Responder {
    let year2003 = DataA {
        year: 2003,
        roger_federer: 2,
        rafael_nadal: 0,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 12
    };

    let year2004 = DataA {
        year: 2004,
        roger_federer: 6,
        rafael_nadal: 0,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 8
    };

    let year2005 = DataA {
        year: 2005,
        roger_federer: 6,
        rafael_nadal: 5,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 3
    };

    let year2006 = DataA {
        year: 2006,
        roger_federer: 8,
        rafael_nadal: 3,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 3
    };

    let year2007 = DataA {
        year: 2007,
        roger_federer: 6,
        rafael_nadal: 4,
        novak_dokovic: 2,
        andy_murray: 0,
        other: 2
    };

    let year2008 = DataA {
        year: 2008,
        roger_federer: 1,
        rafael_nadal: 5,
        novak_dokovic: 4,
        andy_murray: 2,
        other: 2
    };

    let year2009 = DataA {
        year: 2009,
        roger_federer: 4,
        rafael_nadal: 4,
        novak_dokovic: 1,
        andy_murray: 2,
        other: 3
    };

    let year2010 = DataA {
        year: 2010,
        roger_federer: 3,
        rafael_nadal: 6,
        novak_dokovic: 0,
        andy_murray: 2,
        other: 3
    };

    let year2011 = DataA {
        year: 2011,
        roger_federer: 1,
        rafael_nadal: 2,
        novak_dokovic: 8,
        andy_murray: 2,
        other: 1
    };

    let year2012 = DataA {
        year: 2012,
        roger_federer: 4,
        rafael_nadal: 3,
        novak_dokovic: 5,
        andy_murray: 1,
        other: 1
    };

    let year2013 = DataA {
        year: 2013,
        roger_federer: 0,
        rafael_nadal: 6,
        novak_dokovic: 5,
        andy_murray: 2,
        other: 1
    };

    let year2014 = DataA {
        year: 2014,
        roger_federer: 2,
        rafael_nadal: 2,
        novak_dokovic: 6,
        andy_murray: 0,
        other: 4
    };

    let year2015 = DataA {
        year: 2015,
        roger_federer: 1,
        rafael_nadal: 0,
        novak_dokovic: 10,
        andy_murray: 2,
        other: 1
    };

    let year2016 = DataA {
        year: 2016,
        roger_federer: 0,
        rafael_nadal: 1,
        novak_dokovic: 6,
        andy_murray: 4,
        other: 3
    };

    let year2017 = DataA {
        year: 2017,
        roger_federer: 5,
        rafael_nadal: 4,
        novak_dokovic: 0,
        andy_murray: 0,
        other: 5
    };

    let year2018 = DataA {
        year: 2018,
        roger_federer: 1,
        rafael_nadal: 4,
        novak_dokovic: 4,
        andy_murray: 2,
        other: 5
    };

    let year2019 = DataA {
        year: 2019,
        roger_federer: 1,
        rafael_nadal: 4,
        novak_dokovic: 4,
        andy_murray: 0,
        other: 5
    };

    let year2020 = DataA {
        year: 2020,
        roger_federer: 0,
        rafael_nadal: 1,
        novak_dokovic: 3,
        andy_murray: 0,
        other: 9
    };

    let year2021 = DataA {
        year: 2021,
        roger_federer: 0,
        rafael_nadal: 1,
        novak_dokovic: 4,
        andy_murray: 0,
        other: 9
    };

    let year2022 = DataA {
        year: 2022,
        roger_federer: 0,
        rafael_nadal: 2,
        novak_dokovic: 3,
        andy_murray: 0,
        other: 9
    };

    let vec = vec![year2003, year2004, year2005, year2006, year2007, year2008, year2009, year2010, year2011, year2012, year2013, year2014, year2015, year2016, year2017, year2018, year2019, year2020, year2021, year2022];

    web::Json(vec)
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
