extern crate version_check;

static PACKAGE: &str = "hyperx";
static MSRV: &str = "1.27.2";

fn main() {
    match version_check::is_min_version(MSRV) {
        Some((true, actual_v)) => {
            eprintln!(
                "{} MSRV test passed: {} (actual)",
                PACKAGE, actual_v);
        }
        Some((false, actual_v)) => {
            panic!(
                "{} MSRV is {} > {} (this rustc)",
                PACKAGE, MSRV, actual_v);
        }
        None => {
            panic!(
                "{}: couldn't query version info from rustc",
                PACKAGE);
        }
    }
}
