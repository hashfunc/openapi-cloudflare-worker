use crate::response::ResponseWithContentType;
use worker::*;

pub async fn kv_handler(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let key = match ctx.param("key") {
        None => return Response::error("Bad request", 400),
        Some(key) => key,
    };

    let response = ctx.kv("DATA")?.get(key).text().await;

    match response? {
        None => Response::error("Not found", 404),
        Some(value) => Response::ok(value)?.content_type("application/json"),
    }
}
