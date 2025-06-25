//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // Set TEST_FOO environment variable for tests7
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // Set TEST_FOO to 5 seconds before current time to ensure it's in range
    println!("cargo:rustc-env=TEST_FOO={}", timestamp - 5);

    // For tests8, enable "pass" feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
