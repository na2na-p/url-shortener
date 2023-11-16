use std::sync::Arc;

use chrono::Utc;
use warp::{http::Response, http::StatusCode, reject, Rejection, Reply};

use crate::dao::shortened_url_dao::ShortenedUrlDao;

// カスタムリジェクションの定義
#[derive(Debug)]
pub struct NotFoundError;

impl reject::Reject for NotFoundError {}

// カスタムリジェクションの定義
#[derive(Debug)]
pub struct InternalServerError;

impl reject::Reject for InternalServerError {}

pub async fn redirect_short_url(shortened_url_dao: Arc<ShortenedUrlDao>, short_code: String) -> Result<impl Reply, Rejection> {
    match shortened_url_dao.find_by_short_code(&short_code).await {
        Ok(Some(shortened_url)) => {
            // 現在の時刻を取得
            let now = Utc::now();

            // 有効期限が設定されており、現在時刻が有効期限を過ぎている場合はNotFoundエラーを返す
            if let Some(expires_at) = shortened_url.expires_at {
                if now > expires_at {
                    shortened_url_dao.delete_by_short_code(&short_code).await.expect("Failed to delete The Record.");
                    return Err(reject::custom(NotFoundError));
                }
            }

            // 有効期限内の場合、リダイレクトを実行
            Ok(Response::builder()
                .status(StatusCode::FOUND) // 302リダイレクト
                .header("Location", shortened_url.original_url)
                .body("Redirecting...") // 明示的に &str 型を使用
                .unwrap())
        }
        Ok(None) => {
            // 対応するURLが見つからなかった場合
            Err(reject::custom(NotFoundError))
        }
        Err(_) => {
            // エラーが発生した場合
            Err(reject::custom(InternalServerError))
        }
    }
}
