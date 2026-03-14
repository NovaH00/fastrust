/// Canonicalize an HTTP path by collapsing duplicate slashes and removing
/// trailing slashes.
///
/// This function normalizes a path into a single canonical representation by:
///
/// - Collapsing consecutive `/` characters into a single `/`
/// - Removing trailing slashes
/// - Preserving the root path (`"/"`)
///
/// If normalization would result in an empty string (which happens when the
/// input contains only slashes), the function returns `"/"`.
///
/// This ensures that logically equivalent paths map to the same canonical
/// value. For example, `/api`, `/api/`, `/api//`, and `/api///` all normalize
/// to `/api`.
///
///
/// # Examples
///
/// ```
/// assert_eq!(canonicalize_path("/api"), "/api");
/// assert_eq!(canonicalize_path("/api/"), "/api");
/// assert_eq!(canonicalize_path("/api///"), "/api");
/// assert_eq!(canonicalize_path("/api//users"), "/api/users");
/// assert_eq!(canonicalize_path("///api///users//"), "/api/users");
/// assert_eq!(canonicalize_path("/"), "/");
/// assert_eq!(canonicalize_path("///"), "/");
/// ```
///
/// # Notes
///
/// - Consecutive slashes are collapsed into a single `/`.
/// - Trailing slashes are removed except for the root path.
/// - The function allocates a new `String` to produce the normalized path.
pub fn canonicalize_path(path: &str) -> String {
    let mut result = String::with_capacity(path.len());
    let mut prev_was_slash = false;

    for c in path.chars() {
        if c == '/' {
            if !prev_was_slash {
                result.push('/');
            }
            prev_was_slash = true;
        } else {
            result.push(c);
            prev_was_slash = false;
        }
    }

    let trimmed = result.trim_end_matches('/');
    if trimmed.is_empty() {
        "/".to_string()
    } else {
        trimmed.to_string()
    }
}
