use anyhow::*;
use evdev_rs::enums::EventType;
use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::map;
use nom::Err as NomErr;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::*;
use tap::Tap;

use custom_combinators::*;
use error::*;
use identifier::*;
use key::*;
use key_action::*;
use key_sequence::*;
#[cfg(test)]
use tests::*;


#[cfg(test)]
pub(super) fn nom_ok<'a, T>(value: T) -> ResNew<&'a str, T> { Ok(("", (value, None))) }

#[cfg(test)]
pub(super) fn nom_ok_rest<T>(rest: &str, value: T) -> ResNew<&str, T> { Ok((rest, (value, None))) }

#[cfg(test)]
pub(super) fn nom_eval<'a, T>(value: ResNew<&str, T>) -> T { value.unwrap().1.0 }

#[cfg(test)]
pub(super) fn nom_no_last_err<'a, T>(value: ResNew<&str, T>) -> ResNew<&str, T> {
    match value {
        Ok((input, (val, _))) => Ok((input, (val, None))),
        Err(err) => Err(err)
    }
}

use crate::*;

mod custom_combinators;
mod identifier;
mod key;
pub mod key_action;
mod key_sequence;
mod error;
pub(crate) mod python;
