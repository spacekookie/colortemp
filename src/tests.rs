//! A simple unit testing module

use colortemp::*;

#[test]
fn rgb_to_kelvin() {
    let rgb = temp_to_rgb(2500);
    assert_eq!(
        rgb,
        RGB {
            r: 255.,
            g: 159.,
            b: 70.,
        }
    );
}

#[test]
fn kelvin_to_rgb() {
    let rgb = RGB {
        r: 255.,
        g: 159.,
        b: 70.,
    };

    assert_eq!(2500, rgb_to_temp(rgb));
}