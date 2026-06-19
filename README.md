> We're dsplce.co, check out our work on our website: [dsplce.co](https://dsplce.co) 🖤

# dioxus-feather-icons

[![Dioxus](https://img.shields.io/badge/Dioxus-0772B8?style=for-the-badge&logo=rust&logoColor=white)](https://dioxuslabs.com/)
[![crates.io Downloads](https://img.shields.io/crates/d/dioxus-feather-icons?style=for-the-badge&color=%23FF0346)](https://crates.io/crates/dioxus-feather-icons)
[![crates.io Size](https://img.shields.io/crates/size/dioxus-feather-icons?style=for-the-badge)](https://crates.io/crates/dioxus-feather-icons)
[![License](https://img.shields.io/crates/l/dioxus-feather-icons.svg?style=for-the-badge)](https://crates.io/crates/dioxus-feather-icons)
[![crates.io](https://img.shields.io/crates/v/dioxus-feather-icons?style=for-the-badge&color=%230F80C1)](https://crates.io/crates/dioxus-feather-icons)

⚡ The whole set of [Feather Icons](https://feathericons.com/) for [Dioxus](https://dioxuslabs.com/) — inline SVG, checked at build time, paid for at compile time instead of runtime.

`dioxus-feather-icons` drops Feather into your Dioxus UI with a couple of macros: render an icon by name, size and colour it when you need to, and grab the raw SVG string if you'd rather have the markup. No icon font, no JS to ship, no folder of `.svg` files to babysit.

_Disclaimer: this project has no affiliation with Feather Icons or Dioxus, nor with their respective trademarks._

## 🖤 Features

- **Inline SVG, no runtime** — icons are just markup. No icon font to load, no JavaScript to ship, no flash of empty boxes whilst something hydrates; the compiler does the work, the browser just draws.
- **Typo the name, fail the build — not the page** — `icon!(activity)` is resolved when you `cargo build`, so a fat-fingered `icon!(activty)` never makes it to prod where a client clicks around and finds a blank square. You find out on your machine, not theirs.
- **The whole Feather set, bundled** — ~290 icons ship inside the crate. You don't download them, vendor them, or watch a folder of `.svg` files quietly drift out of date.
- **One sprite, not 50 copies** — every icon is a single `<use>` reference into one shared sprite, so a page with fifty icons carries fifty tiny references, not fifty duplicated copies of the same paths.
- **One macro, dialled up only when you need it** — start with `icon!(camera)`, add a size, add a colour. The customisation shows up the moment you reach for it and stays out of the way until then.

---

## Table of Contents

- [🖤 Features](#-features)
- [📦 Installation](#-installation)
- [🧪 Usage](#-usage)
  - [Inject the sprite (once)](#inject-the-sprite-once)
  - [Render an icon](#render-an-icon)
  - [Grab the raw SVG string](#grab-the-raw-svg-string)
- [🛠️ Requirements](#%EF%B8%8F-requirements)
- [📁 Repo & Contributions](#-repo--contributions)
- [📄 License](#-license)

⸻

## 📦 Installation

Add it to your `Cargo.toml`:

```toml
[dependencies]
dioxus-feather-icons = "0.2"
```

Or let cargo do the typing:

```bash
cargo add dioxus-feather-icons
```

The crate is on the **Rust 2024 edition**, so your project needs to be too.

⸻

## 🧪 Usage

There are three moving parts, and the first one is easy to forget — so we'll start there.

### Inject the sprite (once)

Here's the thing worth knowing up front: every icon you render is really a `<use href="#name">` pointing at one shared sprite. So that sprite has to live on the page exactly once — miss it and you get a pile of `<use>` tags pointing at nothing, ergo a whole lot of invisible icons and no error to tell you why.

Drop it in once, toward the end of your body:

```rust
rsx! {
  // ... your app ...
  { dioxus_feather_icons::sprite!() }
}
```

Keep it near the end of the body, **not** in `<head>`. Inline SVG is render-blocking when it sits early, so parking ~90KB of symbol definitions up top makes the browser chew through all of it before it paints a single thing — pay that cost *after* first paint, not before.

### Render an icon

`icon!` is the one you'll reach for most. By default it inherits the surrounding text colour and renders at 24px, so the basic form is just the name:

```rust
use dioxus::prelude::*;
use dioxus_feather_icons::prelude::*;

fn App() -> Element {
  rsx! {
    { icon!(activity) }
  }
}
```

One gotcha worth flagging: icon names are **snake_case** — `alert_circle`, `arrow_up_right` — matching the bundled file names, not Feather's own kebab-case (`alert-circle` won't parse as a Rust identifier, ipso facto won't compile).

Need a specific size, in pixels?

```rust
icon!(alert_circle, 48)
```

Need a size *and* a colour?

```rust
icon!(camera, 32, "#333")
```

### Grab the raw SVG string

Sometimes you don't want the rendered element, you just want the markup — to drop into an attribute, a string, wherever. `icon_str!` hands you the icon's SVG as a plain `&str`:

```rust
let svg: &str = icon_str!(zap);
```

Worth noting: this is still the `<use href="#zap">` reference flavour, so it draws nothing on its own — the sprite from above needs to be on the page, same deal as the macro.

⸻

## 🛠️ Requirements

- **A Dioxus app** — the macros expand into `rsx!`, so they live wherever your Dioxus components do.
- **Rust 2024 edition** — the crate is on the 2024 edition; your project needs to match.
- **The sprite, injected once** — see above. Without it the `<use>` references have nothing to point at, and icons render invisibly with no complaint.

⸻

## 📁 Repo & Contributions

🛠️ **Repo**: [https://github.com/dsplce-co/dioxus-feather-icons](https://github.com/dsplce-co/dioxus-feather-icons)<br>
📦 **Crate**: [https://crates.io/crates/dioxus-feather-icons](https://crates.io/crates/dioxus-feather-icons)

PRs welcome, feel free to contribute 🖤

⸻

## 📄 License

MIT or Apache-2.0, at your option.
