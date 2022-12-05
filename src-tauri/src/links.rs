use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct Link {
    pub url: String,
    pub name: String,
    pub desc: String,
}

impl Link {
    pub fn new(url: String, name: String, desc: String) -> Self {
        Self { url, name, desc }
    }

    pub fn is_valid(&self) -> bool {
        !self.url.is_empty()
    }
}

pub fn parse_links(links: &String) -> Result<Vec<Link>, &str> {
    let mut result = Vec::new();
    for line in links.lines() {
        let link = parse_link_line(line)?;
        result.push(link);
    }
    Ok(result)
}

fn parse_link_line(line: &str) -> Result<Link, &str> {
    let mut parts = line.split(";;");
    let url = parts.next().ok_or("Invalid link line")?;
    let name = parts.next().ok_or("Invalid link line")?;
    let desc = parts.next().ok_or("Invalid link line")?;
    let link = Link::new(url.to_string(), name.to_string(), desc.to_string());
    if link.is_valid() {
        Ok(link)
    } else {
        Err("Invalid link line")
    }
}

mod test {

    use super::*;

    #[test]
    fn test_parse_valid_links() {
        let links = "https://www.example.com;;Example;;Example description\nhttps://www.example2.com;;Example2;;Example2 description".to_string();
        let result = parse_links(&links);
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_invalid_line_format() {
        let links = "https://www.example.com;;Example;;Example description\nhttps://www.example2.com;;Example2".to_string();
        let result = parse_links(&links);
        assert_eq!(result.err(), Some("Invalid link line"));
    }

    #[test]
    fn test_missing_description() {
        let links = "https://www.example.com;;Example;;\nhttps://www.example2.com;;Example2;;Example description2".to_string();
        let mut result = parse_links(&links).unwrap();
        let second = result.pop().unwrap();
        let first = result.pop().unwrap();
        assert_eq!(
            second,
            Link {
                url: "https://www.example2.com".to_string(),
                name: "Example2".to_string(),
                desc: "Example description2".to_string(),
            }
        );
        assert_eq!(
            first,
            Link {
                url: "https://www.example.com".to_string(),
                name: "Example".to_string(),
                desc: "".to_string(),
            }
        );
    }

    #[test]
    fn test_missing_name_and_desc() {
        let links =
            "https://www.example.com;;;;\nhttps://www.example2.com;;Example2;;Example description2"
                .to_string();
        let mut result = parse_links(&links).unwrap();
        let second = result.pop().unwrap();
        let first = result.pop().unwrap();
        assert_eq!(
            second,
            Link {
                url: "https://www.example2.com".to_string(),
                name: "Example2".to_string(),
                desc: "Example description2".to_string(),
            }
        );
        assert_eq!(
            first,
            Link {
                url: "https://www.example.com".to_string(),
                name: "".to_string(),
                desc: "".to_string(),
            }
        );
    }

    #[test]
    fn test_missing_url() {
        let links = ";;Example;;Example description\nhttps://www.example2.com;;Example2;;Example2 description".to_string();
        let result = parse_links(&links);
        assert_eq!(result.err(), Some("Invalid link line"));
    }
}
