use std::process::{Command, exit};
use std::path::PathBuf;

fn main() {
    let mut args = std::env::args();
    let _exe = args.next();
    let krate = args.next().unwrap_or_else(|| { eprintln!("Usage: cargo diff [crate] [version]"); exit(1); });
    let vers  = args.next().unwrap_or_else(|| { eprintln!("Usage: cargo diff [crate] [version]"); exit(1); });

    // NOTE: cargo seems to use some kind of semi-binary format now:~\.cargo\registry\index\github.com-1ecc6299db9ec823\.cache
    //       crates_index OTOH uses the raw textual format... so it needs to update
    let index = crates_index::Index::new_cargo_default();
    let krate = index.crate_(&krate).unwrap_or_else(|| { eprintln!("No such crate {:?}", krate); exit(1); });

    let mut registry_src = PathBuf::from(index.path());
    registry_src.pop(); // github.com-1ecc6299db9ec823
    registry_src.pop(); // index
    registry_src.push("src");
    registry_src.push("github.com-1ecc6299db9ec823");

    let mut prev : Option<String> = None;
    for version in krate.versions().iter().filter(|v| !v.is_yanked()).map(|v| v.version().to_string()) {
        if version == vers {
            if let Some(prev) = prev {
                eprintln!("diffing {} => {}", prev, vers);
                let src = registry_src.join(format!("{}-{}", krate.name(), prev));
                let dst = registry_src.join(format!("{}-{}", krate.name(), vers));
                //Command::new("cargo").args(&["crev", "crate", "diff", "--src", &prev, "--dst", &vers, krate.name()]).status().unwrap();
                Command::new("git").args(&["--no-pager", "-c", "core.autocrlf=false", "diff", "--ignore-cr-at-eol", "--no-index"]).arg(src).arg(dst).status().unwrap();
            } else {
                eprintln!("{} has no previous version", vers);
                exit(1);
            }
            return;
        }
        prev = Some(version);
    }
}
