fn main() { 
    println!(r"cargo:rustc-link-search=native={}\lib", env!("CARGO_MANIFEST_DIR"));
    println!("cargo:rustc-link-lib=static=kahip_static");
    println!("cargo:rustc-link-lib=static=stdc++");
}
   