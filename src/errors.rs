//! Types and traits used for internal errors.

use error_chain::error_chain;

error_chain! {

    types {
        Error, ErrorKind, ChainErr, Result;
    }

    errors {
        /// Got to the end of the input stream but was expecting more.
        UnexpectedEOF {
            display("Unexpected EOF")
            description("Unexpected EOF")
        }

        /// Reached an unknown character while lexing.
        UnknownCharacter(ch: char) {
            display("Unknown Character, {:?}", ch)
            description("Unknown Character")
        }

        /// A message which corresponds to some location in the source code.
        MessageWithLocation(loc: usize, msg: &'static str) {
            display("{} at {}", msg, loc)
            description("Custom Error")
        }
    }

    foreign_links {
        Io(::std::io::Error) #[doc = "Wrapper around a `std::io::Error`"];
        Utf8(::std::str::Utf8Error) #[doc = "An error parsing data as UTF-8"];
        FloatParsing(::std::num::ParseFloatError) #[doc = "A float parsing error"];
        IntParsing(::std::num::ParseIntError) #[doc = "An integer parsing error"];
    }
}
