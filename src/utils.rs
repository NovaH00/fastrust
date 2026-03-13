/// Normalize an HTTP path by removing trailing slashes.
///
/// This function trims all trailing `/` characters from the provided path
/// while preserving the root path (`"/"`). If trimming would result in an
/// empty string (which happens when the input is `"/"` or consists only of
/// slashes), the function returns `"/"`.
///
/// This is useful when normalizing routes so that paths like `/api/`,
/// `/api//`, and `/api///` are treated the same as `/api`.
///
/// # Examples
///
/// ```
/// assert_eq!(normalize_path("/api///"), "/api");
/// assert_eq!(normalize_path("/api/"), "/api");
/// assert_eq!(normalize_path("/api"), "/api");
/// assert_eq!(normalize_path("/"), "/");
/// assert_eq!(normalize_path("///"), "/");
/// ```
///
/// # Notes
///
/// - Only trailing slashes are removed.
/// - Internal slashes (e.g., `/api//users`) are left unchanged.
/// - The returned value borrows from the input and does not allocate.
pub fn normalize_path(path: &str) -> &str {
    let trimmed = path.trim_end_matches('/');
    if trimmed.is_empty() {
        "/"
    } else {
        trimmed
    }
}
