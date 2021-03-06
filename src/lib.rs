#[macro_use]
mod macros;

mod computer;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day03_v2;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day08_v2;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day12_v2;
pub mod day13;
pub mod day14;
pub mod day16;
mod utils;

pub use self::error::Error;
pub use self::reader::Reader;

mod error {
    use std::fmt;
    use std::io;

    #[derive(Debug)]
    pub enum Error {
        Custom(String),
        Io(io::Error),
        Fmt(std::fmt::Error),
        ParseInt(std::num::ParseIntError),
    }

    impl From<std::fmt::Error> for Error {
        fn from(e: std::fmt::Error) -> Self {
            Self::Fmt(e)
        }
    }

    impl From<io::Error> for Error {
        fn from(e: io::Error) -> Self {
            Self::Io(e)
        }
    }

    impl From<std::num::ParseIntError> for Error {
        fn from(e: std::num::ParseIntError) -> Self {
            Self::ParseInt(e)
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Custom(s) => write!(f, "{}", s),
                Self::Io(e) => write!(f, "{}", e),
                Self::ParseInt(e) => write!(f, "{}", e),
                Self::Fmt(e) => write!(f, "{}", e),
            }
        }
    }

    impl std::error::Error for Error {}
}

mod reader {
    use std::fs;
    use std::io;

    pub enum Reader<'a> {
        File(io::BufReader<fs::File>),
        Stdin(io::StdinLock<'a>),
    }

    impl<'a> io::Read for Reader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            match self {
                Self::File(reader) => reader.read(buf),
                Self::Stdin(guard) => guard.read(buf),
            }
        }
    }

    impl<'a> io::BufRead for Reader<'a> {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            match self {
                Self::File(reader) => reader.fill_buf(),
                Self::Stdin(guard) => guard.fill_buf(),
            }
        }

        fn consume(&mut self, amt: usize) {
            match self {
                Self::File(reader) => reader.consume(amt),
                Self::Stdin(guard) => guard.consume(amt),
            }
        }
    }
}
