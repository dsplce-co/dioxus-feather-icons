use std::{env, fs, path::Path};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let icons_dir = format!("{}/assets/icons", manifest_dir);

    let macro_code = format!(r#"
        #[macro_export]
        macro_rules! icon_str {{
            ($name:ident) => {{
                include_str!(concat!(
                    "{icons_dir}",
                    "/",
                    stringify!($name),
                    ".svg"
                ))
            }};
        }}

        #[macro_export]
        macro_rules! sprite_inline {{
            () => {{
                include_str!(concat!(
                    "{icons_dir}",
                    "/",
                    "_sprite.svg"
                ))
            }};
        }}
    "#, icons_dir = icons_dir);

    let out_dir = env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("icon_macro.rs");
    fs::write(&path, macro_code).unwrap();

    println!("cargo:rerun-if-changed=assets/icons");
}
