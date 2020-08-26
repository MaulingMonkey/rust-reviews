// --disable-extension matklad.rust-analyzer --disable-extension kalitaalexey.vscode-rust --disable-extension rust-lang.rust

use std::process::{Command, exit};
use std::path::PathBuf;

fn main() {
    // NOTE: cargo seems to use some kind of semi-binary format now:~\.cargo\registry\index\github.com-1ecc6299db9ec823\.cache
    //       crates_index OTOH uses the raw textual format... so it needs to update
    let index = crates_index::Index::new_cargo_default();

    let mut args = std::env::args();
    let _exe = args.next();
    let krate = args.next().unwrap_or_else(|| { eprintln!("Usage: cargo open [crate] [version]"); exit(1); });
    let krate = index.crate_(&krate).unwrap_or_else(|| { eprintln!("No such crate {:?}", krate); exit(1); });
    let vers  = args.next()
        .or_else(|| krate.highest_stable_version().map(|v| v.to_string()))
        .unwrap_or_else(|| { eprintln!("Usage: cargo open [crate] [version]"); exit(1); });

    let mut vscode = Command::new("cmd");
    vscode.args(&["/S", "/C", "call", "code",
        // Avoid Cargo.lock file spew
        "--disable-extension", "matklad.rust-analyzer",
        "--disable-extension", "kalitaalexey.vscode-rust",
        "--disable-extension", "rust-lang.rust",
    ]);

    let versions = if vers != "*" {
        vec![vers]
    } else {
        krate.versions().iter().filter(|v| !v.is_yanked()).map(|v| v.version().to_string()).collect()
    };

    for vers in versions {
        let mut dir = PathBuf::from(index.path());
        dir.pop(); // github.com-1ecc6299db9ec823
        dir.pop(); // index
        dir.push("src");
        dir.push("github.com-1ecc6299db9ec823");
        dir.push(format!("{}-{}", krate.name(), vers));
        vscode.arg(dir);
    }

    vscode.status().unwrap();
}
