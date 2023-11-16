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

// 短縮URLのリダイレクト用のハンドラー
pub async fn redirect_short_url(shortened_url_dao: ShortenedUrlDao, short_code: String) -> Result<impl Reply, Rejection> {
    match shortened_url_dao.find_by_short_code(&short_code).await {
        Ok(Some(shortened_url)) => {
            // 対応するURLが見つかった場合、そのURLにリダイレクト
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
