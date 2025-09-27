fn main() {
    println!("cargo:rerun-if-env-changed=BOLERO_FUZZER");
    if let Ok(fuzzer) = std::env::var("BOLERO_FUZZER") {
        println!("cargo:rustc-link-arg=-export_dynamic");
        println!("cargo:rustc-cfg=fuzzing");
        println!("cargo:rustc-cfg=fuzzing_{}", fuzzer.to_string());
    }
}