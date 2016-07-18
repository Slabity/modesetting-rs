extern crate modesetting;

use modesetting::control::Device;
use modesetting::control::Connection;

#[test]
fn control() {
    let control = Device::open("/dev/dri/card0").unwrap();
    let resources = control.resources().unwrap();

    let connectors = resources.connectors().map(
        | result | result.unwrap()
        );
    let connected = connectors.filter(
        | con | con.connection() == Connection::Connected
        );

    for con in connected {
        println!("Connected: {:?}", con);
    }
}
