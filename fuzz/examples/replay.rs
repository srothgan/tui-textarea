use std::{
    env, fs,
    path::{Path, PathBuf},
};
use tui_textarea_fuzz::{run, Target};

fn replay(target: Target, path: &Path) {
    eprintln!("Replay {}: {}", target.name(), path.display());
    run(target, &fs::read(path).expect("read fuzz input"));
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() || args == ["--seeds"] {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("seeds");
        let mut count = 0;
        for target in Target::ALL {
            let mut files: Vec<_> = fs::read_dir(root.join(target.name()))
                .expect("seed directory")
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.is_file())
                .collect();
            files.sort();
            for path in files {
                replay(target, &path);
                count += 1;
            }
        }
        println!("Replayed {count} seed inputs.");
    } else if args.len() == 2 {
        let target = Target::parse(&args[0])
            .expect("target: editing_history, selection_atomic, layout, or search");
        replay(target, Path::new(&args[1]));
    } else {
        panic!("usage: replay [--seeds | TARGET INPUT]");
    }
}
