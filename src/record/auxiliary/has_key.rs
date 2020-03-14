use crate::datum::Datum;

pub trait HasKey: Datum {
    type Key;
    const KEY_FIELDS: &'static [&'static str];
}
