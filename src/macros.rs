#[cfg(feature = "swizzle")]
macro_rules! swizzle_type {
    ($a:ident, $b:ident) => {
        crate::Vec2
    };
    ($a:ident, $b:ident, $c:ident) => {
        crate::Vec3
    };
    ($a:ident, $b:ident, $c:ident, $d:ident) => {
        crate::Vec4
    };
}

#[cfg(feature = "swizzle")]
macro_rules! swizzle {
    ($($members:ident),+) => {
        paste::paste! {
            #[doc(cfg(feature="swizzle"))] // make docs.rs more readable by showing the required feature
            pub fn [<$($members)+>](&self) -> swizzle_type!($($members),+) {
                <swizzle_type!($($members),+)>::new(
                    $(self.$members),+
                )
            }
        }
    };
}

#[cfg(not(feature = "swizzle"))]
macro_rules! swizzle {
    ($($members:ident),+) => {};
}
