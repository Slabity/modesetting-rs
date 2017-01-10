extern crate modesetting;

use modesetting::Context;
use modesetting::Resource;
use modesetting::Connector;
use modesetting::Controller;
use modesetting::Plane;
use modesetting::Buffer;
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
                            &Value::Enum(ref en) => *en.value() == 0,
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
    props.into_iter().find(| ref pr | pr.name() == name).unwrap()
}

#[test]
fn enumerate() {
    // Create a context
    let mut ctx = Context::from_path("/dev/dri/card0").unwrap();
    let mut updates = Vec::new();

    // Create a framebuffer from a dumbbuffer
    let db = ctx.create_dumbbuffer(1920, 1080, 32).unwrap();
    let fb = ctx.create_framebuffer(&db).unwrap();

    // Get a connector, controller, and plane
    let connector = get_connector(&ctx);
    let controller = get_controller(&ctx);
    let plane = get_plane(&ctx);

    // Get updates for the above
    println!("Updating CRTC_ID on Connector");
    match get_property(&connector, "CRTC_ID") {
        Value::Object(p) => updates.push(p.update(&controller)),
        _ => panic!("Expected an object")
    };
    ctx.commit(updates.iter()).unwrap();

    println!("Updating CRTC_ID on Plane");
    match get_property(&plane, "CRTC_ID") {
        Value::Object(p) => updates.push(p.update(&controller)),
        _ => panic!("Expected an object")
    };
    ctx.commit(updates.iter()).unwrap();
    /*println!("Updating FB_ID on Plane");
    match get_property(&plane, "FB_ID") {
        Value::Object(p) => updates.push(p.update(&fb)),
        _ => panic!("Expected an unsigned range")
    };
    ctx.commit(updates.iter()).unwrap();*/

    // Get updates for setting plane position
    println!("SRC_X");
    match get_property(&plane, "SRC_X") {
        Value::URange(p) => updates.push(p.update(0)),
        _ => panic!("Expected an unsigned range")
    };
    ctx.commit(updates.iter()).unwrap();
    println!("SRC_Y");
    match get_property(&plane, "SRC_Y") {
        Value::URange(p) => updates.push(p.update(0)),
        _ => panic!("Expected an unsigned range")
    };
    println!("CRTC_X");
    ctx.commit(updates.iter()).unwrap();
    match get_property(&plane, "CRTC_X") {
        Value::IRange(p) => updates.push(p.update(0)),
        _ => panic!("Expected a signed range")
    };
    ctx.commit(updates.iter()).unwrap();
    println!("CRTC_Y");
    match get_property(&plane, "CRTC_Y") {
        Value::IRange(p) => updates.push(p.update(0)),
        _ => panic!("Expected a signed range")
    };
    println!("CRTC_W");
    ctx.commit(updates.iter()).unwrap();
    match get_property(&plane, "CRTC_W") {
        Value::URange(p) => updates.push(p.update(1920)),
        _ => panic!("Expected an unsigned range")
    };
    println!("CRTC_H");
    ctx.commit(updates.iter()).unwrap();
    match get_property(&plane, "CRTC_H") {
        Value::URange(p) => updates.push(p.update(1080)),
        _ => panic!("Expected an unsigned range")
    };
    ctx.commit(updates.iter()).unwrap();

    // Set the controller to be active
    match get_property(&controller, "ACTIVE") {
        Value::URange(p) => updates.push(p.update(0)),
        _ => panic!("Expected an unsigned range")
    };

    ctx.commit(updates.iter()).unwrap();
}

