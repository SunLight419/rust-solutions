use clap::App;

// cargo run -n hello world
// -n が cargo の引数として解釈される
// cargo run -- -n hello world
fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Me")
        .about("Rust echo")
        .get_matches();
}
