use sha1::{ Sha1, Digest };
use std::path::Path;

pub fn is_file_valid(path: &Path, sha1: &str) -> std::io::Result<bool> {
    let bytes = std::fs::read(path)?;
    let hash = Sha1::digest(&bytes);

    if parse_hex_pairs(sha1).eq(hash.into_iter()) {     
        return Ok(true)
    }

    Ok(false)
}

fn parse_digest_digit(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        _ => None,
    }
}

fn parse_hex_pairs(s: &str) -> impl Iterator<Item = u8> + '_ {
    let mut bytes = s.as_bytes();
    core::iter::from_fn(move || {
        if let [b1, b2, rest @ ..] = bytes {
            bytes = rest;
            Some(parse_digest_digit(*b1)? * 16 + parse_digest_digit(*b2)?)
        } else {
            None
        }
    })
}