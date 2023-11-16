use warp::{http::StatusCode, Rejection, Reply};

use crate::handlers::redirect::NotFoundError;
use crate::InternalServerError;

pub(crate) async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let code;
    let message;

    if let Some(_) = err.find::<NotFoundError>() {
        // NotFoundErrorのチェックを追加
        code = StatusCode::NOT_FOUND;
        message = "URL not found";
    } else if let Some(_) = err.find::<InternalServerError>() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Unhandled error";
    }

    let json = warp::reply::json(&{
        let mut map = std::collections::HashMap::new();
        map.insert("message", message);
        map
    });

    Ok(warp::reply::with_status(json, code))
}
