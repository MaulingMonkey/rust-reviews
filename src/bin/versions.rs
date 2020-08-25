use std::process::exit;
use std::time::*;

fn main() {
    let mut args = std::env::args();
    let _exe = args.next();
    let krate = args.next().unwrap_or_else(|| { eprintln!("Usage: cargo versions [crate]"); exit(1); });

    // NOTE: cargo seems to use some kind of semi-binary format now:~\.cargo\registry\index\github.com-1ecc6299db9ec823\.cache
    //       crates_index OTOH uses the raw textual format... so it needs to update
    let index = crates_index::Index::new_cargo_default();
    let now = SystemTime::now();
    let day = Duration::from_secs(24 * 60 * 60);
    let index_mod = index.path().join(".git").join("index").metadata().and_then(|md| md.modified());
    if !index.exists() {
        println!("Cloning index (this will take awhile...)");
        index.retrieve().unwrap();
    } else if index_mod.map_or(true, |index_mod| index_mod <= now - day) {
        println!("Updating index (this may take awhile...)");
        index.update().unwrap();
    }
    let krate = index.crate_(&krate).unwrap_or_else(|| { eprintln!("No such crate {:?}", krate); exit(1); });

    println!("versions:");
    for version in krate.versions() {
        println!("    {}", version.version());
    }
    println!("earliest: {}", krate.earliest_version().version());
    println!("latest:   {}", krate.latest_version().version());
    println!("stable:   {}", krate.highest_stable_version().map_or("N/A".to_string(), |v| v.to_string()));
}
