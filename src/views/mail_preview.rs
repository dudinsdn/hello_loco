use loco_rs::prelude::*;

pub fn page(
    view: impl ViewRenderer,
    kind: &str,
    email: crate::mailers::auth::EmailPreview,
) -> Result<Response> {
    format::view(
        &view,
        "mail_preview/index.html",
        data!({"title": "Pratinjau email", "kind": kind, "email": email}),
    )
}
