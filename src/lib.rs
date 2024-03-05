pub mod parser;

/// Bookmark
///
/// A bookmark is a link to a website with a name and a description
#[derive(Debug, PartialEq, Clone)]
pub struct Bookmark {
    pub name: String,
    pub description: String,
    pub url: String,
}

impl Bookmark {
    pub fn new(name: &str, description: &str, url: &str) -> Bookmark {
        Bookmark {
            name: name.to_string(),
            description: description.to_string(),
            url: url.to_string(),
        }
    }
}

impl std::fmt::Display for Bookmark {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}|{}|{}", self.name, self.description, self.url)
    }
}

/// Category Header
///
/// A header is a name and an optional icon
#[derive(Debug, PartialEq, Clone)]
pub struct Header {
    pub name: String,
    pub icon: Option<String>,
}

impl Header {
    pub fn new(name: &str, icon: Option<&str>) -> Header {
        Header {
            name: name.to_string(),
            icon: icon.map(|s| s.to_string()),
        }
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.icon {
            Some(icon) => write!(f, "#{}|{}", self.name, icon),
            None => write!(f, "#{}", self.name),
        }
    }
}

/// Category
///
/// A category is a header with a list of bookmarks
#[derive(Debug, PartialEq, Clone)]
pub struct Category {
    pub header: Header,
    pub bookmarks: Vec<Bookmark>,
}

impl Category {
    pub fn new(header: Header) -> Category {
        Category {
            header,
            bookmarks: Vec::new(),
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.header,
            self.bookmarks
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Sbm(Vec<Category>);

impl Sbm {
    pub fn new(categories: Vec<Category>) -> Sbm {
        Sbm(categories)
    }
}

impl std::fmt::Display for Sbm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bookmark_new() {
        let bookmark = Bookmark::new(
            "Rust",
            "Systems programming language",
            "https://www.rust-lang.org/",
        );
        assert_eq!(bookmark.name, "Rust");
        assert_eq!(bookmark.description, "Systems programming language");
        assert_eq!(bookmark.url, "https://www.rust-lang.org/");
    }

    #[test]
    fn test_header_new() {
        let header = Header::new("Programming Languages", None);
        assert_eq!(header.name, "Programming Languages");
        assert_eq!(header.icon, None);

        let header = Header::new("Programming Languages", Some("👨‍💻"));
        assert_eq!(header.name, "Programming Languages");
        assert_eq!(header.icon, Some("👨‍💻".to_string()));
    }

    #[test]
    fn test_category_new() {
        let header = Header::new("Programming Languages", None);
        let category = Category::new(header);
        assert_eq!(category.header.name, "Programming Languages");
        assert_eq!(category.header.icon, None);
        assert_eq!(category.bookmarks.len(), 0);
    }

    #[test]
    fn test_sbm_new() {
        let header = Header::new("Programming Languages", None);
        let category = Category::new(header);
        let sbm = Sbm::new(vec![category]);
        assert_eq!(sbm.0.len(), 1);
    }

    #[test]
    fn test_sbm_display() {
        let sbm = Sbm(vec![
            Category {
                header: Header::new("Programming Languages", None),
                bookmarks: vec![Bookmark::new(
                    "Rust",
                    "Systems programming language",
                    "https://www.rust-lang.org/",
                )],
            },
            Category {
                header: Header::new("Web Development", Some("🌐")),
                bookmarks: vec![Bookmark::new(
                    "MDN",
                    "Web documentation",
                    "https://developer.mozilla.org/",
                )],
            },
        ]);
        assert_eq!(
            sbm.to_string(),
            "#Programming Languages\nRust|Systems programming language|https://www.rust-lang.org/\n#Web Development|🌐\nMDN|Web documentation|https://developer.mozilla.org/"
        );
    }
}
