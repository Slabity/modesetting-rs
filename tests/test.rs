extern crate modesetting;

use modesetting::Device;
use modesetting::Resource;

#[test]
fn enumerate() {
    let device = Device::open("/dev/dri/card0").unwrap();
    let resources = device.resources().unwrap();

    for &id in resources.connectors.iter() {
        let connector = device.connector(id).unwrap();

        let props = connector.get_property_ids().unwrap();
        for id in props {
            let prop = device.property(id);
            println!("{:#?}", prop);
        }
    }

    for &id in resources.encoders.iter() {
        let encoder = device.encoder(id).unwrap();
    }

    for &id in resources.controllers.iter() {
        let controller = device.controller(id).unwrap();

        let props = controller.get_property_ids().unwrap();
        for id in props {
            let prop = device.property(id);
            println!("{:#?}", prop);
        }
    }

    for &id in resources.planes.iter() {
        let plane = device.plane(id).unwrap();

        let props = plane.get_property_ids().unwrap();
        for id in props {
            let prop = device.property(id);
            println!("{:#?}", prop);
        }
    }
}

