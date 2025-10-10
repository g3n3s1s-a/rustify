# Rustify

## Setup

1. Install Trunk: `cargo install trunk`
2. Add WASM target: `rustup target add wasm32-unknown-unknown`

## Local Development
```bash
trunk serve
```
## Deploy to Google Cloud

1.  Build for production
`trunk build --release`

2. Deploy to App Engine
`gcloud app deploy`
