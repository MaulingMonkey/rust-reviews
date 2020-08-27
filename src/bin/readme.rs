use rust_reviews::*;

use std::fs::File;

fn main() {
    let template = mustache::compile_path("README.template.md").expect("README.template.md");
    let crates = Crates::from_dir("reviews").expect("Unable to read review");

    let data = mustache::MapBuilder::new()
        .insert_vec("categories", |v| crates.by_category.iter().fold(v, |v, (category, crates)| { v
            .push_map(|m| { m
                .insert_str("category", &**category)
                .insert_vec("crates", |v| crates.iter().fold(v, |v, krate| { v
                    .push_map(|m| { m
                        .insert_str("crate", &krate.name)
                        .insert_str("badges", krate.badges.join(", ").to_string())
                        .insert_str("description", &krate.description)
                    })
                }))
            })
        }))
        .build();

    let mut md = File::create("README.md").expect("Failed to create/open README.md for writing");
    template.render_data(&mut md, &data).expect("README.template.md rendering error");
}
