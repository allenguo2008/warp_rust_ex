use serde_json::{json, Value};
use warp::{reply::Json, Filter};

pub const WEB_DIR: &str = "web/";
pub const XAUTH: &str = "X-AUTH-Token";

pub struct ContextUser {
    pub id: i32,
}

#[derive(Debug)]
pub struct AuthError;

impl warp::reject::Reject for AuthError {}

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

