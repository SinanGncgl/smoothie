pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5 && email.len() < 255
}

pub fn validate_profile_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 255 && !name.contains('\0')
}

pub fn validate_resolution(resolution: &str) -> bool {
    let parts: Vec<&str> = resolution.split('x').collect();
    if parts.len() != 2 {
        return false;
    }
    
    match (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
        (Ok(w), Ok(h)) => w > 0 && h > 0 && w <= 8192 && h <= 4320,
        _ => false,
    }
}

pub fn validate_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

pub fn validate_bundle_id(bundle_id: &str) -> bool {
    !bundle_id.is_empty() && bundle_id.len() <= 255 && bundle_id.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_')
}

pub fn validate_monitor_bounds(x: i32, y: i32, width: i32, height: i32) -> bool {
    width > 0 && height > 0 && x >= -8192 && y >= -4320 && (x + width) <= 16384 && (y + height) <= 8640
}
