fn main() {
    cc::Build::new().file("src/privod.c").compile("privod");
    println!("cargo:rerun-if-changed=src/privod.c");
}
