extern crate libdrm;

use libdrm::Device;
use libdrm::DRMDevice;
use libdrm::ClientCapability;
use libdrm::control::Control;

use std::fs::File;
use std::fs::OpenOptions;

use std::thread::sleep_ms;

#[test]
fn enumerate() {
    let mut options = OpenOptions::new();
    options.read(true);
    options.write(true);
    let file = options.open("/dev/dri/card0").unwrap();

    let mut card: Device<_> = Device::from(file);

    println!("{:?}", card.magic());

    println!("{:?}", card.set_client_cap(ClientCapability::Stereo3D, true));
    println!("{:?}", card.set_client_cap(ClientCapability::UniversalPlanes, true));
    println!("{:?}", card.set_client_cap(ClientCapability::Atomic, true));

    println!("{:#?}", card.resource_ids());
}
