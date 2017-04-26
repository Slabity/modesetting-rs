extern crate libdrm;

use libdrm::Device;
use libdrm::DRMDevice;
use libdrm::ClientCapability;
use libdrm::control::Control;

use std::fs::OpenOptions;

#[test]
fn enumerate() {
    let mut options = OpenOptions::new();
    options.read(true);
    options.write(true);
    let file = options.open("/dev/dri/card0").unwrap();

    let card: Device<_> = Device::from(file);

    println!("{:?}", card.magic());

    println!("{:?}", card.set_client_cap(ClientCapability::Stereo3D, true));
    println!("{:?}", card.set_client_cap(ClientCapability::UniversalPlanes, true));
    println!("{:?}", card.set_client_cap(ClientCapability::Atomic, true));

    let res = card.resource_ids().unwrap();
    let planes = card.plane_ids().unwrap();

    for &id in res.crtcs().iter() {
        let crtc = card.crtc(id).unwrap();
        card.gamma(id, crtc.gamma_length).unwrap();
    }

    for &id in planes.planes().iter() {
        card.plane(id).unwrap();
    }

    for &id in res.encoders().iter() {
        card.encoder(id).unwrap();
    }

    for &id in res.connectors().iter() {
        card.connector(id).unwrap();
    }

    let fb = {
        let db = card.dumbbuffer((1920, 1080), 32).unwrap();
        card.create_framebuffer(&db).unwrap()
    };

    println!("{:#?}", fb);
}
