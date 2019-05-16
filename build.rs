extern crate cc;
extern crate pkg_config;

fn main() {
    let lib = pkg_config::probe_library("Qt5Core").unwrap();

    let mut builder = cc::Build::new();
    builder.file("src/signal.cpp");

    for path in lib.include_paths {
        builder.include(path);
    }

    builder.compile("signal");
}
