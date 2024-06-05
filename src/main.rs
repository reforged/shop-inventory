use std::{env, sync::Arc};

use actix_web::{App, HttpServer, web::Data};
use dotenv::dotenv;
use lapin::{Connection, ConnectionProperties};
use tracing::info;


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let conn = Connection::connect("amqp://rabbit:rabbit@localhost/%2f", ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;
        
    let addr = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("3333".to_string());
    let addr_in = format!("{}:{}", addr, port);

    info!("Starting server at http://{}", addr_in);

    let channel = Arc::new(channel.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(channel.clone()))
    })
        .bind(addr_in)?
        .run()
        .await?;

    Ok(())
}
