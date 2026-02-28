fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    
    // 1. Force the include path so the C compiler finds your dummy pthread.h
    println!("cargo:rustc-env=APRILTAG_SYS_WINDOWS_PTHREAD_INCLUDE_DIR={}/c_include", manifest_dir);
    
    // 2. Tell the C compiler to define the "NO_PTHREAD" flag during compilation
    // This handles the internal logic of the apriltag C code.
    println!("cargo:rustc-cfg=apriltag_no_pthread");

    // 3. Ensure this script reruns if you change the dummy header
    println!("cargo:rerun-if-changed=c_include/pthread.h");
}