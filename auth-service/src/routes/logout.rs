use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState, domain::AuthAPIError, utils::{auth::validate_token,
        constants::JWT_COOKIE_NAME
    }
};


pub async fn logout(State(state): State<AppState>,jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match  jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        _ => return (jar, Err(AuthAPIError::MissingToken))
    };

    let token = cookie.value().to_owned();

    
    let banned_tk_store =  state.banned_token_store.clone();
    if validate_token(&token, banned_tk_store).await.is_err() {
        return (jar, Err(AuthAPIError::InvalidToken))
    }
    

    // drop(banned_tk_store);
    let mut banned_tk_store = state.banned_token_store.write().await;

    match banned_tk_store.store_banned_token(token).await {
        Ok(()) => {
            let jar = jar.remove(JWT_COOKIE_NAME);
            (jar, Ok(StatusCode::OK))
        }
        Err(_) => (jar, Err(AuthAPIError::UnexpectedError)),
    }
}
