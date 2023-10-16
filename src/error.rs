pub mod myerror {
    use core::fmt;
    use std::error::Error;

    #[derive(Debug)]
    pub struct MyError {
        pub(crate) message: String,
    }

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl Error for MyError {}
}
