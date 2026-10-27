use loco_rs::prelude::*;

pub fn page(view: impl ViewRenderer, register: bool) -> Result<Response> {
    format::view(
        &view,
        "account/index.html",
        data!({
            "title": if register { "Daftar akun" } else { "Masuk" },
            "register": register,
        }),
    )
}
