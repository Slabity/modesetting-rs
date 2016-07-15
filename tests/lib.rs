extern crate drm;

use drm::control::Device;

#[test]
fn control() {
    let control = Device::open("/dev/dri/controlD64").unwrap();
    println!("{:#?}", control.resources().unwrap());
}
