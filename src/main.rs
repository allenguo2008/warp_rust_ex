use std::{collections::HashMap, env};
use warp::Filter;
mod rest_if;
use crate::rest_if::rest_api;
use crate::rest_if::WEB_DIR;

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

