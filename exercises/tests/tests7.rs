// build.rs
fn main() {
    // Get the current time as a duration since UNIX_EPOCH
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Tell Cargo to set the TEST_FOO environment variable for the crate
    // The format is: println!("cargo:rustc-env=VAR_NAME=value");
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    
    // Optional: Re-run if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
}