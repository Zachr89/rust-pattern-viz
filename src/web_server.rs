//! Web server for sharing analysis results via HTTP API.
//!
//! Provides REST endpoints for creating and retrieving shared analyses.

use crate::models::AnalysisReport;
use crate::share::{ShareService, SharedAnalysis};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use warp::{http::StatusCode, Filter, Rejection, Reply};

#[derive(Debug, Deserialize)]
struct CreateShareRequest {
    report: AnalysisReport,
}

#[derive(Debug, Serialize)]
struct CreateShareResponse {
    share_id: String,
    share_url: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Serialize)]
struct GetShareResponse {
    share: SharedAnalysis,
}

/// Web server for share API
pub struct ShareServer {
    service: Arc<Mutex<ShareService>>,
    base_url: String,
}

impl ShareServer {
    /// Create a new share server
    pub fn new(storage_dir: PathBuf, base_url: String) -> std::io::Result<Self> {
        let service = ShareService::new(storage_dir)?;
        Ok(Self {
            service: Arc::new(Mutex::new(service)),
            base_url,
        })
    }

    /// Start the web server
    pub async fn run(self, port: u16) {
        let service = self.service.clone();
        let base_url = self.base_url.clone();

        // POST /api/share - Create a new share
        let create_share = warp::path!("api" / "share")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_service(service.clone()))
            .and(with_base_url(base_url.clone()))
            .and_then(handle_create_share);

        // GET /api/share/:id - Retrieve a share
        let get_share = warp::path!("api" / "share" / String)
            .and(warp::get())
            .and(with_service(service.clone()))
            .and_then(handle_get_share);

        // GET /api/health - Health check
        let health = warp::path!("api" / "health")
            .and(warp::get())
            .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

        let routes = create_share
            .or(get_share)
            .or(health)
            .with(warp::cors().allow_any_origin().allow_methods(vec!["GET", "POST"]));

        println!("Share server running on http://0.0.0.0:{}", port);
        warp::serve(routes).run(([0, 0, 0, 0], port)).await;
    }
}

fn with_service(
    service: Arc<Mutex<ShareService>>,
) -> impl Filter<Extract = (Arc<Mutex<ShareService>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || service.clone())
}

fn with_base_url(
    base_url: String,
) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || base_url.clone())
}

async fn handle_create_share(
    req: CreateShareRequest,
    service: Arc<Mutex<ShareService>>,
    base_url: String,
) -> Result<impl Reply, Rejection> {
    let service = service.lock().unwrap();

    match service.create_share(req.report) {
        Ok(share_id) => {
            let share_url = format!("{}/share/{}", base_url, share_id);
            let response = CreateShareResponse {
                share_id,
                share_url,
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::CREATED,
            ))
        }
        Err(e) => {
            let response = ErrorResponse {
                error: format!("Failed to create share: {}", e),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

async fn handle_get_share(
    share_id: String,
    service: Arc<Mutex<ShareService>>,
) -> Result<impl Reply, Rejection> {
    let service = service.lock().unwrap();

    match service.get_share(&share_id) {
        Ok(share) => {
            let response = GetShareResponse { share };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::OK,
            ))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let response = ErrorResponse {
                error: "Share not found or expired".to_string(),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::NOT_FOUND,
            ))
        }
        Err(e) => {
            let response = ErrorResponse {
                error: format!("Failed to retrieve share: {}", e),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_service_filter() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let service = ShareService::new(temp_dir.path().to_path_buf()).unwrap();
        let arc_service = Arc::new(Mutex::new(service));
        let filter = with_service(arc_service.clone());

        // Filter should produce the same Arc
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let result = warp::test::request()
                .filter(&filter)
                .await
                .unwrap();
            assert!(Arc::ptr_eq(&result, &arc_service));
        });
    }
}
