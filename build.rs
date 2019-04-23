extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/signal.cpp")
        .include("/usr/include/x86_64-linux-gnu/qt5/QtCore")
        .include("/usr/include/x86_64-linux-gnu/qt5")
        .compile("signal");
}
