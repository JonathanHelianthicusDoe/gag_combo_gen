#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(deprecated)]

pub mod accuracy;
pub mod gag_types;
pub mod gags;
mod hp;
pub mod opt;

#[cfg(test)]
mod tests {
    use crate::{
        gag_types::{
            Gag,
            GagName,
            GagType::{Drop, Squirt, Throw},
        },
        gags::{DEFAULT_GAGS, PASS},
        opt::opt_combo,
    };

    #[test]
    fn pie_slice_test() {
        assert_eq!(
            opt_combo(
                DEFAULT_GAGS
                    .iter()
                    .filter(|g| g.gag_type == Throw)
                    .cloned()
                    .collect::<Vec<_>>()
                    .as_slice(),
                2,
                true,
                false,
                1,
                0,
            ),
            Some(vec![Gag {
                name:     GagName::FruitPieSlice,
                gag_type: Throw,
                is_org:   false,
                base_dmg: 10,
                cost:     160,
            }]),
        );
    }

    #[test]
    fn main_test() {
        assert_eq!(
            opt_combo(DEFAULT_GAGS, 11, true, false, 2, 1),
            Some(vec![
                Gag {
                    name:     GagName::Squirtgun,
                    gag_type: Squirt,
                    is_org:   false,
                    base_dmg: 12,
                    cost:     1_400,
                },
                Gag {
                    name:     GagName::StormCloud,
                    gag_type: Squirt,
                    is_org:   false,
                    base_dmg: 80,
                    cost:     400_000,
                },
            ]),
        );
        let squirt_gags: Vec<_> = DEFAULT_GAGS
            .iter()
            .filter(|g| g.gag_type == Squirt)
            .cloned()
            .collect();
        assert_eq!(
            opt_combo(&squirt_gags, 11, true, false, 3, 1),
            Some(vec![
                Gag {
                    name:     GagName::FireHose,
                    gag_type: Squirt,
                    is_org:   false,
                    base_dmg: 30,
                    cost:     48_000,
                },
                Gag {
                    name:     GagName::FireHose,
                    gag_type: Squirt,
                    is_org:   false,
                    base_dmg: 30,
                    cost:     48_000,
                },
                Gag {
                    name:     GagName::FireHose,
                    gag_type: Squirt,
                    is_org:   true,
                    base_dmg: 33,
                    cost:     56_000,
                },
            ]),
        );
        assert_eq!(
            opt_combo(&squirt_gags, 11, true, false, 4, 4),
            Some(vec![
                Gag {
                    name:     GagName::SeltzerBottle,
                    gag_type: Squirt,
                    is_org:   true,
                    base_dmg: 23,
                    cost:     9_000,
                },
                Gag {
                    name:     GagName::SeltzerBottle,
                    gag_type: Squirt,
                    is_org:   true,
                    base_dmg: 23,
                    cost:     9_000,
                },
                Gag {
                    name:     GagName::SeltzerBottle,
                    gag_type: Squirt,
                    is_org:   true,
                    base_dmg: 23,
                    cost:     9_000,
                },
                Gag {
                    name:     GagName::SeltzerBottle,
                    gag_type: Squirt,
                    is_org:   true,
                    base_dmg: 23,
                    cost:     9_000,
                },
            ]),
        );
        assert_eq!(
            opt_combo(&squirt_gags, 3, true, false, 4, 4),
            Some(vec![
                Gag {
                    name:     GagName::SquirtingFlower,
                    gag_type: Squirt,
                    is_org:   false,
                    base_dmg: 4,
                    cost:     15,
                },
                Gag {
                    name:     GagName::SquirtingFlower,
                    gag_type: Squirt,
                    is_org:   false,
                    base_dmg: 4,
                    cost:     15,
                },
                Gag {
                    name:     GagName::SquirtingFlower,
                    gag_type: Squirt,
                    is_org:   false,
                    base_dmg: 4,
                    cost:     15,
                },
                PASS,
            ]),
        );
        assert_eq!(
            opt_combo(DEFAULT_GAGS, 11, true, true, 3, 2),
            Some(vec![
                Gag {
                    name:     GagName::BigWeight,
                    gag_type: Drop,
                    is_org:   false,
                    base_dmg: 45,
                    cost:     6_000,
                },
                Gag {
                    name:     GagName::StormCloud,
                    gag_type: Squirt,
                    is_org:   false,
                    base_dmg: 80,
                    cost:     400_000,
                },
                Gag {
                    name:     GagName::StormCloud,
                    gag_type: Squirt,
                    is_org:   false,
                    base_dmg: 80,
                    cost:     400_000,
                },
            ]),
        );

        assert_eq!(
            opt_combo(DEFAULT_GAGS, 12, false, true, 3, 3),
            Some(vec![
                Gag {
                    name:     GagName::Anvil,
                    gag_type: Drop,
                    is_org:   false,
                    base_dmg: 30,
                    cost:     1_200,
                },
                Gag {
                    name:     GagName::GrandPiano,
                    gag_type: Drop,
                    is_org:   false,
                    base_dmg: 170,
                    cost:     740_000,
                },
                Gag {
                    name:     GagName::GrandPiano,
                    gag_type: Drop,
                    is_org:   false,
                    base_dmg: 170,
                    cost:     740_000,
                },
            ]),
        );
    }
}
