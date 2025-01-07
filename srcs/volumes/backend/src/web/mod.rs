pub mod error;
pub mod routes_login;

pub mod mw_auth;
pub mod mw_res_map;
pub mod routes_static;
pub mod routes_tickets;
pub mod routes_user;

pub const AUTH_TOKEN: &str = "auth-token";

use tower_cookies::{Cookie, Cookies};

use crate::crypt::token::generate_web_token;

pub use self::error::{Error, Result};

fn set_auth_token_cookie(cookies: &Cookies, user: &str, salt: &str) -> Result<()> {
    let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);
    Ok(())
}

fn remove_auth_token_cookie(cookies: &Cookies) {
    cookies.remove(Cookie::from(AUTH_TOKEN));
}
