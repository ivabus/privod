fn main() {
    cc::Build::new()
        .file("src/get_status.c")
        .compile("get_status");
    println!("cargo:rerun-if-changed=src/get_status.c");
}
