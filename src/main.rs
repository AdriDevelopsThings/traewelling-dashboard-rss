use std::{env, net::SocketAddr, str::FromStr};

use axum::{Router, Server, extract, response::{IntoResponse, Redirect, Response}, routing, http::HeaderValue};
use errors::Error;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};
use rand::{Rng, distributions::Alphanumeric};
use serde::Deserialize;
use traewelling::{Traewelling, BASE_URL};
use dotenv::dotenv;

mod traewelling;
mod rss;
mod errors;

#[derive(Clone)]
struct State {
    connection: Pool<SqliteConnectionManager>,
    traewelling: Traewelling,
    public_url: String
}

impl State {
    fn new(path: &str, traewelling: Traewelling, public_url: String) -> Result<Self, r2d2::Error> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager)?;
        Ok(Self { connection: pool, traewelling, public_url })
    }

    fn get_connection(&self) -> Result<PooledConnection<SqliteConnectionManager>, r2d2::Error> {
        self.connection.get()
    }
}

pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

async fn create_rss_feed(
    extract::State(state): extract::State<State>
) -> impl IntoResponse {
    Redirect::temporary(format!("{}/oauth/authorize?response_type=code&client_id={}&redirect_uri={}/callback", BASE_URL, state.traewelling.client_id, state.public_url).as_str())
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: String
}

async fn callback(
    extract::Query(query): extract::Query<CallbackQuery>,
    extract::State(state): extract::State<State>
) -> Result<Response<String>, Error> {
    let token = state.traewelling.token(&query.code, format!("{}/callback", state.public_url).as_str()).await?;
    let id = generate_random_string(64);
    {
        let connection = state.get_connection()?;
        connection.execute("INSERT INTO tokens (id, token) VALUES (?, ?)", params![id, token.access_token])?;
    }
    let url = format!("{}/rss/{}", state.public_url, id);
    let mut response = Response::new(format!(r#"<html><body>Success!<br>Your rss feed: <a href="{}">{}</a></body></html>"#, url, url));
    response.headers_mut().insert("Content-Type", HeaderValue::from_static("text/html"));
    Ok(response)
}

#[derive(Deserialize)]
struct RssQuery {
    pub timezone: Option<String>,
    #[serde(default)]
    pub ignore_users: String
}

async fn rss(
    extract::Path(id): extract::Path<String>,
    extract::Query(query): extract::Query<RssQuery>,
    extract::State(state): extract::State<State>
) -> Result<Response<String>, Error> {
    let connection = state.get_connection()?;
    let token: String = connection.query_row("SELECT token FROM tokens WHERE id = ?", params![id], |row| row.get(0))?;
    let dashboard = state.traewelling.dashboard(&token).await?;
    let ignore_users = query.ignore_users.split(',').collect::<Vec<&str>>();
    let rss = dashboard.to_channel(query.timezone.unwrap_or_else(|| "Europe/Berlin".to_string()), ignore_users);
    let mut response = Response::new(rss.to_string());
    response.headers_mut().insert("Content-Type", HeaderValue::from_static("application/xml"));
    Ok(response)
}

async fn delete_rss(
    extract::Path(id): extract::Path<String>,
    extract::State(state): extract::State<State>
) -> Result<String, Error> {
    let connection = state.get_connection()?;
    connection.execute("DELETE FROM tokens WHERE id = ?", params![id])?;
    Ok(String::from("Success"))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let state = State::new(
        env::var("DATABASE_PATH").expect("No DATABASE_PATH set").as_ref(),
        Traewelling::new(env::var("TRAEWELLING_CLIENT_ID").expect("No TRAEWELLING_CLIENT_ID set"), env::var("TRAEWELLING_CLIENT_SECRET").expect("No TRAEWELLING_CLIENT_SECRET set")),
        env::var("PUBLIC_URL").expect("No PUBLIC_URL set")
    ).expect("Error while connecting to database");
    {
        let connection = state.get_connection().unwrap();
        connection.execute("CREATE TABLE IF NOT EXISTS tokens (id STRING, token STRING)", params![]).unwrap();
    }
    let router = Router::new()
        .route("/", routing::get(create_rss_feed))
        .route("/callback", routing::get(callback))
        .route("/rss/:id", routing::get(rss).delete(delete_rss))
        .with_state(state);
    
    let listen_address = env::var("LISTEN_ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1:8000"));
    let addr = SocketAddr::from_str(&listen_address).expect("Invalid listen address");
    println!("Server listing on {listen_address}");
    Server::bind(&addr).serve(router.into_make_service()).await.expect("Error while listening");
}
