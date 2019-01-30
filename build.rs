extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/madgwick.c")
        .compile("madgwick");
}
