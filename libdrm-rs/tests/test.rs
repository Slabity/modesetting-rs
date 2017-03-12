extern crate libdrm;

use libdrm::Card;

use std::fs::File;
use std::fs::OpenOptions;

#[test]
fn enumerate() {
    let mut options = OpenOptions::new();
    options.read(true);
    options.write(true);
    let file = options.open("/dev/dri/card0").unwrap();

    let mut card: Card<File> = Card::from(file);

    let res = card.resource_ids();
    let ver = card.version_info();
    let bid = card.get_bus_id();

    println!("{:#?}", bid);
}
