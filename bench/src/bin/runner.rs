use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const TARGETS: &[&str] = &[
    "insert_append",
    "insert_random",
    "insert_long",
    "search_forward",
    "search_backward",
    "cursor_char",
    "cursor_word",
    "cursor_paragraph",
    "cursor_edge",
    "delete_char",
    "delete_word",
    "delete_line",
    "wrap_render",
    "undo_redo",
];

fn main() {
    let total = TARGETS.len();
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir)
        .parent()
        .expect("bench package must have a parent workspace directory");

    let mut failed: Vec<&str> = Vec::new();

    for (i, target) in TARGETS.iter().enumerate() {
        let n = i + 1;
        let timestamp = utc_now();
        println!("[{}/{}] START {} at {}", n, total, target, timestamp);
        io::stdout().flush().ok();

        let wall_start = Instant::now();
        let result = Command::new(&cargo)
            .args([
                "bench",
                "-p",
                "tui-textarea-bench",
                "--bench",
                target,
                "--",
                "--verbose",
                "--noplot",
            ])
            .current_dir(workspace_root)
            .status();
        let duration = wall_start.elapsed();

        let (status_str, ok) = match result {
            Ok(s) if s.success() => ("ok", true),
            Ok(_) => ("err", false),
            Err(e) => {
                eprintln!("  error: failed to spawn cargo for {}: {}", target, e);
                ("err", false)
            }
        };

        println!(
            "[{}/{}] DONE  {} status={} duration={:.2}s",
            n,
            total,
            target,
            status_str,
            duration.as_secs_f64()
        );

        if !ok {
            failed.push(target);
        }
    }

    println!();
    if failed.is_empty() {
        println!("All {} benchmark targets completed successfully.", total);
    } else {
        eprintln!("Failed targets ({}/{}):", failed.len(), total);
        for t in &failed {
            eprintln!("  - {}", t);
        }
        std::process::exit(1);
    }
}

/// Format the current UTC time as `YYYY-MM-DDTHH:MM:SSZ`.
/// Uses only `std::time` — no external date crate needed.
fn utc_now() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let secs = (ts % 60) as u32;
    let mins = ((ts / 60) % 60) as u32;
    let hours = ((ts / 3600) % 24) as u32;
    let days = ts / 86400;

    let (year, month, day) = days_since_epoch_to_ymd(days);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, mins, secs
    )
}

/// Convert days since Unix epoch (1970-01-01) to (year, month, day).
/// Algorithm: http://howardhinnant.github.io/date_algorithms.html
fn days_since_epoch_to_ymd(days: u64) -> (u32, u32, u32) {
    let z = days as i64 + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as u32, m as u32, d as u32)
}
