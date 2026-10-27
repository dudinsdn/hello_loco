(() => {
  "use strict";
  const form = document.getElementById("account-form");
  const message = document.getElementById("form-message");
  const button = form.querySelector('button[type="submit"]');
  const signedIn = document.getElementById("signed-in");
  let busy = false;

  form.addEventListener("submit", async (event) => {
    event.preventDefault();
    if (busy) return;
    busy = true;
    button.disabled = true;
    form.setAttribute("aria-busy", "true");
    message.textContent = "Sebentar, ya…";
    message.classList.remove("error");
    const register = form.dataset.mode === "register";
    const fields = new FormData(form);
    const payload = {
      email: fields.get("email"),
      password: fields.get("password"),
    };
    if (register) payload.name = fields.get("name");
    try {
      const response = await fetch(`/api/auth/${register ? "register" : "login"}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
        cache: "no-store",
      });
      if (!response.ok) {
        message.classList.add("error");
        message.textContent = response.status === 401
          ? "Email atau kata sandi belum cocok. Coba lagi, ya."
          : "Belum berhasil. Periksa isian Anda lalu coba lagi.";
        return;
      }
      if (register) {
        message.textContent = "Permintaan pendaftaran diterima. Anda bisa mencoba masuk. Jika email sudah digunakan, gunakan akun yang sudah ada.";
        form.reset();
        return;
      }
      const login = await response.json();
      const current = await fetch("/api/auth/current", {
        headers: { Authorization: `Bearer ${login.token}` },
        cache: "no-store",
      });
      if (!current.ok) throw new Error("current user unavailable");
      const user = await current.json();
      document.getElementById("user-name").textContent = user.name;
      document.getElementById("user-email").textContent = user.email;
      form.reset();
      form.hidden = true;
      signedIn.hidden = false;
      document.getElementById("logout").focus();
    } catch {
      message.classList.add("error");
      message.textContent = "Tidak dapat terhubung. Silakan coba lagi.";
    } finally {
      busy = false;
      button.disabled = false;
      form.removeAttribute("aria-busy");
      document.getElementById("password").value = "";
    }
  });

  document.getElementById("logout").addEventListener("click", () => {
    signedIn.hidden = true;
    form.hidden = false;
    document.getElementById("user-name").textContent = "";
    document.getElementById("user-email").textContent = "";
    message.textContent = "Anda sudah keluar.";
    message.classList.remove("error");
    document.getElementById("email").focus();
  });
})();
