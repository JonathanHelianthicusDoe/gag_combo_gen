#![feature(collection_placement)]
#![feature(placement_in_syntax)]

mod gag_types;
mod gags;
mod hp;
mod opt;

extern crate fnv;


#[cfg(test)]
mod tests {
    use gag_types::Gag;
    use gag_types::GagType::{DropGag, SquirtGag};
    use gags::{DROP_GAGS, PASS, SQUIRT_GAGS};
    use opt::opt_combo;

    #[test]
    fn main_test() {
        let mut some_gags = Vec::with_capacity(5 * 7 * 2);
        some_gags.place_back() <- PASS;
        SQUIRT_GAGS.into_iter()
                .for_each(|gag| {
                    some_gags.place_back() <- gag.clone();
                    some_gags.place_back() <- Gag::get_org(gag.clone());
                });

        assert_eq!(
            opt_combo(&some_gags, 11, true, false, 2, 1),
            Some(vec![
                Gag { name: "squirtgun",   gag_type: SquirtGag, is_org: false, base_dmg: 12 },
                Gag { name: "storm_cloud", gag_type: SquirtGag, is_org: false, base_dmg: 80 },
            ])
        );
        assert_eq!(
            opt_combo(&some_gags, 11, true, false, 3, 1),
            Some(vec![
                Gag { name: "fire_hose", gag_type: SquirtGag, is_org: false, base_dmg: 30 },
                Gag { name: "fire_hose", gag_type: SquirtGag, is_org: false, base_dmg: 30 },
                Gag { name: "fire_hose", gag_type: SquirtGag, is_org: true,  base_dmg: 33 },
            ])
        );
        assert_eq!(
            opt_combo(&some_gags, 11, true, false, 4, 4),
            Some(vec![
                Gag { name: "seltzer_bottle", gag_type: SquirtGag, is_org: true, base_dmg: 23 },
                Gag { name: "seltzer_bottle", gag_type: SquirtGag, is_org: true, base_dmg: 23 },
                Gag { name: "seltzer_bottle", gag_type: SquirtGag, is_org: true, base_dmg: 23 },
                Gag { name: "seltzer_bottle", gag_type: SquirtGag, is_org: true, base_dmg: 23 },
            ])
        );
        assert_eq!(
            opt_combo(&some_gags, 3, true, false, 4, 4),
            Some(vec![
                PASS,
                Gag { name: "squirting_flower", gag_type: SquirtGag, is_org: false, base_dmg: 4 },
                Gag { name: "squirting_flower", gag_type: SquirtGag, is_org: false, base_dmg: 4 },
                Gag { name: "squirting_flower", gag_type: SquirtGag, is_org: false, base_dmg: 4 },
            ])
        );
        assert_eq!(
            opt_combo(&some_gags, 11, true, true, 3, 3),
            Some(vec![
                Gag { name: "storm_cloud", gag_type: SquirtGag, is_org: false, base_dmg: 80 },
                Gag { name: "storm_cloud", gag_type: SquirtGag, is_org: false, base_dmg: 80 },
                Gag { name: "storm_cloud", gag_type: SquirtGag, is_org: false, base_dmg: 80 },
            ])
        );

        let mut some_gags = Vec::with_capacity(5 * 7 * 2);
        some_gags.place_back() <- PASS;
        DROP_GAGS.into_iter()
                .zip(SQUIRT_GAGS.into_iter())
                .for_each(|(drop, squirt)| {
                    some_gags.place_back() <- drop.clone();
                    some_gags.place_back() <- Gag::get_org(drop.clone());
                    some_gags.place_back() <- squirt.clone();
                    some_gags.place_back() <- Gag::get_org(squirt.clone());
                });

        assert_eq!(
            opt_combo(&some_gags, 12, false, true, 3, 3),
            Some(vec![
                Gag { name: "anvil",       gag_type: DropGag, is_org: false, base_dmg: 30  },
                Gag { name: "grand_piano", gag_type: DropGag, is_org: false, base_dmg: 170 },
                Gag { name: "grand_piano", gag_type: DropGag, is_org: false, base_dmg: 170 },
            ])
        );
    }
}
