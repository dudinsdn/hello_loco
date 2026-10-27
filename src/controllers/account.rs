use loco_rs::prelude::*;

#[debug_handler]
async fn login(ViewEngine(view): ViewEngine<TeraView>) -> Result<Response> {
    crate::views::account::page(view, false)
}

#[debug_handler]
async fn register(ViewEngine(view): ViewEngine<TeraView>) -> Result<Response> {
    crate::views::account::page(view, true)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/")
        .add("/login", get(login))
        .add("/register", get(register))
}
