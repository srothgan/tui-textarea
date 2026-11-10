use std::{env, fs, path::PathBuf};

fn main() {
    let check = match env::args().nth(1).as_deref() {
        None => false,
        Some("--check") => true,
        _ => panic!("usage: seed_corpus [--check]"),
    };
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("seeds");
    let seeds = tui_textarea_fuzz::seeds::all();
    for seed in &seeds {
        let dir = root.join(seed.target.name());
        let path = dir.join(seed.name);
        if check {
            assert_eq!(
                fs::read(&path).expect("missing seed"),
                seed.bytes,
                "{}",
                path.display()
            );
        } else {
            fs::create_dir_all(&dir).unwrap();
            fs::write(&path, &seed.bytes).unwrap();
        }
    }
    println!(
        "{} {} deliberate seeds.",
        if check { "Verified" } else { "Wrote" },
        seeds.len()
    );
}
