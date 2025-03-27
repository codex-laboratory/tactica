fn main() {
    println!("cargo:rustc-link-lib=dylib=calc");
    println!("cargo:rustc-link-search=native=../calculations/build/Release");
}