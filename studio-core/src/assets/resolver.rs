// src/assets/resolver.rs

use crate::config::{HTTP_FETCH_TIMEOUT, MAX_IMAGE_BYTES};
use base64::{engine::general_purpose, Engine as _};
use std::io::Read;
use std::net::{IpAddr, ToSocketAddrs};
use typst::foundations::Bytes;

pub fn resolve_image_source(src: &str) -> Result<Bytes, String> {
    if let Some(rest) = src.strip_prefix("data:") {
        let payload = rest.split(',').nth(1).ok_or("malformed data URI")?;
        return Ok(Bytes::new(
            general_purpose::STANDARD
                .decode(payload.trim())
                .map_err(|e| e.to_string())?,
        ));
    }
    if src.starts_with("http://") || src.starts_with("https://") {
        return fetch_remote_image(src);
    }
    Ok(Bytes::new(
        general_purpose::STANDARD
            .decode(src.trim())
            .map_err(|_| "Invalid base64".to_string())?,
    ))
}

fn fetch_remote_image(src: &str) -> Result<Bytes, String> {
    let allowed_hosts = std::env::var("ALLOWED_IMAGE_HOSTS").unwrap_or_default();
    let allowed: Vec<&str> = allowed_hosts
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if allowed.is_empty() {
        return Err("Remote image fetching disabled".to_string());
    }

    let url = reqwest::Url::parse(src).map_err(|e| e.to_string())?;
    if url.scheme() != "https" {
        return Err("Only HTTPS allowed".to_string());
    }
    let host = url.host_str().ok_or("No host")?;
    if !allowed.iter().any(|h| *h == host) {
        return Err(format!("Host {} not allowlisted", host));
    }

    let port = url.port_or_known_default().unwrap_or(443);
    for addr in (host, port).to_socket_addrs().map_err(|e| e.to_string())? {
        if !is_public_ip(&addr.ip()) {
            return Err("Resolves to private IP".to_string());
        }
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(HTTP_FETCH_TIMEOUT)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.to_string())?;
    let response = client.get(url).send().map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err("HTTP error".to_string());
    }

    let mut limited = response.take(MAX_IMAGE_BYTES as u64 + 1);
    let mut buf = Vec::new();
    limited.read_to_end(&mut buf).map_err(|e| e.to_string())?;
    if buf.len() > MAX_IMAGE_BYTES {
        return Err("Image too large".to_string());
    }
    Ok(Bytes::new(buf))
}

fn is_public_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            !(v4.is_loopback()
                || v4.is_private()
                || v4.is_link_local()
                || v4.is_unspecified()
                || v4.is_multicast()
                || v4.is_broadcast())
        }
        IpAddr::V6(v6) => {
            if v6.is_loopback() || v6.is_unspecified() || v6.is_multicast() {
                return false;
            }
            let seg = v6.segments();
            if (seg[0] & 0xffc0) == 0xfe80 || (seg[0] & 0xfe00) == 0xfc00 {
                return false;
            }
            true
        }
    }
}

pub fn sniff_image_format(bytes: &[u8]) -> &'static str {
    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
        "png"
    } else if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        "jpg"
    } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        "gif"
    } else if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        "webp"
    } else {
        "png"
    }
}
