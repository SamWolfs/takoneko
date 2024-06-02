mod settings;

use axum::{extract::Json, http::StatusCode, routing::post, Router};
use octocrab::{models::events::payload::EventPayload, Octocrab};
use serde::Serialize;
use settings::Settings;
use std::fs;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let Ok(settings) = Settings::new() else {
        panic!("Unable to parse settings")
    };

    let app_private_key = fs::read_to_string(settings.github.private_key_path)
        .expect("Should have been able to read the file");

    let key = jsonwebtoken::EncodingKey::from_rsa_pem(app_private_key.as_bytes()).unwrap();

    let octocrab = Octocrab::builder()
        .app(settings.github.app_id.into(), key)
        .build()?;
    let _installations = octocrab.apps().installations().send().await.unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();

    Ok(())
}

fn app() -> Router {
    Router::new().route("/", post(webhook_handler))
}

async fn webhook_handler(payload: Json<EventPayload>) -> (StatusCode, Json<Empty>) {
    match payload.0 {
        EventPayload::InstallationEvent(event) => println!("Received: {:?}", event),
        _ => println!("Oops"),
    }

    (StatusCode::OK, Json(Empty {}))
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct Empty {}
