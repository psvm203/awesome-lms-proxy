mod clients;
mod handlers;
mod models;
mod router;
mod utils;

use worker::*;

#[event(fetch)]
async fn fetch(request: Request, _env: Env, _context: Context) -> Result<Response> {
    router::dispatch(request).await
}
