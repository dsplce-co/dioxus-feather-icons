include!(concat!(env!("OUT_DIR"), "/icon_macro.rs"));

pub mod prelude;

#[macro_export]
macro_rules! icon {
    ($name:ident, $color:expr, $size:expr) => {
        rsx! {
            i {
                style: format!("color:{};--size:{}px", $color, $size),
                dangerous_inner_html: $crate::icon_str!($name)
            }
        }
    };
    ($name:ident, $color:expr) => {
        $crate::icon!($name, $color, 24)
    };
    ($name:ident) => {
        $crate::icon!($name, "currentColor")
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
