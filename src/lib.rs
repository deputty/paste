use base62::decode;
use http::StatusCode;
use worker::event;
use worker::{Env, Context, Result};
use worker::{Request, Response};

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let url = req.url()?;
    let slug = url
        .path_segments()
        .and_then(|mut seg| seg.next())
        .unwrap_or_default();
    let kv = env.kv("KV")?;
    if let Ok(id) = decode(slug) {
        if let Some(text) = kv.get(&format!("pastes:{}", id)).text().await? {
            Response::ok(text)
        } else {
            Response::error("not found", StatusCode::NOT_FOUND.as_u16())
        }
    } else {
        Response::error("not found", StatusCode::NOT_FOUND.into())
    }
}
