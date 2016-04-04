//! Encode the following ASN1 structure:
//!
//!     Example ::= SEQUENCE {
//!         b BOOLEAN,
//!         i INTEGER
//!     }
#![feature(type_macros)]

#[macro_use] extern crate asn1;

fn main() {
    asn1!(
        Example ::= SEQUENCE {
            b BOOLEAN,
            i INTEGER,
        }
    );
    let ex = Example {
        b: true,
        i: 42,
    };
    let data = ex.to_der();
    assert_eq!(data, [48, 6, 1, 1, 255, 2, 1, 42]);

    let hexstr: String = data.iter().map(|b| format!("{:02X} ", b)).collect::<Vec<_>>().concat();
    println!("Encoded: {}", hexstr);
}
