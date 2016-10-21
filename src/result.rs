error_chain! {
    foreign_links {
        ::std::io::Error, IoError;
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
    }
}

