# modesetting-rs

High level bindings to modesetting functionality.

Modesetting is the process of activating display modes on a computer's display controller.

```toml
[dependencies]
modesetting = "*"
```

## Usage

```rust
extern crate modesetting;

use modesetting::Device;
use modesetting::ConnectorState;
use modesetting::DumbBuffer;

use std::thread::sleep;
use std::time::Duration;

#[test]
fn main() {
    // Open the character device for modesetting
    let dev = Device::open("/dev/dri/card0").unwrap();

    // Get a master handle for modesetting.
    let master = dev.master_lock().unwrap();

    // Attempt to iterate through each connector.
    for result in master.connectors() {
        // Unwrap each result.
        let mut con = result.unwrap();

        // Skip the ones that aren't connected.
        if con.state() != ConnectorState::Connected {
            continue
        }

        // Get the primary mode of the connector.
        let mode = con.modes().into_iter().next().unwrap();

        // From the mode, let's create a dumb buffer with the correct size.
        let (width, height) = mode.display;
        let dumb = dev.dumb_buffer(width as u32, height as u32, 32).unwrap();

        // Get a slice that maps to the dumb buffer.
        let mapping = dumb.map().unwrap();

        // Make each pixel white
        for pixel in mapping.iter_mut() {
            *pixel = 255;
        }

        // Create a virtual framebuffer for it.
        let fb = dev.framebuffer(&dumb).unwrap();

        // Get the first available encoder.
        let encoder = con.encoders().next().unwrap().unwrap();

        // Get a controller that can use this encoder.
        let mut controller = encoder.controllers().next().unwrap().unwrap();

        // Attach the encoder to the connector.
        con.attach_encoder(encoder);

        // Set the controller.
        controller.set_controller(&fb, vec![con], mode);
    }

    // Wait a few seconds to see the result.
    let duration = Duration::new(5, 0);
    sleep(duration);
}```
