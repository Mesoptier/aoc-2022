use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::tuple,
    IResult,
};

/// Returns mutable references to a pair of indices at once.
///
/// Panics if `index1 == index2` or either index is out of range.
pub fn get_pair_mut<T>(slice: &mut [T], (index1, index2): (usize, usize)) -> (&mut T, &mut T) {
    assert_ne!(index1, index2, "indices must not be the same");

    if index1 < index2 {
        let (left, right) = slice.split_at_mut(index2);
        (&mut left[index1], &mut right[0])
    } else {
        let (left, right) = slice.split_at_mut(index1);
        (&mut right[0], &mut left[index2])
    }
}

pub fn parse_signed_int<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    map_res(recognize(tuple((opt(tag("-")), digit1))), str::parse::<T>)(input)
}
