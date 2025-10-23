use std::str::FromStr;

use anyhow::bail;
use serde::Serialize;

/// A wrapper that indicates a value should be interpreted as a delta
/// (relative change).
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Delta<T> {
  pub inner: T,
  pub is_negative: bool,
}

impl<T: FromStr<Err = anyhow::Error>> FromStr for Delta<T> {
  type Err = anyhow::Error;

  fn from_str(unparsed: &str) -> anyhow::Result<Self> {
    let unparsed = unparsed.trim();

    let (raw, is_negative) = match unparsed.chars().next() {
      Some('+') => (&unparsed[1..], false),
      Some('-') => (&unparsed[1..], true),
      // No sign means positive.
      _ => (unparsed, false),
    };

    if raw.is_empty() {
      bail!("Empty value.");
    }

    let inner = T::from_str(raw)?;

    Ok(Self { inner, is_negative })
  }
}
