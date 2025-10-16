# Rustify – Music Recommendation App

**Rustify** is a full-stack Rust application that recommends songs based on your favourite artist or genre.  
It consists of a **backend** service built with [Axum](https://crates.io/crates/axum) and [Tokio](https://crates.io/crates/tokio), and a **frontend** written with the [Yew](https://yew.rs/) framework compiled to WebAssembly (WASM).

---

## Overview

Rustify loads a dataset of music tracks (in CSV format) and provides intelligent song recommendations through a RESTful API.  
The frontend interacts with the backend API to deliver a modern, browser-based user interface where users can explore recommendations by artist or genre.

**Architecture:**

- **Backend (Axum + Tokio):**
  - Loads `tracks.csv` into memory.
  - Provides endpoints to query songs by artist, genre, or similarity score.
  - Uses multithreading and asynchronous I/O for high performance.
- **Frontend (Yew + Trunk):**
  - Compiles to WASM for execution in the browser.
  - Fetches data from the backend using HTTP requests.
  - Provides a clean, reactive UI for exploring tracks.

---

## Project Structure

```
rustify/
├── backend/
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   └── public/             # Static assets served by the backend
├── frontend/
│   ├── Cargo.toml
│   ├── index.html
│   └── src/
│       └── main.rs
├── app.yaml                # App Engine configuration
└── README.md
```

---

## How It Works

1. The backend reads a CSV dataset containing track metadata (artist, genre, tempo, etc.).
2. Data is stored in in-memory hash maps for fast lookups by artist or genre.
3. The `/recommend` endpoint computes a similarity score between the target and candidate songs based on track features.
4. The frontend provides a search interface for users to request recommendations.
5. Results are displayed dynamically using Yew’s component-based architecture.

---

## API Endpoints

| Endpoint | Method | Description |
|-----------|--------|--------------|
| `/tracks` | GET | Returns all available tracks |
| `/tracks/:artist` | GET | Returns tracks for the specified artist |
| `/recommend` | GET | Returns recommended tracks based on artist or genre |

Example request:

```
GET http://localhost:8080/recommend?artist=Daft+Punk
```

---

## Prerequisites

Before running the project locally, install the following:

- [Rust (latest stable)](https://www.rust-lang.org/tools/install)
- [Trunk](https://trunkrs.dev/#install)
- [Node.js](https://nodejs.org/) (optional, for static serving)
- [Google Cloud SDK](https://cloud.google.com/sdk/docs/install) (for deployment)

---

## 🧪 Running Locally

1. **Clone the repository**
   ```bash
   git clone https://github.com/g3n3s1s-a/rustify.git
   cd rustify
   ```

2. **Start the backend**
   ```bash
   cd backend
   cargo run
   ```
   The API will be available at `http://localhost:8080`.

3. **Run the frontend**
   ```bash
   cd frontend
   trunk serve --open
   ```
   The app will open in your browser at `http://localhost:8081`.

---

## Deployment (Google Cloud)

Once ready to deploy:

1. **Build frontend**
   ```bash
   cd frontend
   trunk build --release
   ```

   This creates a `dist/` folder with optimized WASM and assets.

2. **Copy the `dist/` folder to the backend’s `public/` directory:**
   ```bash
   cp -r dist/* ../backend/public/
   ```

3. **Deploy using App Engine**
   ```bash
   cd ../backend
   gcloud app deploy
   ```

4. Access your hosted app at:

   ```
   https://<placeholder-cloud-link>/
   ```

   *(Replace `<placeholder-cloud-link>` once deployed.)*

---

## Example Use

- Search by artist name: *Daft Punk, Coldplay, or The Weeknd*
- Filter by genre: *pop, rock, electronic*
- See recommended tracks with dynamic front-end updates

---



### Live App

Once deployed, you’ll be able to access the hosted app here:

https://rustify-project.uc.r.appspot.com
