error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        PermissionDenied {
            description("not the DRM master")
            display("not the DRM master")
        }
        InvalidNode {
            description("invalid DRM control node")
            display("invalid DRM control node")
        }
        InvalidVersion {
            description("invalid version description")
            display("invalid version description")
        }
        InvalidResource(id: u32) {
            description("could not load resource")
            display("could not load resource: {}", id)
        }
        Unsupported(msg: &'static str) {
            description("unsupported operation")
            display("unsupported operation {}", msg)
        }
        UnknownPropertyType(flags: u32) {
            description("unknown property type")
            display("property not stored in enum or blob")
        }
    }
}

