fn main() {
    let manifest_dir: &str = env!("CARGO_MANIFEST_DIR");
    println!("cargo:rustc-link-arg=-T{manifest_dir}/ferrum.ld");
    println!("cargo:rerun-if-changed=ferrum.ld");
}
