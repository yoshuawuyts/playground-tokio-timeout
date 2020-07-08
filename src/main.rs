//! Testing whether Tokio doesn't display timeouts correctly on Windows
//!
//! # Examples
//!
//! ```
//! // tbi
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

use tokio::net::TcpStream;
use std::time::Duration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("104.198.14.52:80").await?;
    println!("{:?}", stream.keepalive()?);

    stream.set_keepalive(Some(Duration::from_secs(1)))?;
    println!("{:?}", stream.keepalive()?);
    Ok(())
}
