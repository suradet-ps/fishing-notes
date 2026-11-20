# Fishing Notes

> Thai-first fishing notes from Japanese pro-angler videos - seabass, bass, eging, and tenya.

Fishing Notes is a Thai-language blog that distills lure-fishing techniques from
Japanese pro anglers. Every article is a plain Markdown file in `content/posts/`
and is picked up by the site automatically.

The site uses the same architecture as
[`rust-blog.github.io`](https://github.com/rust-blog/rust-blog.github.io):

- **Rust 2024 + Leptos 0.8 (CSR)** - the UI is written in Rust
- **Trunk** - compiles the app to WebAssembly
- **pulldown-cmark** - renders Markdown to HTML
- **syntect** - syntax-highlights code blocks at render time
- **GitHub Pages** - deployed with GitHub Actions

## Quick start

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve
```

Open [http://127.0.0.1:8080](http://127.0.0.1:8080).

Production build:

```bash
trunk build --release --public-url /fishing-notes/
cargo run --bin post_build
```

Regenerate the OG image (requires Pillow):

```bash
python scripts/render-og-image.py
```

Regenerate the article covers from their source frames (requires Pillow):

```bash
python scripts/make-covers.py
```

## Writing a post

Create a `.md` file in `content/posts/` with frontmatter:

```yaml
---
title: "Post title"
date: "2026-11-14"
description: "One or two sentence summary"
tags: ["seabass", "lures", "beginner"]
author: "suradet-ps"
lang: th
# cover: "assets/my-cover.jpg"   # optional
---

Markdown content...
```

- `draft: true` hides the post
- `slug` is optional - the filename is used as the slug (must match `a-z`, `0-9`, `-`)
- Put images in `content/assets/` and reference them as `![alt](assets/name.png)`
- Tables, task lists, and footnotes are supported via GitHub-flavored Markdown

### Covers

A post's cover, in priority order, is:

1. the `cover:` frontmatter reference, when present;
2. `content/assets/<slug>-cover.jpg`, if that file exists;
3. the first Markdown image in the body.

Generated covers are 21:9 cinematic crops with a shared, light grade. The
source-frame map lives in `scripts/make-covers.py`; add a slug there and rerun
the script when you want a curated frame. When the cover is the first body
image, the renderer moves it out of the prose and into the article header, so
the same still never appears twice.

### Inline video clips

Short technique clips can be embedded with a fenced `video` block:

````markdown
```video
src: assets/clip.mp4
poster: assets/clip-poster.jpg
title: Two-finger grip
caption: The index and middle fingers pinch the reel seat.
muted: true
```
````

The renderer emits a native `<video controls playsinline preload="metadata">`
figure; `src` and `poster` are validated against `content/assets/` at build
time. Keep clips short, use a poster still that is different from the nearby
article image, and leave `muted: true` unless the audio is essential.

## Project layout

```text
content/
  posts/          # Markdown articles
  assets/         # Images, generated covers, supporting files
fonts/            # Self-hosted IBM Plex Sans Thai + Mono (SIL OFL)
scripts/          # Cover and OG image generators (Pillow)
src/
  content.rs      # Loads posts, picks covers, builds the TOC
  frontmatter.rs  # Frontmatter parser + validation
  markdown.rs     # Markdown -> HTML, captions, heading anchors, video blocks
  video.rs        # Parse and validate inline video directives
  pages/          # Home / About / Post / 404
  components.rs   # Nav, Footer, cards, theme toggle
styles/
  main.css        # Design system (light/dark)
build.rs          # RSS, sitemap, robots, asset fingerprinting
```

## Deploy

This repo is intended to be named **`fishing-notes`** and deployed as a project
site:

```text
https://suradet-ps.github.io/fishing-notes/
```

If you change the username or repository name, update four places:

1. `src/site.rs` - `SITE_URL`, `GITHUB_URL`
2. `src/util.rs` - `detect_base()` to match the repo base path
3. `index.html` - `og:image`, `og:url`, Twitter image
4. `.github/workflows/deploy.yml` and `ci.yml` - `--public-url /<repo>/`

If you use a custom domain or a `<username>.github.io` site, set `--public-url`
to `/`.

If the domain changes, also update the URL baked into the OG card in
`scripts/render-og-image.py`, then rerun `python scripts/render-og-image.py`.

## Sources

Each article cites its source video in a summary box at the top and at the end
of the post, with the video title, channel, YouTube link, and notes about
translation from auto-generated captions.

## License

MIT - see [LICENSE](LICENSE).

The IBM Plex fonts in `fonts/` are licensed under the SIL Open Font License;
see `fonts/OFL.txt`.
