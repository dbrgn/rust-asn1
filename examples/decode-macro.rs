//! Decode the following ASN1 structure:
//!
//!     Example ::= SEQUENCE {
//!         b BOOLEAN,
//!         i INTEGER
//!     }
//!
#![feature(type_macros)]

#[macro_use] extern crate asn1;

fn main() {
    asn1!(
        Example ::= SEQUENCE {
            b BOOLEAN,
            i INTEGER,
        }
    );
    let data = vec![48, 6, 1, 1, 255, 2, 1, 42];
    match Example::from_der(&data) {
        Ok(ex) => println!("Decoded: b={}, i={}", ex.b, ex.i),
        Err(_) => println!("Error!"),
    }
}
