fn main() {
    println!("cargo:rustc-link-lib=static=Crypt32");
    println!("cargo:rustc-link-lib=static=comdlg32");
}
