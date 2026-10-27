# Welcome to Loco :train:

[Loco](https://loco.rs) is a web and API framework running on Rust.

This is the **SaaS starter** which includes a `User` model and authentication based on JWT.
It also include configuration sections that help you pick either a frontend or a server-side template set up for your fullstack server.


## Quick Start

```sh
cargo loco start
```

```sh
$ cargo loco start
Finished dev [unoptimized + debuginfo] target(s) in 21.63s
    Running `target/debug/myapp start`

    :
    :
    :

controller/app_routes.rs:203: [Middleware] Adding log trace id

                      ▄     ▀
                                 ▀  ▄
                  ▄       ▀     ▄  ▄ ▄▀
                                    ▄ ▀▄▄
                        ▄     ▀    ▀  ▀▄▀█▄
                                          ▀█▄
▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄ ▀▀█
 ██████  █████   ███ █████   ███ █████   ███ ▀█
 ██████  █████   ███ █████   ▀▀▀ █████   ███ ▄█▄
 ██████  █████   ███ █████       █████   ███ ████▄
 ██████  █████   ███ █████   ▄▄▄ █████   ███ █████
 ██████  █████   ███  ████   ███ █████   ███ ████▀
   ▀▀▀██▄ ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀ ██▀
       ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
                https://loco.rs

environment: development
   database: automigrate
     logger: debug
compilation: debug
      modes: server

listening on http://localhost:5150
```

## Full Stack Serving

You can check your [configuration](config/development.yaml) to pick either frontend setup or server-side rendered template, and activate the relevant configuration sections.


## Getting help

Check out [a quick tour](https://loco.rs/docs/tutorials/the-tour/) or [build your first app](https://loco.rs/docs/tutorials/your-first-app/).

## The Tour in this project

Continues [The Tour](https://loco.rs/docs/tutorials/the-tour/) using the existing
SQLite app and posts scaffold. The existing nullable `posts.title` is preserved;
the report displays `(untitled)` for a missing title.

```sh
cargo loco start
# In another terminal (use the id returned by the first request):
curl -X POST -H 'Content-Type: application/json' -d '{"title":"Tour post","content":"hi"}' localhost:5150/api/posts
curl -X POST -H 'Content-Type: application/json' -d '{"content":"nice post","post_id":1}' localhost:5150/api/comments
curl localhost:5150/api/posts/1/comments
cargo loco routes
cargo loco task
cargo loco task posts_report
```

Posts retain their CRUD routes. Comments support creation and listing under
`/api/posts/{id}/comments`; they have no standalone read, update, or delete route.
Creating a post enqueues `notifier`, which prints its post id (no email is sent).
Development uses `BackgroundAsync`; tests use `ForegroundBlocking`.

```sh
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
```

## Authenticated app tutorial

Continues [Build a Small Authenticated App](https://loco.rs/docs/tutorials/saas-with-auth/)
in this existing project. Development email uses `mailer.stub: true`, so
registration works without an SMTP catcher and no real email is sent.

```sh
cargo loco start
```

In another terminal:

```sh
curl -X POST localhost:5150/api/auth/register \
  -H 'Content-Type: application/json' \
  -d '{"name":"Loco user","email":"user@loco.rs","password":"12341234"}'
curl -X POST localhost:5150/api/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"email":"user@loco.rs","password":"12341234"}'
# Copy the token from the login response:
TOKEN='paste-login-token-here'
curl localhost:5150/api/auth/current -H "Authorization: Bearer $TOKEN"
# Without a token: 401
curl -X POST localhost:5150/api/notes \
  -H 'Content-Type: application/json' \
  -d '{"title":"secret","content":"shh"}'
# With a valid token for an existing user: 201
curl -X POST localhost:5150/api/notes \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"title":"secret","content":"shh"}'
```

This follows the tutorial's deliberately limited protection: only creating a
note requires JWT authentication and a user lookup. Listing, reading, updating,
and deleting notes remain public. Notes are not assigned to an owner or tenant;
this is an authentication exercise, not private per-user storage. Login and
note creation do not require a verified email. Existing posts/comments retain
their previous behavior.

The scaffold is generated with `--no-auth`, then JWT is added to `create`,
because this Loco version otherwise protects all scaffold actions by default.
The generated response DTOs are retained instead of serializing entities
as the tutorial snippet does. Note creation is kept on the model per the local Loco guide. The existing auth
suite is reused unchanged.

## Server-side frontend

The existing app now includes Tera server-side rendering, using Loco's
[API-only app setup](https://loco.rs/docs/how-to/render-views/).
Run `cargo loco start`, then visit http://localhost:5150/.

- `src/initializers/view_engine.rs` installs the Tera view engine.
- `src/controllers/home.rs` serves the home page through `src/views/home.rs`.
- `assets/views/` contains the base layout and home template.
- `assets/static/` contains CSS and the static fallback page.
- All environments serve static assets at `/static`; existing APIs stay at `/api`.

The home page provides the server-side frontend foundation; its API links open
JSON responses. Login forms are described below. A dashboard and per-user note
ownership are not implemented. Deploy the `assets/` directory alongside the
binary and `config/` because templates and static files are loaded from disk.

## Login and mail template previews

- `/login`: sign in using the existing `/api/auth/login` endpoint and display
  the current user's name/email from `/api/auth/current`.
- `/register`: submit account registration through the existing auth API.
- `/mail-preview/welcome`, `/mail-preview/forgot`, `/mail-preview/magic_link`:
  development/test-only previews of the actual `src/mailers/auth/*/*.t` sources.
  Each shows the rendered subject, HTML in a sandboxed iframe, plain text, and
  original source. Fixed sample values and nonfunctional example links are used;
  previewing does not send mail or create users/tokens.

The login page is an auth demonstration, not a persistent browser session or a
dashboard: the JWT is used only in memory for the current-user request, never
stored in localStorage/cookies or displayed. Refreshing requires signing in
again. “Keluar” clears the displayed identity; it does not revoke an issued JWT.
Development still uses the mail stub, so registering does not send real email.
Mail preview data is independent from registration and contains no real tokens.

`subject.t` defines the email subject, `html.t` the HTML body, and `text.t` its
plain-text alternative. They are embedded at compile time, so changing these
mailer templates requires rebuilding the Rust binary. Existing reset email
links refer to `/reset`, whose browser flow is outside this login/preview task.
