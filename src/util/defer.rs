//!
//! - Example:
//! ```rust
//! use bwserver::defer;
//! #[test]
//! fn test_defer() {
//!
//!     println!("defer 1");
//!
//!     defer!(println!("defer 2"));
//!
//!    {
//!         defer!(println!("defer 3"));
//!         defer! {
//!             println!("defer 4");
//!         }
//!     }
//!
//!     defer!(println!("defer 5"));
//!
//!     defer! {}
//!     println!("defer 6");
//! }
//! ```
//! - Output:
//! ```text
//! defer 1
//! defer 4
//! defer 3
//! defer 6
//! defer 5
//! defer 2
//! ```
//!

#[macro_export]
macro_rules! defer {
    ($($body: tt)*) => {
        let _defer = {
            pub struct Defer<F: FnOnce()>(Option<F>);
            impl<F: FnOnce()> Drop for Defer<F> {
                fn drop(&mut self) {
                    self.0.take().map(|f| f());
                }
            }
            Defer(Some(|| { $($body)* }))
        };
    };
    ($($e:expr)*) => {
        let _defer = {
            pub struct Defer<F: FnOnce()>(Option<F>);
            impl<F: FnOnce()> Drop for Defer<F> {
                fn drop(&mut self) {
                    self.0.take().map(|f| f());
                }
            }
            Defer(Some(|| { $($e)* }))
        };
    };
    () => {};
}
