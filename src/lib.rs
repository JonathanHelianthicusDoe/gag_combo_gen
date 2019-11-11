#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(deprecated)]

pub mod gag_types;
pub mod gags;
mod hp;
pub mod opt;

#[cfg(test)]
mod tests {
    use crate::{
        gag_types::{
            Gag,
            GagType::{DropGag, SquirtGag, ThrowGag},
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
                    .filter(|g| g.gag_type == ThrowGag)
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
                name:     "fruit_pie_slice",
                gag_type: ThrowGag,
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
                    name:     "squirtgun",
                    gag_type: SquirtGag,
                    is_org:   false,
                    base_dmg: 12,
                    cost:     1_400,
                },
                Gag {
                    name:     "storm_cloud",
                    gag_type: SquirtGag,
                    is_org:   false,
                    base_dmg: 80,
                    cost:     400_000,
                },
            ]),
        );
        let squirt_gags: Vec<_> = DEFAULT_GAGS
            .iter()
            .filter(|g| g.gag_type == SquirtGag)
            .cloned()
            .collect();
        assert_eq!(
            opt_combo(&squirt_gags, 11, true, false, 3, 1),
            Some(vec![
                Gag {
                    name:     "fire_hose",
                    gag_type: SquirtGag,
                    is_org:   false,
                    base_dmg: 30,
                    cost:     48_000,
                },
                Gag {
                    name:     "fire_hose",
                    gag_type: SquirtGag,
                    is_org:   false,
                    base_dmg: 30,
                    cost:     48_000,
                },
                Gag {
                    name:     "fire_hose",
                    gag_type: SquirtGag,
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
                    name:     "seltzer_bottle",
                    gag_type: SquirtGag,
                    is_org:   true,
                    base_dmg: 23,
                    cost:     9_000,
                },
                Gag {
                    name:     "seltzer_bottle",
                    gag_type: SquirtGag,
                    is_org:   true,
                    base_dmg: 23,
                    cost:     9_000,
                },
                Gag {
                    name:     "seltzer_bottle",
                    gag_type: SquirtGag,
                    is_org:   true,
                    base_dmg: 23,
                    cost:     9_000,
                },
                Gag {
                    name:     "seltzer_bottle",
                    gag_type: SquirtGag,
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
                    name:     "squirting_flower",
                    gag_type: SquirtGag,
                    is_org:   false,
                    base_dmg: 4,
                    cost:     15,
                },
                Gag {
                    name:     "squirting_flower",
                    gag_type: SquirtGag,
                    is_org:   false,
                    base_dmg: 4,
                    cost:     15,
                },
                Gag {
                    name:     "squirting_flower",
                    gag_type: SquirtGag,
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
                    name:     "big_weight",
                    gag_type: DropGag,
                    is_org:   false,
                    base_dmg: 45,
                    cost:     6_000,
                },
                Gag {
                    name:     "storm_cloud",
                    gag_type: SquirtGag,
                    is_org:   false,
                    base_dmg: 80,
                    cost:     400_000,
                },
                Gag {
                    name:     "storm_cloud",
                    gag_type: SquirtGag,
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
                    name:     "anvil",
                    gag_type: DropGag,
                    is_org:   false,
                    base_dmg: 30,
                    cost:     1_200,
                },
                Gag {
                    name:     "grand_piano",
                    gag_type: DropGag,
                    is_org:   false,
                    base_dmg: 170,
                    cost:     740_000,
                },
                Gag {
                    name:     "grand_piano",
                    gag_type: DropGag,
                    is_org:   false,
                    base_dmg: 170,
                    cost:     740_000,
                },
            ]),
        );
    }
}
