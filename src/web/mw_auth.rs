use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{async_trait, RequestPartsExt};
use lazy_regex::regex_captures;
use tower_cookies::Cookie;
use tower_cookies::Cookies;

use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

//MW is for middleware
pub async fn mw_requiure_auth<B>(
    ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:^12} - mw_require_aut - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver<B>(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:^12} - mw_ctx_aut - ", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    //Compute Result<Ctx>
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => {
            //TODO: Components validations
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e),
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    //Store ctx_result in the request extension
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

// Region: -- Ctx extractor

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:^12} - Ctx", "EXTRACTOR");

        //User cookies extractor
        // let cookies = parts.extract::<Cookies>().await.unwrap();

        // let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // Parse token
        // let (user_id, exp, sign) = auth_token
        //     .ok_or(Error::AuthFailNoAuthTokenCookie)
        //     .and_then(parse_token)?;

        // Ok(Ctx::new(user_id))

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}

// Region: -- Ctx extractor

//Parse a token of format "user-[user-id].[expiration].[signature]"
//Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, //a literal regex
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
