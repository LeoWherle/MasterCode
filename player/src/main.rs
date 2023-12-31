use actix_web::cookie::Cookie;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use common::{ColorSequence, MAX_GUESSES, ALL_FIELDS};
use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;
use tracing::{debug, info};
use uuid::Uuid;

type AppState = RwLock<HashMap<String, (ColorSequence, u32)>>;

async fn play_game(
    req: HttpRequest,
    guess: web::Json<ColorSequence>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut map = data.write().unwrap();

    let player_id = req
        .cookie("player_id")
        .map(|c| c.value().to_string())
        .unwrap_or_default();
    let (sequence, tries) = map.entry(player_id.clone()).or_insert_with(|| (ColorSequence::random(), 0));
    let response = guess.check_guess(sequence);

    info!("Guess: {}", guess);

    if response.correct_positions == ALL_FIELDS {
        *tries = 0;
        return HttpResponse::Ok().json("Congratulations! You won the game!");
    } else {
        *tries += 1;
        if *tries >= MAX_GUESSES as u32 {
            map.remove(&player_id);
        }
    }

    HttpResponse::Ok().json(response)
}

async fn index(req: HttpRequest) -> HttpResponse {
    let player_id = req
        .cookie("player_id")
        .map(|c| c.value().to_string())
        .unwrap_or_else(|| {
            let new_id = Uuid::new_v4().to_string();
            debug!("New player id: {}", new_id);
            new_id
        });

    HttpResponse::Ok()
        .content_type("text/html")
        .cookie(Cookie::new("player_id", player_id))
        .body(fs::read_to_string("player/web/index.html").unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let shared_data = web::Data::new(RwLock::new(HashMap::<String, (ColorSequence, u32)>::new()));

    // print computer local IP
    let local_ip = get_if_addrs::get_if_addrs()
        .unwrap()
        .iter()
        .find(|iface| iface.name == "eth0")
        .unwrap()
        .ip();

    info!("Computer local IP: {}", local_ip);

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .route("/", web::get().to(index))
            .route("/guess", web::post().to(play_game))
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await
}
