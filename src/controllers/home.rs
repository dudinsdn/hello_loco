use loco_rs::prelude::*;

#[debug_handler]
async fn index(
    State(ctx): State<AppContext>,
    ViewEngine(view): ViewEngine<TeraView>,
) -> Result<Response> {
    crate::views::home::index(
        view,
        matches!(
            ctx.environment,
            loco_rs::environment::Environment::Development
                | loco_rs::environment::Environment::Test
        ),
    )
}

pub fn routes() -> Routes {
    Routes::new().prefix("/").add("/", get(index))
}
