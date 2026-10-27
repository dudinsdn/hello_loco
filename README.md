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
