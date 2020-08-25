use rust_reviews::*;

use std::fs::File;
use std::io::{self, Write};

fn main() {
    let mut readme = File::create("README.md").unwrap();
    write_readme_md(&mut readme).unwrap();
}

fn write_readme_md(md: &mut File) -> io::Result<()> {
    let placeholder = "{{CRATES}}";
    let template = include_str!("../../README.template.md");
    let i = template.find(placeholder).unwrap();
    let (pre, post) = template.split_at(i);
    let post = &post[placeholder.len()..];

    let crates = Crates::from_dir("reviews")?;

    write!(md, "{}", pre)?;

    for (category, crates) in crates.by_category.iter() {
        writeln!(md, "## {}", category)?;
        writeln!(md)?;
        writeln!(md, "| Crate | Review | Description |")?;
        writeln!(md, "| ----- | ------ | ----------- |")?;
        for c in crates.iter() {
            write!(md, "| [{name}](https://crates.io/crates/{name}) ", name=&c.name)?;
            write!(md, "| ")?;
            for badge in c.badges.iter() { write!(md, "{} ", badge)?; }
            write!(md, "| {}", c.description.replace("\r\n", "<br>").replace("\n", "<br>"))?;
            writeln!(md)?;
        }
        writeln!(md)?;
    }

    write!(md, "{}", post)?;
    Ok(())
}
