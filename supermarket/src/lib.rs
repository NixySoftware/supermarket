mod client;
pub mod credentials;
pub mod internal;

pub use client::Client;
pub use client::Identifier;
pub use internal::ClientError;

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
