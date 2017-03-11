/// Generate a timestamp representing now (UTC) in RFC3339 format.
pub fn now() -> &'static str {
    "2017-03-10T20:10:46Z"
}

/// Generate a timstamp string representing now (UTC).
pub fn short_now() -> &'static str {
    "2017-03-10"
}

/// Generate a SHA string
pub fn sha() -> &'static str {
    "eb49ff34f1fa5aa032d71573cb91101f315c1a34"
}

/// Generate a short SHA string
pub fn short_sha() -> &'static str {
    "eb49ff3"
}

/// Generate the commit date string
pub fn commit_date() -> &'static str {
    "2017-03-06"
}

/// Generate the target triple string
pub fn target() -> &'static str {
    "arm-unknown-linux-gnueabihf"
}

/// Generate a semver string
pub fn semver() -> &'static str {
    ""
}
