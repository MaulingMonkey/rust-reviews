use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path};
use std::process::Command;

fn main() {
    let mut readme = File::create("README.md").unwrap();
    write_readme_md(&mut readme).unwrap();
}

fn write_readme_md(md: &mut File) -> io::Result<()> {
    let placeholder = "{{CRATES}}";
    let template = include_str!("../README.template.md");
    let i = template.find(placeholder).unwrap();
    let (pre, post) = template.split_at(i);
    let post = &post[placeholder.len()..];

    write!(md, "{}", pre)?;

    writeln!(md, "| Crate | Review | Description |")?;
    writeln!(md, "| ----- | ------ | ----------- |")?;
    for e in fs::read_dir("reviews")? {
        let c = Crate::from_review_md(e?.path().as_ref());
        write!(md, "| [{name}](https://crates.io/crates/{name}) ", name=&c.name)?;
        write!(md, "| ")?;
        for badge in c.badges.iter() { write!(md, "{} ", badge)?; }
        write!(md, "| {}", c.description.replace("\r\n", "<br>").replace("\n", "<br>"))?;
        writeln!(md)?;
    }

    write!(md, "{}", post)?;
    Ok(())
}

#[derive(Clone, Debug)]
struct Crate {
    pub name:       String,
    pub category:   String,
    pub description:       String,
    pub badges:     Vec<String>,
}

impl Crate {
    pub fn from_review_md(review_md: &Path) -> Self {
        Self::new(review_md.file_stem().unwrap().to_str().unwrap(), review_md)
    }

    fn new(name: &str, review_md: &Path) -> Self {
        let mut category = None;
        let mut crev = None;
        let mut description = None;
        
        {
            let review_md = BufReader::new(File::open(review_md).unwrap());
            let mut review_md = review_md.lines();
            if review_md.next().map(|l| l.unwrap() == "---").unwrap_or(false) {
                for line in review_md {
                    let line = line.unwrap();
                    let line = line.as_ref();
                    if line == "---" { break; }
                    if let Some(v) = prefixed(line, "category:") {
                        category = Some(v.trim().to_string());
                    } else if let Some(v) = prefixed(line, "crev:") {
                        crev = Some(v.trim().to_string());
                    } else if let Some(v) = prefixed(line, "description:") {
                        description = Some(v.trim().to_string());
                    }
                }
            }
        }

        if crev.is_none() {
            crev = query_crev_rating(name);
        }

        let mut badges = vec![];
        if let Some(crev) = crev {
            badges.push(format!("[![crev-{rating}]](reviews/{name}.md)", rating=crev, name=name));
        }

        Self {
            name:           name.to_string(),
            category:       category.unwrap_or(String::new()),
            description:    description.unwrap_or(String::new()),
            badges,
        }
    }
}

fn query_crev_rating(name: &str) -> Option<String> {
    let mut cmd = Command::new("cargo");
    cmd.args(&["crev", "query", "review", name]);
    let crev_review = cmd.output().unwrap();

    let mut review = None;
    let mut _review_version = None;

    let mut section = None;
    let mut from_id = None;
    let mut _package_name = None;
    let mut package_version = None;
    let stdout = String::from_utf8_lossy(&crev_review.stdout[..]);
    for line in stdout.lines() {
        if line.trim() == "" {
            continue;
        } else if !line.starts_with("  ") {
            if line.ends_with(":") {
                section = Some(line);
                if line == "package:" {
                    _package_name = None;
                    package_version = None;
                }
            } else {
                section = None;
            }
        } else if section == Some("from:") {
            if let Some(v) = prefixed(line, "  id: ") { from_id = Some(v); }
        } else if section == Some("package:") {
            if      let Some(v) = prefixed(line, "  name: "   ) { _package_name = Some(v); }
            else if let Some(v) = prefixed(line, "  version: ") { package_version = Some(v); }
        } else if section == Some("review:") && from_id == Some("6OZqHXqyUAF57grEY7IVMjRljdd9dgDxiNtr1NF1BdY") {
            if let Some(v) = prefixed(line, "  rating: ") {
                review = Some(v.into());
                _review_version = package_version.clone();
            }
        }
    }

    review
}

fn prefixed<'a>(line: &'a str, prefix: &str) -> Option<&'a str> {
    if line.starts_with(prefix) {
        Some(&line[prefix.len()..])
    } else {
        None
    }
}
