extern crate gcc;

fn main() {
    gcc::Config::new().file("src/factors.c").compile("libfactors.a");
}
