use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct Chores {
    common: Vec<String>,
    uncommon: Vec<String>,
    rare: Vec<String>,
    spicy: Vec<String>,
}

async fn listen(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = &*req.uri().to_string();
    let json: Vec<u8> = match path {
        "/read_all" => {
            let dir: String = exe_dir();
            let common = read_file(dir.clone() + "/common").await;
            let uncommon = read_file(dir.clone() + "/uncommon").await;
            let rare = read_file(dir.clone() + "/rare").await;
            let spicy = read_file(dir.clone() + "/spicy").await;

            let chores = Chores {
                common: common.split("\n").map(|s| s.to_string()).collect(),
                uncommon: uncommon.split("\n").map(|s| s.to_string()).collect(),
                rare: rare.split("\n").map(|s| s.to_string()).collect(),
                spicy: spicy.split("\n").map(|s| s.to_string()).collect(),
            };
            let json = serde_json::to_vec(&chores).unwrap();

            // serde_json
            //     serde_json::to_string(commons).unwrap()
            //                 let out = r#"{"statusCode":200}"#;
            //                 // let out = read_file().await;
            //                 writer
            //                     .write_all(serde_json::to_string(&out).unwrap().as_bytes())
            //                     .await
            //                     .unwrap();

            json
        }
        "/write" => "ok".into(),
        _ => "nothing to do".into(),
    };
    let mut res = Response::builder()
        .status(200)
        .header("Access-Control-Allow-Origin", "*")
        .body(json.into())
        .unwrap();
    // res.headers_mut().insert(
    //     "Access-Control-Allow-Origin",
    //     HeaderValue::from_str("*").unwrap(),
    // );
    // let (parts, _body) = res.into_parts();
    // Ok(Response::from_parts(parts, json.into()))
    Ok(res)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(listen)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn exe_dir() -> String {
    let mut dir = env::current_exe().unwrap();
    dir.pop();
    dir.pop();
    dir.pop();
    dir.pop();
    dir.into_os_string().into_string().unwrap()
}

async fn write_file() {
    let f = tokio::fs::write("common", "ok").await.unwrap();
    f
}

async fn read_file(fname: String) -> String {
    let f = tokio::fs::read_to_string(fname).await.unwrap();
    f
}
