use std::sync::Arc;

use reqwest::StatusCode;

pub use responses::*;

use crate::config::GamesConfig;
use crate::data::games::clash_royale_tag_validator;
use crate::error::{AppError, AppResult};

mod responses;

pub struct ClashRoyaleClient {
    pub games_config: Arc<GamesConfig>,
    pub http_client: Arc<reqwest::Client>,
}

impl ClashRoyaleClient {
    // CONSTRUCTORS -----------------------------------------------------------

    pub fn new(games_config: Arc<GamesConfig>, http_client: Arc<reqwest::Client>) -> Self {
        ClashRoyaleClient {
            games_config,
            http_client,
        }
    }

    // METHODS ----------------------------------------------------------------

    pub async fn get_player_info(&self, tag: &str) -> AppResult<ClashRoyalePlayerInfoResponse> {
        if !clash_royale_tag_validator(tag) {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                arcstr::literal!("incorrect_clash_royale_tag"),
            )
            .message(arcstr::literal!("The Clash Royale tag is incorrect")));
        }

        let config = &self.games_config.clash_royale;
        let response = self
            .http_client
            .get(format!("{}/players/%23{}", config.url, tag))
            .bearer_auth(&config.token)
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                arcstr::literal!("undefined_clash_royale_player"),
            )
            .message(arcstr::literal!("Missing Clash Royale player")));
        }

        let body: ClashRoyalePlayerInfoResponse = response.json().await?;

        Ok(body)
    }

    pub async fn get_player_battlelog(
        &self,
        tag: &str,
    ) -> AppResult<Vec<ClashRoyaleBattlelogResponse>> {
        if !clash_royale_tag_validator(tag) {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                arcstr::literal!("incorrect_clash_royale_tag"),
            )
            .message(arcstr::literal!("The Clash Royale tag is incorrect")));
        }

        let config = &self.games_config.clash_royale;
        let response = self
            .http_client
            .get(format!("{}/players/%23{}/battlelog", config.url, tag))
            .bearer_auth(&config.token)
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            return Err(AppError::new_with_status(
                warp::http::StatusCode::BAD_REQUEST,
                arcstr::literal!("undefined_clash_royale_player"),
            )
            .message(arcstr::literal!("Missing Clash Royale player")));
        }

        let body: Vec<ClashRoyaleBattlelogResponse> = response.json().await?;

        Ok(body)
    }
}
