use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let r = Router::new();
    r.get("/", |_, _| Response::ok("Hello from Logrocket Workers!"))
        .run(req, env)
        .await
}
