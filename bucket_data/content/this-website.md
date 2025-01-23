*I made this website from scratch! Because why would you set up a basic WordPress website in minutes when you can spend
weeks writing something that does pretty much the same? On multiple occasions, I was unable to answer that question. But
I did end up learning a lot, and now I can easily add any features I like, which is pretty cool.*

## Stack

- The backend is written in Rust
    - It uses the [axum](https://crates.io/crates/axum) web framework
    - [sqlx](https://crates.io/crates/sqlx) for interacting with a Postgres database
    - Amazon's [aws-sdk-s3](https://crates.io/crates/aws-sdk-s3) crate for storing these markdown files and
      thumbnail images
- The frontend is written using basic templates
    - It uses the [askama](https://crates.io/crates/askama) templating engine
    - I used [tailwindcss](https://tailwindcss.com/) to write as little CSS as necessary
    - For improved interactivity, I tried [htmx](https://htmx.org/) for the first time
- It uses some external services
    - A self-hosted [Keycloak](https://www.keycloak.org/) instance for OpenID Connect authentication
    - [Cloudflare](https://www.cloudflare.com/)'s R2 for S3 compatible object storage
        - For local testing, a [Docker](https://www.docker.com/) Compose setup spins up [MinIO](https://min.io/) instead
    - A [PostgreSQL](https://www.postgresql.org/) database
    - [Velero](https://velero.io/) for backing up the database from time to time
- It runs on my dual-node [k3s](https://k3s.io/) cluster
    - GitHub Actions automatically builds Docker images for new versions
    - [fleet](https://fleet.rancher.io/) automatically deploys those images

## Lessons learned

Overall, I'm happy I tried this stack. A few days in, I was even convinced that using simple templates in combination
with HTMX was far superior to using a frontend framework for the majority of websites. Maybe I still kind of am, but I
do see some problems with it now.

It's not visible for the majority of the readers (assuming that at least one other person is going to be reading this),
but this website has a full-fledged content management system built-in. One useful feature for me would be to have a
preview button that displays the content I've written so far as if I'd already published it, so I can verify it looks as
intended. But due to templates being completely server-side, the most straightforward way to implement this would be to
use HTMX to send a request to an endpoint that converts the markdown written so far to html. That's not necessarily a
problem for the one person writing content for this website, but it is something that could easily be done client-side.
Doing this manually with JavaScript in your template, however, is cumbersome without some npm-like setup.

Another drawback is that the frontend and backend become more closely tied together. Some endpoints return full HTML
pages, others only reply partial HTML fragments for HTMX to insert into an already loaded full page. In many cases you
even require both types of endpoints for the same data, for example when you want to support deeplinks or users that
disable JavaScript (the latter was a random requirement that I imposed on myself, because this stack
allows [graceful degradation](https://developer.mozilla.org/en-US/docs/Glossary/Graceful_degradation) more easily than
frontend frameworks. I gave up halfway through when I realized what a pain it was). Requiring new endpoints in the
backend depending on how I wanted the frontend to look was acceptable for this project, especially because the backend
and frontend are both in the same crate. For larger projects though, I think it gets old quickly.

All in all, I think the *templates + HTMX* portion of the stack is awesome for relatively small projects, of which you
know in advance how the end result should look. Otherwise, I'd opt for a frontend framework for its flexibility.
