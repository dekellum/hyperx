extern crate version_check;

fn main() {
    match version_check::is_min_version("1.27.2") {
        Some((true, actual_v)) => {
            eprintln!("hyperx MSRV test passed: {} (actual)", actual_v);
        }
        Some((false, actual_v)) => {
            panic!("hyperx MSRV is 1.27.2 > {} (this rustc)", actual_v);
        }
        None => {
            panic!("couldn't query version info from rustc");
        }
    }
}
