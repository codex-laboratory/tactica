fn main() {
    println!("cargo:rustc-link-lib=dylib=calc");
    println!("cargo:rustc-link-search=native=../engine-core/build/Release");
}