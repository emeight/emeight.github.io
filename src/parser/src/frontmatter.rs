//! Split an optional `---` front matter header off the top of a document.

use std::collections::HashMap;

pub struct Frontmatter {
    fields: HashMap<String, String>,
}

impl Frontmatter {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields.get(key).map(String::as_str)
    }

    pub fn require(&self, key: &str) -> Result<&str, String> {
        self.get(key)
            .ok_or_else(|| format!("missing frontmatter field `{key}`"))
    }
}

/// Returns `(frontmatter, body)`. If `src` does not open with a `---` fence the
/// frontmatter is empty and the body is the whole input.
pub fn split(src: &str) -> (Frontmatter, &str) {
    let mut fields = HashMap::new();

    let rest = match src
        .strip_prefix("---\n")
        .or_else(|| src.strip_prefix("---\r\n"))
    {
        Some(r) => r,
        None => return (Frontmatter { fields }, src),
    };

    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        offset += line.len();
        let line = line.trim_end();
        if line == "---" {
            let body = rest[offset..].trim_start_matches(['\n', '\r']);
            return (Frontmatter { fields }, body);
        }
        if let Some((k, v)) = line.split_once(':') {
            let v = v.trim().trim_matches(['"', '\'']);
            fields.insert(k.trim().to_string(), v.to_string());
        }
    }

    // no closing fence: treat the whole input as body
    (Frontmatter { fields: HashMap::new() }, src)
}