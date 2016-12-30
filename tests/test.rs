extern crate modesetting;

use modesetting::Context;
use modesetting::Resource;
use modesetting::PropertyValue::Enum;
use modesetting::PropertyInfo;
use modesetting::Buffer;

#[test]
fn enumerate() {
    let mut ctx = Context::from_path("/dev/dri/card0").unwrap();

    let cons: Vec<_> = ctx.connectors().iter().filter(| &c | {
        match c.connector_state() {
            Ok(modesetting::ConnectorState::Connected(_, (_, _))) => true,
            _ => false
        }
    }).collect();

    let connected = cons.get(0).unwrap();

    let planes: Vec<_> = ctx.planes().iter().filter(| &pl | {
        match pl.get_properties() {
            Ok(props) => {
                match props.iter().find(| &pr | { pr.name() == "type" }) {
                    Some(pr) => {
                        match pr.value() {
                            &Enum(ref en) => en.value() == 0,
                            _ => false
                        }
                    },
                    None => false
                }
            },
            _ => false
        }
    }).collect();

    println!("{:#?}", planes);

    let prime_plane = planes.get(0).unwrap();

    println!("{:#?}", prime_plane.get_properties());

    let db = ctx.create_dumbbuffer(1920, 1080, 32).unwrap();
    ctx.create_framebuffer(&db).unwrap();
}

