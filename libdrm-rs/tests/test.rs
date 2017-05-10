extern crate libdrm;

use libdrm::Device;
use libdrm::UnprivilegedDevice;
use libdrm::ClientCapability;
use libdrm::control::Control;
use libdrm::control::MasterControl;
use libdrm::control::ConnectorState;
use libdrm::control::ResourceInfo;

use std::fs::{OpenOptions, File};
use std::os::unix::io::{AsRawFd, RawFd};

#[derive(Debug)]
struct TestDevice(File);

impl AsRawFd for TestDevice {
    fn as_raw_fd(&self) -> RawFd { self.0.as_raw_fd() }
}

impl Device for TestDevice {}
impl Control for TestDevice {}
impl UnprivilegedDevice for TestDevice {}

#[test]
fn legacy_modesetting_dumbbuffer() {
    // Open the file
    let mut options = OpenOptions::new();
    options.read(true);
    options.write(true);
    let file = options.open("/dev/dri/card0").unwrap();

    // Create a TestDevice from it and acquire the master lock
    let card = TestDevice(file);
    let mref = card.set_master().unwrap();

    // Enumerate resources and get a reasonable setup
    let res = card.resource_ids().unwrap();
    let con = res.connectors().iter().map(| &id |
        card.connector(id).unwrap()).filter(| c | c.connection_state() ==
                                             ConnectorState::Connected).next().unwrap();
    let crtc = res.crtcs().iter().map(| &id | card.crtc(id).unwrap()).next().unwrap();
    let enc = res.encoders().iter().map(| &id | card.encoder(id).unwrap()).next().unwrap();

    // Get the first mode
    let modes = con.modes().clone();
    let m = modes[0].clone();

    // Create a dumb buffer
    let db = card.dumbbuffer(m.size(), 32).unwrap();
    let fb = card.create_framebuffer(&db).unwrap();
    let mut dbmap = db.map().unwrap();

    // Create a slice from this one connector.
    let cons = [con.id()];

    // Start from black and go to white.
    for x in 0..8 {
        let brightness = (x * 32) as u8;
        let mut data = vec![brightness; dbmap.as_slice().len()];
        dbmap.as_slice().copy_from_slice(&data);
        mref.set_crtc(crtc.id(), fb, &cons, (0, 0), Some(m));
    }
}

#[test]
fn properties() {
    // Open the file
    let mut options = OpenOptions::new();
    options.read(true);
    options.write(true);
    let file = options.open("/dev/dri/card0").unwrap();

    // Create a TestDevice from it and enable atomic modesetting.
    let card = TestDevice(file);
    card.set_client_cap(ClientCapability::UniversalPlanes, true).unwrap();
    card.set_client_cap(ClientCapability::Atomic, true).unwrap();

    // Enumerate resources and get a connector
    let res = card.resource_ids().unwrap();

    for &id in res.connectors() {
        for &p in card.properties(id).unwrap().handles() {
            let p = card.resource_property(p).unwrap();
            println!("{:?}", p.info().name());
        }
    }

    for &id in res.crtcs() {
        println!("Loading {:?}", id);
        println!("Props: {:?}", card.properties(id));
        for &p in card.properties(id).unwrap().handles() {
            let p = card.resource_property(p).unwrap();
            println!("{:?}", p.info().name());
        }
    }

    for &id in res.framebuffers() {
        for &p in card.properties(id).unwrap().handles() {
            let p = card.resource_property(p).unwrap();
            println!("{:?}", p.info().name());
        }
    }

    let planes = card.plane_ids().unwrap();

    for &id in planes.planes() {
        for &p in card.properties(id).unwrap().handles() {
            let p = card.resource_property(p).unwrap();
            println!("{:?}", p.info().name());
        }
    }
}
