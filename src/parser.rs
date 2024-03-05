use crate::{Bookmark, Category, Header};

/// Split a line by the pipe character
/// # Examples
///
/// ```
/// use sbm::parser;
/// let line = "Rust|Systems programming language|https://www.rust-lang.org/";
/// let parts = parser::split_pipe(line);
/// assert_eq!(parts, vec!["Rust", "Systems programming language", "https://www.rust-lang.org/"]);
/// ```
pub fn split_pipe(line: &str) -> Vec<&str> {
    line.split('|').collect()
}

/// Parse a bookmark from a line
/// # Examples
///
/// ```
/// use sbm::{parser, Bookmark};
/// let line = "Rust|Systems programming language|https://www.rust-lang.org/";
/// let bookmark = parser::parse_bookmark(line).unwrap();
/// assert_eq!(bookmark.name, "Rust");
/// assert_eq!(bookmark.description, "Systems programming language");
/// assert_eq!(bookmark.url, "https://www.rust-lang.org/");
/// ```
pub fn parse_bookmark(line: &str) -> Result<Bookmark, &'static str> {
    let parts = split_pipe(line);
    if parts.len() != 3 {
        return Err("bookmark has wrong number of parts");
    }
    Ok(Bookmark {
        name: parts[0].trim().to_string(),
        description: parts[1].trim().to_string(),
        url: parts[2].trim().to_string(),
    })
}

/// parse a header from a line
/// # Examples
///
/// ```
/// use sbm::{parser, Header};
/// let line = "Programming Languages|👨‍💻";
/// let header = parser::parse_header(line).unwrap();
/// assert_eq!(header.name, "Programming Languages");
/// assert_eq!(header.icon, Some("👨‍💻".to_string()));
/// ```
pub fn parse_header(line: &str) -> Result<Header, &'static str> {
    let parts = split_pipe(line);
    if parts.len() != 1 && parts.len() != 2 {
        return Err("header has wrong number of parts");
    }
    Ok(Header {
        name: parts[0].trim().to_string(),
        icon: parts.get(1).map(|i| i.trim().to_string()),
    })
}

/// Parse categories from a string
/// # Examples
///
/// ```
/// use sbm::{parser, Category, Header};
/// let data = r#"
/// #Programming Languages
/// Rust|The Rust Programming Language|https://www.rust-lang.org/
/// Python|Python Programming Language|https://www.python.org/
///
/// #Web Development|🌐
/// HTML|Hypertext Markup Language|https://developer.mozilla.org/en-US/docs/Web/HTML
/// CSS|Cascading Style Sheets|https://developer.mozilla.org/en-US/docs/Web/CSS
/// "#;
/// let categories = parser::parse_categories(data).unwrap();
/// assert_eq!(categories.len(), 2);
/// assert_eq!(categories[0].header.name, "Programming Languages");
/// assert_eq!(categories[0].bookmarks.len(), 2);
/// assert_eq!(categories[1].header.name, "Web Development");
/// assert_eq!(categories[1].header.icon, Some("🌐".to_string()));
/// assert_eq!(categories[1].bookmarks.len(), 2);
/// ```
pub fn parse_categories(data: &str) -> Result<Vec<Category>, &'static str> {
    let mut categories = Vec::new();
    let mut current: Option<Category> = None;

    for line in data.lines() {
        if line.starts_with("//") || line.trim().is_empty() {
            continue;
        }

        if let Some(stripped) = line.strip_prefix('#') {
            if let Some(c) = current.take() {
                categories.push(c);
            }
            let header = parse_header(stripped.trim())?;
            current = Some(Category {
                header,
                bookmarks: Vec::new(),
            });
        } else if let Some(c) = current.as_mut() {
            let bookmark = parse_bookmark(line)?;
            c.bookmarks.push(bookmark);
        }
    }
    if let Some(c) = current.take() {
        categories.push(c);
    }

    Ok(categories)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bookmark() {
        let line = "Rust|Systems programming language|https://www.rust-lang.org/";
        let bookmark = parse_bookmark(line).unwrap();
        assert_eq!(bookmark.name, "Rust");
        assert_eq!(bookmark.description, "Systems programming language");
        assert_eq!(bookmark.url, "https://www.rust-lang.org/");
    }

    #[test]
    fn test_parse_header() {
        let line = "Programming Languages";
        let header = parse_header(line).unwrap();
        assert_eq!(header.name, "Programming Languages");
        assert_eq!(header.icon, None);

        let line = "Programming Languages|👨‍💻";
        let header = parse_header(line).unwrap();
        assert_eq!(header.name, "Programming Languages");
        assert_eq!(header.icon, Some("👨‍💻".to_string()));
    }

    #[test]
    fn test_parse_categories() {
        let data = r#"
#Programming Languages
Rust|The Rust Programming Language|https://www.rust-lang.org/
Python|Python Programming Language|https://www.python.org/
// This is a comment
#Web Development|🌐
HTML|Hypertext Markup Language|https://developer.mozilla.org/en-US/docs/Web/HTML
CSS|Cascading Style Sheets|https://developer.mozilla.org/en-US/docs/Web/CSS
"#;
        let categories = parse_categories(data).unwrap();
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].header.name, "Programming Languages");
        assert_eq!(categories[0].bookmarks.len(), 2);
        assert_eq!(categories[1].header.name, "Web Development");
        assert_eq!(categories[1].header.icon, Some("🌐".to_string()));
        assert_eq!(categories[1].bookmarks.len(), 2);
    }

    #[test]
    fn test_bad_bookmark() {
        let line = "Rust|Systems programming language";
        let bookmark = parse_bookmark(line);
        assert!(bookmark.is_err());
    }

    #[test]
    fn test_bad_header() {
        let line = "Programming Languages|👨‍💻|Extra";
        let header = parse_header(line);
        assert!(header.is_err());
    }
}
