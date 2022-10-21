use serde_json::{json, Value};
use std::{collections::HashMap, env};
use warp::{reply::Json, Filter};

const WEB_DIR: &str = "web/";
const XAUTH: &str = "X-AUTH-Token";

pub struct ContextUser {
    pub id: i32,
}

#[derive(Debug)]
pub struct AuthError;

impl warp::reject::Reject for AuthError {}

async fn get_items(
    param: String,
    param_map: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("Hello {} {:?}", param, param_map))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("Rust APP LOG", "debug");
    pretty_env_logger::init_custom_env("Rust APP LOG");
    let log = warp::log("basic");
    // let hello = warp::path!("basic" / String)
    //     .map(|name| format!("Hello {}", name))
    //     .with(log);

    // map 同步
    // and_then 异步

    // http://localhost:5980/basic/kei/15
    let info = warp::path!("basic" / String / i32)
        .map(|name, age| format!("Hello name: {}, Age: {}", name, age));

    // http://localhost:5980/add/15/63
    let sum = warp::path!("add" / i32 / i32).map(|a, b| format!("a + b = {}", a + b));

    // path/string?ssa=sasa
    // http://localhost:5980/items/niu?hah=name
    let items = warp::get()
        .and(warp::path("items"))
        .and(warp::path::param::<String>())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path::end())
        .and_then(get_items);

    // let apis = info.or(sum).with(log);
    let apis = warp::get().and(info.or(sum).or(items));

    // http://localhost:5980
    let dir_static = warp::fs::dir(WEB_DIR);
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", WEB_DIR)));
    let static_route = dir_static.or(index);

    let routes = static_route.or(apis);

    // http://localhost:5980/rest/15
    let hi = warp::path("hi").and(warp::get()).map(|| "hi");
    let end = hi.or(rest_api()).or(routes).with(log);

    warp::serve(end).run(([127, 0, 0, 1], 5980)).await;

    Ok(())
}

pub fn rest_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let base_url = warp::path("rest");
    // http://localhost:5980/rest/15
    let get = base_url
        .and(warp::get())
        .and(warp::path::param())
        .and(auth())
        .and_then(rest_get);

    // http://localhost:5980/rest
    let list = base_url
        .and(warp::get())
        .and(warp::path::end())
        // .and(auth().untuple_one())
        .and_then(rest_list);

    // http://localhost:5980/rest
    // {
    //     "id": 25,
    //     "name": "name: 15"
    //   }
    let creat = base_url
        .and(warp::body::json())
        .and(warp::path::end())
        // .and(auth().untuple_one())
        .and_then(rest_creat);
    get.or(list).or(creat)
}

async fn rest_get(id: i32, user: ContextUser) -> Result<Json, warp::Rejection> {
    let something = json!({
        "id": id,
        "name": format!("name: {}", id),
        "user_id": user.id
    });
    let some_warp = warp::reply::json(&something);
    Ok(some_warp)
}

async fn rest_list() -> Result<Json, warp::Rejection> {
    let something = json!([{
        "id": 1,
        "name": "ok"
    }, {
        "id": 2,
        "name": "ok"
    }, {
        "id": 3,
        "name": "ok"
    }]);
    let some_warp = warp::reply::json(&something);
    Ok(some_warp)
}

async fn rest_creat(data: Value) -> Result<Json, warp::Rejection> {
    let some_warp = warp::reply::json(&data);
    Ok(some_warp)
}

// http://localhost:5980/rest/15
// X-AUTH-Token: ok
pub fn auth() -> impl Filter<Extract = (ContextUser,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header::<String>(XAUTH))
        .and_then(|xauth: String| async move {
            if !xauth.starts_with("ok") {
                // panic!("please check the auth!")
                return Err(warp::reject::custom(AuthError));
            }

            // http://localhost:5980/rest/15
            // X-AUTH-Token: ok:15
            if let Some(use_id) = xauth
                .split(":")
                .skip(1)
                .next()
                .and_then(|v| v.parse::<i32>().ok())
            {
                Ok::<ContextUser, warp::Rejection>(ContextUser { id: use_id })
            } else {
                // panic!("please check the use_id!")
                return Err(warp::reject::custom(AuthError));
            }
        })
}

