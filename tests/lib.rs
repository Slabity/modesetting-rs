extern crate drm;

use drm::control::Device;

#[test]
fn control() {
    let control = Device::open("/dev/dri/card0").unwrap();
    let resources = control.resources().unwrap();
    println!("{:#?}", resources);

    for res in resources.connectors() {
        println!("{:#?}", res);
    };
    for res in resources.encoders() {
        println!("{:#?}", res);
    };
    for res in resources.crtcs() {
        println!("{:#?}", res);
    };
}
