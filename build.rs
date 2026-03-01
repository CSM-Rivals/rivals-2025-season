fn main() {
    // Link the Windows multimedia library (required for AprilTag timing on Windows)
    println!("cargo:rustc-link-lib=winmm");

    // Tell Cargo to rebuild if you change the shim headers
    println!("cargo:rerun-if-changed=c_include/pthread.h");
    println!("cargo:rerun-if-changed=c_include/sched.h");
}