use std::net::SocketAddr;

use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder as ConnBuilder;
use s3s::service::S3Service;
use tokio::net::TcpListener;
use tracing::{debug, error, info, instrument};

use crate::Result;

#[tokio::main]
#[instrument(skip(service))]
pub async fn serve(service: S3Service, address: SocketAddr) -> Result {
    // Run server
    info!("Starting S3 server");
    let listener = TcpListener::bind(address).await.unwrap();
    let local_addr = listener.local_addr().unwrap();

    let http_server = ConnBuilder::new(TokioExecutor::new());
    let graceful = hyper_util::server::graceful::GracefulShutdown::new();

    let mut ctrl_c = std::pin::pin!(tokio::signal::ctrl_c());

    info!("Server is running at http://{local_addr}");

    loop {
        let (socket, _) = tokio::select! {
            res = listener.accept() => {
                match res {
                    Ok(conn) => conn,
                    Err(err) => {
                        error!("error accepting connection: {err}");
                        continue;
                    }
                }
            }
            _ = ctrl_c.as_mut() => {
                debug!("Ctrl-C received, shutting down...");
                break;
            }
        };

        let conn = graceful.watch(
            http_server
                .serve_connection(TokioIo::new(socket), service.clone())
                .into_owned(),
        );
        tokio::spawn(async move {
            let _ = conn.await;
        });
    }

    tokio::select! {
        () = graceful.shutdown() => {
             debug!("Gracefully shutdown!");
        },
        () = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
             debug!("Waited 10 seconds for graceful shutdown, aborting...");
        }
    }

    info!("Server is stopped");
    Ok(())
}
