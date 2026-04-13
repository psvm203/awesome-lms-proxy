mod clients;
mod handler;
mod model;
mod router;
mod utils;

use worker::*;

#[event(fetch)]
async fn fetch(request: Request, _env: Env, _context: Context) -> Result<Response> {
    router::dispatch(request).await
}
