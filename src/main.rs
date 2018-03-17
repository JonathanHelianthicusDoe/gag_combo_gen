#![feature(collection_placement)]
#![feature(placement_in_syntax)]

mod gag_types;
mod gags;
mod opt;

extern crate fnv;

use fnv::FnvHashMap as Map;
use gag_types::Gag;
use gag_types::GagType::SquirtGag;
use gags::{PASS, SQUIRT_GAGS};
use opt::opt_combo;


fn main() {
    let mut some_gags: Map<Gag, u8> = Map::default();

    some_gags.insert(PASS, 0);
    SQUIRT_GAGS.into_iter()
               .enumerate()
               .map(|(i, gag)| ((2 * i + 1) as u8, gag))
               .for_each(|(i, gag)| {
                   some_gags.insert(gag.clone(), i);
                   some_gags.insert(Gag::get_org(gag.clone()), i + 1);
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

    println!("All test cases passed.");
}
