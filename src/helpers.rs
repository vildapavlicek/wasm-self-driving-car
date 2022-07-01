use std::str::FromStr;
use tap::prelude::*;

pub fn parse<T: FromStr>(v: String) -> Option<T> {
    v.parse::<T>()
        .tap_err(|_| tracing::error!(%v, "failed to parse"))
        .ok()
}
