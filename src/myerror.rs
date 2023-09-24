pub mod myerror {
    use core::fmt;
    use std::error::Error;

    #[derive(Debug)]
    pub struct MyError {
        pub(crate) message: String,
        pub(crate) position: usize,
    }

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}, by {}", self.message, self.position)
        }
    }

    impl Error for MyError {}
}
