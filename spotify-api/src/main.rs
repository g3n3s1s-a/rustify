use axum::{Router, routing::get, response::Json, Server, extract::Query};
use http::Method;
use tower_http::cors::{Any, CorsLayer};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;
use csv;
use reqwest;
use anyhow::Result;

// ====== DATA STRUCTURE ======
#[derive(Debug, Serialize, Deserialize, Clone)]
struct SpotifySong {
    track_id: String,
    track_name: String,
    track_artist: String,
    track_popularity: Option<u32>,
    playlist_genre: String,
    playlist_subgenre: String,
    danceability: Option<f32>,
    energy: Option<f32>,
    tempo: Option<f32>,
}

// Frontend expects this structure
#[derive(Debug, Serialize, Clone)]
struct Track {
    id: String,
    title: String,
    artist: String,
    primary_genre: String,
    year: Option<i32>,
}

// Query parameters from frontend
#[derive(Debug, Deserialize)]
struct RecommendationQuery {
    artist: Option<String>,
    genre: Option<String>,
    limit: Option<usize>,
}

// ====== GLOBAL STORAGE ======
static SONGS: Lazy<Arc<RwLock<Vec<SpotifySong>>>> = Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

// ====== ROUTES ======
async fn hello() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "message": "Hello from Rust backend!" }))
}

async fn get_songs() -> Json<Vec<SpotifySong>> {
    let songs = SONGS.read().await;
    let sample: Vec<SpotifySong> = songs.iter().take(5).cloned().collect();
    Json(sample)
}

async fn get_recommendations(Query(params): Query<RecommendationQuery>) -> Json<Vec<Track>> {
    let songs = SONGS.read().await;
    let limit = params.limit.unwrap_or(20);
    
    let artist_query = params.artist.unwrap_or_default().to_lowercase();
    let genre_query = params.genre.unwrap_or_default().to_lowercase();
    
    // Filter and score songs based on artist/genre match
    let mut matched: Vec<(SpotifySong, i32)> = songs
        .iter()
        .filter_map(|song| {
            let mut score = 0;
            
            // Check artist match
            if !artist_query.is_empty() && song.track_artist.to_lowercase().contains(&artist_query) {
                score += 10;
            }
            
            // Check genre match
            if !genre_query.is_empty() {
                if song.playlist_genre.to_lowercase().contains(&genre_query) {
                    score += 8;
                }
                if song.playlist_subgenre.to_lowercase().contains(&genre_query) {
                    score += 5;
                }
            }
            
            if score > 0 {
                Some((song.clone(), score))
            } else {
                None
            }
        })
        .collect();
    
    // Sort by score (highest first)
    matched.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Convert to Track format and limit results
    let results: Vec<Track> = matched
        .into_iter()
        .take(limit)
        .map(|(song, _)| Track {
            id: song.track_id.clone(),
            title: song.track_name.clone(),
            artist: song.track_artist.clone(),
            primary_genre: song.playlist_genre.clone(),
            year: None,
        })
        .collect();
    
    Json(results)
}

// ====== DATA LOADER ======
async fn load_spotify_songs() -> Result<()> {
    let url = "https://raw.githubusercontent.com/rfordatascience/tidytuesday/master/data/2020/2020-01-21/spotify_songs.csv";
    println!("Downloading Spotify dataset...");
    let resp = reqwest::get(url).await?.text().await?;
    let mut reader = csv::Reader::from_reader(resp.as_bytes());

    let mut data = Vec::new();
    for result in reader.deserialize() {
        let record: SpotifySong = result?;
        data.push(record);
    }

    let mut songs = SONGS.write().await;
    *songs = data;
    println!("✅ Loaded {} songs into memory!", songs.len());
    Ok(())
}

// ====== MAIN ======
#[tokio::main]
async fn main() -> Result<()> {
    // load data first
    load_spotify_songs().await?;

    // --- CORS setup ---
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET]);

    // --- Router setup ---
    let app = Router::new()
        .route("/", get(hello))
        .route("/songs", get(get_songs))
        .route("/recommendations", get(get_recommendations))
        .layer(cors);

    // Read PORT from environment variable (Cloud Run sets this)
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port).parse().unwrap();
    println!("🚀 Server running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
