use percent_encoding::{AsciiSet, CONTROLS};

pub use percent_encoding::utf8_percent_encode;

// from https://github.com/servo/rust-url/blob/master/url/src/parser.rs
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
const PATH: &AsciiSet = &FRAGMENT.add(b'#').add(b'?').add(b'{').add(b'}');
pub const PATH_SEGMENT: &AsciiSet = &PATH.add(b'/').add(b'%');
