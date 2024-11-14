#![no_std]

/// This macro will tigger a linker error if it can't be optimized out
/// ```rust
/// use unreachable_checked::unreachable_checked;
///
/// let a = [1, 2, 3, 4];
/// for i in 0..4 {
///    println!("{}", a.get(i).unwrap_or_else(|| unreachable_checked!()))
/// }
/// ```
#[macro_export]
macro_rules! unreachable_checked {
    () => {{
        extern "Rust" {
            #[link_name = concat!(
                    "\n\n\n",
                    "ERROR: unreachable_checked!() not optimized out at ",
                    file!(),
                    ":",
                    line!(),
                    ":",
                    column!(),
                    "\n\n\n")]
            fn check() -> !;
        }
        unsafe { check() };
    }};
}
