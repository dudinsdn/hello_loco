use loco_rs::prelude::*;

#[debug_handler]
async fn preview(
    ViewEngine(view): ViewEngine<TeraView>,
    Path(kind): Path<String>,
) -> Result<Response> {
    let Some(email) = crate::mailers::auth::AuthMailer::preview(&kind)? else {
        return not_found();
    };
    crate::views::mail_preview::page(view, &kind, email)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/mail-preview")
        .add("/{kind}", get(preview))
}
