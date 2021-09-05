use hyper::header::{AsHeaderName, HeaderValue};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[derive(Serialize, Deserialize)]
struct Package {
    path: String,
}

#[derive(Serialize, Deserialize)]
struct Chores {
    commons: Vec<String>,
    uncommons: Vec<String>,
    rare: Vec<String>,
    spicy: Vec<String>,
}

async fn listen(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = &*req.uri().to_string();
    let res: String = match path {
        "/read_all" => {
            let mut dir: String = exe_dir();
            let commons = read_file(dir.clone() + "/common").await;
            let uncommons = read_file(dir.clone() + "/uncommon").await;
            let rare = read_file(dir.clone() + "/rare").await;
            let spicy = read_file(dir.clone() + "/spicy").await;

            let chores = Chores {
                commons: commons.split("\n").map(|s| s.to_string()).collect(),
                uncommons: uncommons.split("\n").map(|s| s.to_string()).collect(),
                rare: rare.split("\n").map(|s| s.to_string()).collect(),
                spicy: spicy.split("\n").map(|s| s.to_string()).collect(),
            };
            let res = serde_json::to_string(&chores).unwrap();

            // serde_json
            //     serde_json::to_string(commons).unwrap()
            //                 let out = r#"{"statusCode":200}"#;
            //                 // let out = read_file().await;
            //                 writer
            //                     .write_all(serde_json::to_string(&out).unwrap().as_bytes())
            //                     .await
            //                     .unwrap();

            res
        }
        "/write" => "ok".into(),
        _ => "nothing to do".into(),
    };
    Ok(Response::new(res.into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

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

#[ignore]
async fn write_file() {
    let f = tokio::fs::write("common", "ok").await.unwrap();
    println!("done writing");
    f
}

async fn read_file(fname: String) -> String {
    let f = tokio::fs::read_to_string(fname).await.unwrap();
    println!("{}", f);
    f
}
