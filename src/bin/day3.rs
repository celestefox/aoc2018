// Example claim: "#123 @ 3,2: 5x4"

use nalgebra::MatrixN;
use nom::types::CompleteByteSlice;
use nom::{
    call, do_parse, error_position, is_digit, many1, map_res, named, newline,
    separated_nonempty_list, tag, take_while,
};
use std::fmt;
use std::str;
use typenum::U1000;

#[derive(Debug, PartialEq, Eq)]
struct Claim {
    pub id: u16,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub length: u16,
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Claim #{} @ {},{}: {}x{}",
            self.id, self.x, self.y, self.width, self.length
        )
    }
}

named!(claim_num<CompleteByteSlice, u16>, map_res!(take_while!(is_digit), |b: CompleteByteSlice| str::from_utf8(&b).map_err(|e| e.to_string()).map(|s: &str| s.parse::<u16>().map_err(|e| e.to_string()))?));

named!(claim<CompleteByteSlice, Claim>, do_parse!(tag!("#") >> id: claim_num >> tag!(" @ ") >> x: claim_num >> tag!(",") >> y: claim_num >> tag!(": ") >> width: claim_num >> tag!("x") >> length: claim_num >> (Claim {id, x, y, width, length})));

named!(claims<CompleteByteSlice, Vec<Claim>>, separated_nonempty_list!(tag!("\n"),claim));

pub fn main() {
    /*let test = b"#123 @ 3,2: 5x4";
    let tp = claim(CompleteByteSlice(test));
    println!("{:?}", tp);*/
    let file = include_bytes!("../../inputs/day3");
    let claims: Vec<Claim> = claims(CompleteByteSlice(file)).unwrap().1;
    let mut surface: MatrixN<u16, U1000> = MatrixN::<u16, U1000>::zeros();
    for claim in claims.iter() {
        let mut slice = surface.slice_mut(
            (claim.x.into(), claim.y.into()),
            (claim.width.into(), claim.length.into()),
        );
        slice.add_scalar_mut(1);
    }
    let mut conflicting_inches = 0;
    for coord in surface.iter() {
        if *coord > 1 {
            conflicting_inches += 1;
        }
    }
    println!("Conflicting inches of fabric: {}", conflicting_inches);
    for claim in claims.iter() {
        let slice = surface.slice(
            (claim.x.into(), claim.y.into()),
            (claim.width.into(), claim.length.into()),
        );
        if slice.iter().filter(|&coord| *coord != 1).count() == 0 {
            println!("Found a claim with no conflicts! {}", claim);
        }
    }
}
