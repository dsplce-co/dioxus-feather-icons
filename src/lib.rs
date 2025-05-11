include!(concat!(env!("OUT_DIR"), "/icon_macro.rs"));

pub mod prelude;

#[macro_export]
macro_rules! icon {
    ($name:ident, $size:expr, $color:expr) => {
        rsx! {
            i {
                style: format!("--size:{}px;color:{}", $size, $color),
                dangerous_inner_html: $crate::icon_str!($name)
            }
        }
    };
    ($name:ident, $size:expr) => {
        $crate::icon!($name, $size, "currentColor")
    };
    ($name:ident) => {
        $crate::icon!($name, 24)
    };
}

#[macro_export]
macro_rules! sprite {
    () => {
        rsx! {
            div {
                dangerous_inner_html: $crate::sprite_inline!()
            }
        }
    };
}
