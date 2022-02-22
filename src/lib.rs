use worker::*;

mod handlers;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    Router::new()
        .get_async("/kv/:key", handlers::kv_handler)
        .run(req, env)
        .await
}
