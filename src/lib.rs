const MM_CREV_ID : &'static str = "6OZqHXqyUAF57grEY7IVMjRljdd9dgDxiNtr1NF1BdY";

use std::collections::{BTreeMap};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path};
use std::process::Command;
use std::sync::Arc;

pub type Category = Arc<String>;

#[derive(Clone, Debug)]
pub struct Crate {
    pub name:           String,
    pub category:       Category,
    pub description:    String,
    pub crev:           String,
}

impl Crate {
    pub fn from_review_md(reviews: &Reviews, review_md: &Path) -> Self {
        Self::new(reviews, review_md.file_stem().unwrap().to_str().unwrap(), review_md)
    }

    fn new(reviews: &Reviews, name: &str, review_md: &Path) -> Self {
        let mut category = None;
        let mut crev = "none".to_string();
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
                        crev = v.trim().to_string();
                    } else if let Some(v) = prefixed(line, "description:") {
                        description = Some(v.trim().to_string());
                    }
                }
            }
        }

        if crev == "none" {
            crev = reviews.by_name.get(name)
                .and_then(|r| r.iter().filter(|r| r.from_id == MM_CREV_ID).last())
                .map(|r| r.review_rating.clone())
                .unwrap_or_else(|| "none".into());
        }

        Self {
            name:           name.to_string(),
            category:       Category::new(category.unwrap_or(String::from("Uncategorized"))),
            description:    description.unwrap_or(String::new()),
            crev,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Crates {
    pub by_category: BTreeMap<Category, BTreeMap<String, Crate>>,
}

impl Crates {
    pub fn from_dir(dir: &(impl ?Sized + AsRef<Path>)) -> io::Result<Self> {
        let reviews = Reviews::find();
        let mut crates = Self::default();

        for e in fs::read_dir(dir)? {
            let c = Crate::from_review_md(&reviews, e?.path().as_ref());
            crates.by_category.entry(c.category.clone()).or_default().insert(c.name.clone(), c);
            let _ = std::io::stdout().flush();
        }

        Ok(crates)
    }
}

#[derive(Default)]
struct ReviewBuilder<'s> {
    pub section:            Option<&'s str>,
    pub from_id:            Option<String>,
    pub package_name:       Option<String>,
    pub package_version:    Option<String>,
    pub review_rating:      Option<String>,
    pub review_version:     Option<String>,
}

impl ReviewBuilder<'_> {
    pub fn build(self) -> Option<Review> {
        Some(Review{
            from_id:            self.from_id?,
            package_name:       self.package_name?,
            package_version:    self.package_version?,
            review_rating:      self.review_rating?,
        })
    }
}

pub struct Review {
    pub from_id:            String,
    pub package_name:       String,
    pub package_version:    String,
    pub review_rating:      String,
}

fn cargo_crev_reviews() -> Vec<Review> {
    let reviews = Command::new("cargo").args(&["crev", "proof", "find"]).output().unwrap();
    let stdout = String::from_utf8_lossy(&reviews.stdout[..]);
    let mut reviews = Vec::new();
    let mut next_review = ReviewBuilder::default();
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed == "---" {
            reviews.extend(std::mem::take(&mut next_review).build());
        } else if trimmed == "" {
            continue;
        } else if !line.starts_with("  ") {
            if line.ends_with(":") {
                next_review.section = Some(line);
            } else {
                next_review.section = None;
            }
        } else if next_review.section == Some("from:") {
            if let Some(v) = prefixed(line, "  id: ") { next_review.from_id = Some(v.to_string()); }
        } else if next_review.section == Some("package:") {
            if      let Some(v) = prefixed(line, "  name: "   ) { next_review.package_name = Some(v.to_string()); }
            else if let Some(v) = prefixed(line, "  version: ") { next_review.package_version = Some(v.to_string()); }
        } else if next_review.section == Some("review:") {
            if let Some(v) = prefixed(line, "  rating: ") {
                next_review.review_rating = Some(v.into());
            }
        }
    }
    reviews.extend(std::mem::take(&mut next_review).build());
    reviews
}

pub struct Reviews {
    pub by_name: BTreeMap<String, Vec<Review>>,
}
impl Reviews {
    pub fn find() -> Self {
        let mut by_name = BTreeMap::<String, Vec<Review>>::new();
        for review in cargo_crev_reviews() {
            by_name.entry(review.package_name.clone()).or_default().extend(Some(review));
        }
        Self { by_name }
    }
}

fn prefixed<'a>(line: &'a str, prefix: &str) -> Option<&'a str> {
    if line.starts_with(prefix) {
        Some(&line[prefix.len()..])
    } else {
        None
    }
}
