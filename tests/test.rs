extern crate modesetting;

use modesetting::Context;
use modesetting::Resource;
use modesetting::Connector;
use modesetting::Controller;
use modesetting::Plane;
use modesetting::property::*;

// Gets the first connected connector
fn get_connector(ctx: &Context) -> &Connector {
    let connected: Vec<_> = ctx.connectors().iter().filter(| &c | {
        match c.connector_state() {
            Ok(modesetting::ConnectorState::Connected(_, (_, _))) => true,
            _ => false
        }
    }).collect();
    connected.get(0).unwrap()
}

// Gets the first controller
fn get_controller(ctx: &Context) -> &Controller {
    ctx.controllers().get(0).unwrap()
}

// Gets a primary plane
fn get_plane(ctx: &Context) -> &Plane {
    // Get the first primary plane we can find.
    let prime_planes: Vec<_> = ctx.planes().iter().filter(| &pl | {
        match pl.properties() {
            Ok(props) => {
                match props.iter().find(| &pr | { pr.name() == "type" }) {
                    Some(pr) => {
                        match pr {
                            &Value::Enum(ref en) => *en.value() == 1,
                            _ => false
                        }
                    },
                    None => false
                }
            },
            _ => false
        }
    }).collect();
    prime_planes.get(0).unwrap()
}

fn get_property<T>(res: &Resource<T>, name: &str) -> Value {
    let props = res.properties().unwrap();
    let prop = props.into_iter().find(| ref pr | pr.name() == name).unwrap();
    prop
}

#[test]
fn enumerate() {
    // Create a context
    let ctx = Context::from_path("/dev/dri/card0").unwrap();
    let mut updates = Vec::new();

    // Create a framebuffer from a dumbbuffer
    let db = ctx.create_dumbbuffer(1920, 1080, 32).unwrap();
    let mut map = db.map().unwrap();

    for mut b in map.as_mut_slice() {
        *b = 128;
    }

    let fb = ctx.create_framebuffer(&db).unwrap();

    // Get a connector, controller, and plane
    let connector = get_connector(&ctx);
    let controller = get_controller(&ctx);
    let plane = get_plane(&ctx);

    // Get first mode:
    let modes = connector.modes().unwrap();
    let mode = modes.get(0).unwrap();

    // Create the blob
    let blob = ctx.create_blob(mode).unwrap();

    // Get updates for the above
    match get_property(&connector, "CRTC_ID") {
        Value::Object(p) => updates.push(p.update(&controller)),
        _ => panic!("Expected an object")
    };

    match get_property(&controller, "MODE_ID") {
        Value::Blob(p) => updates.push(p.update(&blob)),
        _ => panic!("Expected a blob")
    };

    match get_property(&controller, "ACTIVE") {
        Value::URange(p) => updates.push(p.update(1)),
        _ => panic!("Expected an unsigned range")
    };

    match get_property(&plane, "FB_ID") {
        Value::Object(p) => updates.push(p.update(&fb)),
        _ => panic!("Expected an unsigned range")
    };
    match get_property(&plane, "CRTC_ID") {
        Value::Object(p) => updates.push(p.update(&controller)),
        _ => panic!("Expected an object")
    };


    // Get updates for setting plane position
    match get_property(&plane, "SRC_X") {
        Value::URange(p) => updates.push(p.update(0)),
        _ => panic!("Expected an unsigned range")
    };
    match get_property(&plane, "SRC_Y") {
        Value::URange(p) => updates.push(p.update(0)),
        _ => panic!("Expected an unsigned range")
    };
    match get_property(&plane, "SRC_W") {
        Value::URange(p) => updates.push(p.update(1920 << 16)),
        _ => panic!("Expected an unsigned range")
    };
    match get_property(&plane, "SRC_H") {
        Value::URange(p) => updates.push(p.update(1080 << 16)),
        _ => panic!("Expected an unsigned range")
    };
    match get_property(&plane, "CRTC_X") {
        Value::IRange(p) => updates.push(p.update(0)),
        _ => panic!("Expected a signed range")
    };
    match get_property(&plane, "CRTC_Y") {
        Value::IRange(p) => updates.push(p.update(0)),
        _ => panic!("Expected a signed range")
    };
    match get_property(&plane, "CRTC_W") {
        Value::URange(p) => updates.push(p.update(1920)),
        _ => panic!("Expected an unsigned range")
    };
    match get_property(&plane, "CRTC_H") {
        Value::URange(p) => updates.push(p.update(1080)),
        _ => panic!("Expected an unsigned range")
    };


    ctx.commit(updates.iter()).unwrap();

    let time = std::time::Duration::from_millis(1000);
    std::thread::sleep(time);
}

