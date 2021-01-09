use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use hyper::{Body, Request, Response};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn hello_rust(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Micro Service".into()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 8080).into();

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_rust)) });

    let server = Server::bind(&addr).serve(make_svc);

    //server.await?;
    if let Err(e) = server.await {
        eprintln!("Error {:?}", e);
    }
    Ok(())
}
