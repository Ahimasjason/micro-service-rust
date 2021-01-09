use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

const INDEX: &'static str = r#"
<!doctype html>
<html>
   <head>
       <title> Dummy Title </title>
   </head>
   <body>
     <h3> Hello Champ </h3>
  </body>
</html>        
"#;

async fn service_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(INDEX.into())),

        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            Ok(response)
        }
    }
    //Ok(Response::new("Micro Service".into()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 8080).into();

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(service_handler)) });

    let server = Server::bind(&addr).serve(make_svc);

    //server.await?;
    if let Err(e) = server.await {
        eprintln!("Error {:?}", e);
    }
    Ok(())
}
