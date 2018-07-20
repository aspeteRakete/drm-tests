extern crate drm;

fn main() {
    println!("Hello World");
    let card = DrmCard::open_global();
    card.print_connector_info();
}