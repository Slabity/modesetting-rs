extern crate drm;

use drm::control::Device;

#[test]
fn control() {
    let dev = Device::open("/dev/dri/controlD64").unwrap();
}
