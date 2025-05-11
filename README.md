# dioxus-feather-icons

**Feather Icons for Dioxus** — Inline SVG icons packaged for seamless use in [Dioxus](https://dioxuslabs.com/) apps. This crate provides ergonomic macros to embed [Feather Icons](https://feathericons.com/) in your UI with no runtime cost.

---

## ✨ Features

- Render Feather Icons as inline SVG
- Lightweight and dependency-free
- Simple macro-based usage with `rsx!`
- Optimized for performance via SVG sprite injection

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus-feather-icons = "0.1.1"
```
This crate requires Rust 2024 edition.

⸻

## 🗂️ Icons Included

The crate includes a curated set of Feather .svg files and a bundled _sprite.svg. You do not need to download or manage the icons yourself.

⸻

## 🚀 Usage

### 1. Add the Sprite

To make icons render, you must inject the sprite once in your component tree, ideally toward the end of the body to avoid blocking first paint:
```rust
rsx! {
  ...
  dioxus_feather_icons::sprite!()
}
```

Avoid injecting it into `<head>` — inline SVGs are render-blocking when placed early.

⸻

### 2. Render an Icon

Use the `icon!` macro to render a Feather icon inline. It supports progressively more customization:

**✅ Basic usage (inherits text color, 24px size by default):**
```rust 
use dioxus::prelude::*;
use dioxus_feather_icons::prelude::*;

fn App() -> Element {
  rsx! {
    { icon!(activity) }
  }
}
```

**🎨 Set a specific color:**

```rust 
icon!(alert_circle, "red")
```

**📐 Customize color and size:**

```rust
icon!(camera, "#333", 32)
```

### 3. Access Raw SVG String (if needed)

```rust
let svg: &str = icon_str!(zap);
```

⸻

## 🔒 License

MIT or Apache-2.0, at your option.

⸻
