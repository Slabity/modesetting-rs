error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        Incompatible {
            description("incompatible resource pair")
            display("attempted to attach resouces")
        }
        NotAvailable {
            description("unavailable resource requested")
            display("attempted to acquire resource")
        }
        UnknownPropertyType(flags: u32) {
            description("unknown property type")
            display("property not stored in enum or blob")
        }
    }
}

