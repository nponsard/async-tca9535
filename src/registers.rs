#![allow(non_snake_case)]
use bitfield::bitfield;

bitfield! {

    /// Invert the polarity of pins.
    ///
    /// `true` mean the polarity is inverted, inverting the reading in [`Input`].
    pub struct Polarity(u16);
    impl Debug;
    impl new;
    bool;

    pub P00, set_P00: 0;
    pub P01, set_P01: 1;
    pub P02, set_P02: 2;
    pub P03, set_P03: 3;
    pub P04, set_P04: 4;
    pub P05, set_P05: 5;
    pub P06, set_P06: 6;
    pub P07, set_P07: 7;

    pub P10, set_P10: 8;
    pub P11, set_P11: 9;
    pub P12, set_P12: 10;
    pub P13, set_P13: 11;
    pub P14, set_P14: 12;
    pub P15, set_P15: 13;
    pub P16, set_P16: 14;
    pub P17, set_P17: 15;
}

bitfield! {
    /// State of the pins.
    ///
    /// `true` means the pin is currently at a high logic level, unless its corresponding [`Polarity`]
    /// is inverted (polarity bit at `true`).
    pub struct Input(u16);
    impl Debug;
    impl new;
    bool;

    pub P00, _: 0;
    pub P01, _: 1;
    pub P02, _: 2;
    pub P03, _: 3;
    pub P04, _: 4;
    pub P05, _: 5;
    pub P06, _: 6;
    pub P07, _: 7;

    pub P10, _: 8;
    pub P11, _: 9;
    pub P12, _: 10;
    pub P13, _: 11;
    pub P14, _: 12;
    pub P15, _: 13;
    pub P16, _: 14;
    pub P17, _: 15;
}

bitfield! {
    /// Output configuration of the pins.
    /// `true` means the pin is outputing a high logic level.
    pub struct Output(u16);
    impl Debug;
    impl new;
    bool;

    pub P00, set_P00: 0;
    pub P01, set_P01: 1;
    pub P02, set_P02: 2;
    pub P03, set_P03: 3;
    pub P04, set_P04: 4;
    pub P05, set_P05: 5;
    pub P06, set_P06: 6;
    pub P07, set_P07: 7;

    pub P10, set_P10: 8;
    pub P11, set_P11: 9;
    pub P12, set_P12: 10;
    pub P13, set_P13: 11;
    pub P14, set_P14: 12;
    pub P15, set_P15: 13;
    pub P16, set_P16: 14;
    pub P17, set_P17: 15;
}

bitfield! {

    /// Configuration of the pins (intupt or output).
    ///
    /// `true` means the pin is configured as an input, `false` means it's configured as output.
    pub struct Configuration(u16);
    impl Debug;
    impl new;
    bool;

    pub P00, set_P00: 0;
    pub P01, set_P01: 1;
    pub P02, set_P02: 2;
    pub P03, set_P03: 3;
    pub P04, set_P04: 4;
    pub P05, set_P05: 5;
    pub P06, set_P06: 6;
    pub P07, set_P07: 7;

    pub P10, set_P10: 8;
    pub P11, set_P11: 9;
    pub P12, set_P12: 10;
    pub P13, set_P13: 11;
    pub P14, set_P14: 12;
    pub P15, set_P15: 13;
    pub P16, set_P16: 14;
    pub P17, set_P17: 15;
}
