use loco_rs::prelude::*;

pub fn index(view: impl ViewRenderer, show_preview: bool) -> Result<Response> {
    format::view(
        &view,
        "home/index.html",
        data!({"title": "Hello, Loco.", "show_preview": show_preview}),
    )
}
