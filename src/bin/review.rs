use std::time::*;
use std::io::Write;
use std::path::*;
use std::process::{Command, exit};

enum Mode {
    Stable,
    All,
    Version(String),
}

fn usage() {
    eprintln!("Usage:");
    eprintln!("cargo review [crate] [version]");
    eprintln!();
    eprintln!("version:");
    eprintln!("    --one | --latest | --stable      Latest stable version");
    eprintln!("    * | --all                        All stable versions");
    eprintln!("    0.0.3-beta                       A specific version, stable or otherwise");
    eprintln!();
}

fn main() {
    let mut args = std::env::args();
    let _exe = args.next();

    let mut krate : Option<String> = None;
    let mut mode : Option<Mode> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--one"     => { assert!(mode.is_none()); mode = Some(Mode::Stable); },
            "--latest"  => { assert!(mode.is_none()); mode = Some(Mode::Stable); },
            "--stable"  => { assert!(mode.is_none()); mode = Some(Mode::Stable); },
            "--all"     => { assert!(mode.is_none()); mode = Some(Mode::All); },
            "*"         => { assert!(mode.is_none()); mode = Some(Mode::All); },
            // --range?
            flag if flag.starts_with("--") => { eprintln!("Unrecognized flag: {}", flag); usage(); exit(1); },
            _ => {
                if krate.is_none() {
                    krate = Some(arg);
                } else if mode.is_none() {
                    if arg == "*" {
                        mode = Some(Mode::All);
                    } else {
                        mode = Some(Mode::Version(arg));
                    }
                } else {
                    usage();
                    exit(1);
                }
            }
        }
    }

    let krate = krate.unwrap_or_else(|| { usage(); exit(1); });
    let mode  = mode .unwrap_or_else(|| { usage(); exit(1); });

    let temp = PathBuf::from(std::env::var_os("TEMP").expect("%TEMP% not set"));
    let registry_src = PathBuf::from(std::env::var_os("USERPROFILE").expect("%USERPROFILE% not set")).join(r".cargo\registry\src\github.com-1ecc6299db9ec823");

    // NOTE: cargo seems to use some kind of semi-binary format now:~\.cargo\registry\index\github.com-1ecc6299db9ec823\.cache
    //       crates_index OTOH uses the raw textual format... so it needs to update
    let index = crates_index::Index::new_cargo_default();
    let now = SystemTime::now();
    let index_mod = index.path().join(".git").join("index").metadata().and_then(|md| md.modified());
    if !index.exists() {
        println!("Cloning index (this will take awhile...)");
        index.retrieve().unwrap();
    } else if index_mod.map_or(true, |index_mod| index_mod <= now - Duration::from_secs(120)) {
        println!("Updating index (this may take awhile...)");
        index.update().unwrap();
    }
    let krate = index.crate_(&krate).unwrap_or_else(|| { eprintln!("No such crate {:?}", krate); exit(1); });

    let mut versions = Vec::new();
    match mode {
        Mode::Stable    => versions.extend(krate.highest_stable_version().map(|v| v.to_string())),
        Mode::All       => versions.extend(krate.versions().iter().filter(|v| !v.is_yanked()).map(|v| v.version().to_string())),
        Mode::Version(v)=> versions.extend(Some(v)),
    }

    let review = temp.join("rust-reviews-fetch");
    let _ = std::fs::create_dir(&review);
    let _ = std::fs::write(review.join("lib.rs"), "");
    let review_toml = review.join("Cargo.toml");

    for version in &versions {
        let mut o = std::fs::File::create(&review_toml).expect("Unable to create %TEMP%/rust-reviews-fetch/Cargo.toml");
        let _ = writeln!(o, "workspace.members = [{:?}]", ".");
        let _ = writeln!(o, "package.name    = {:?}", "rust-reviews-fetch");
        let _ = writeln!(o, "package.version = {:?}", "0.0.0");
        let _ = writeln!(o, "lib.path = {:?}", "lib.rs");
        let _ = writeln!(o, "[dependencies]");
        let _ = writeln!(o, "{:?} = {:?}", krate.name(), format!("={}", version));
        std::mem::drop(o);

        print!("Fetching {}...", version);
        let _ = std::io::stdout().flush();
        Command::new("cargo").arg("fetch").current_dir(&review).status().unwrap();
    }

    let old_cwd = std::env::current_dir().expect("CWD");
    let data = mustache::MapBuilder::new()
        .insert_map("crate", |c|{c
            .insert_str("name", krate.name())
        })
        .insert_vec("versions", |mut v| {
            for version in versions.iter().rev() {
                let src = registry_src.join(&format!("{}-{}", krate.name(), version));
                std::env::set_current_dir(&src).expect("set CWD"); // evil
                v = v.push_map(|m|{m
                    .insert_str("version", version)
                    .insert_str("local_path", src.to_string_lossy().into_owned())
                    .insert_vec("files", |mut f|{
                        for file in glob::glob("**/*").unwrap() {
                            let file = file.unwrap();
                            f = f.push_map(|f|{
                                f.insert_str("name", &format!("{:33}", file.display()))
                            });
                        }
                        f
                    })
                });
            }
            v
        })
        .build();
    std::env::set_current_dir(old_cwd).expect("set CWD"); // restore

    let template = mustache::compile_str(include_str!("review.template.md")).expect("Unable to compile review.template.md");
    let mut review_md = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!("reviews/{}.md", krate.name()))
        .expect("Unable to open reviews/crate.md");
    template.render_data(&mut review_md, &data).expect("Unable to render review.template.md");
}
