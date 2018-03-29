#![feature(collection_placement)]
#![feature(placement_in_syntax)]

pub mod gag_types;
pub mod gags;
mod hp;
pub mod opt;

extern crate fnv;
#[macro_use]
extern crate lazy_static;


#[cfg(test)]
mod tests {
    use gag_types::Gag;
    use gag_types::GagType::{DropGag, SoundGag, SquirtGag, ThrowGag, TrapGag};
    use gags::{
        DROP_GAGS, PASS, SOUND_GAGS, SQUIRT_GAGS, THROW_GAGS, TRAP_GAGS,
    };
    use opt::opt_combo;

    #[test]
    fn pie_slice_test() {
        println!();

        let ref default_gags: Vec<Gag> = vec![
            PASS,

            Gag { name: "banana_peel",      gag_type: TrapGag,   is_org: false, base_dmg: 12,  cost: 10      },
            Gag { name: "bikehorn",         gag_type: SoundGag,  is_org: false, base_dmg: 4,   cost: 12      },
            Gag { name: "flowerpot",        gag_type: DropGag,   is_org: false, base_dmg: 10,  cost: 14      },
            Gag { name: "squirting_flower", gag_type: SquirtGag, is_org: false, base_dmg: 4,   cost: 15      },
            Gag { name: "cupcake",          gag_type: ThrowGag,  is_org: false, base_dmg: 6,   cost: 16      },

            Gag { name: "banana_peel",      gag_type: TrapGag,   is_org: true,  base_dmg: 13,  cost: 15      },
            Gag { name: "bikehorn",         gag_type: SoundGag,  is_org: true,  base_dmg: 4,   cost: 17      },
            Gag { name: "flowerpot",        gag_type: DropGag,   is_org: true,  base_dmg: 11,  cost: 19      },
            Gag { name: "squirting_flower", gag_type: SquirtGag, is_org: true,  base_dmg: 4,   cost: 20      },
            Gag { name: "cupcake",          gag_type: ThrowGag,  is_org: true,  base_dmg: 6,   cost: 21      },

            Gag { name: "rake",             gag_type: TrapGag,   is_org: false, base_dmg: 20,  cost: 100     },
            Gag { name: "whistle",          gag_type: SoundGag,  is_org: false, base_dmg: 7,   cost: 110     },
            Gag { name: "sandbag",          gag_type: DropGag,   is_org: false, base_dmg: 18,  cost: 120     },
            Gag { name: "glass_of_water",   gag_type: SquirtGag, is_org: false, base_dmg: 8,   cost: 140     },
            Gag { name: "fruit_pie_slice",  gag_type: ThrowGag,  is_org: false, base_dmg: 10,  cost: 160     },

            Gag { name: "rake",             gag_type: TrapGag,   is_org: true,  base_dmg: 22,  cost: 150     },
            Gag { name: "whistle",          gag_type: SoundGag,  is_org: true,  base_dmg: 7,   cost: 160     },
            Gag { name: "sandbag",          gag_type: DropGag,   is_org: true,  base_dmg: 19,  cost: 170     },
            Gag { name: "glass_of_water",   gag_type: SquirtGag, is_org: true,  base_dmg: 8,   cost: 190     },
            Gag { name: "fruit_pie_slice",  gag_type: ThrowGag,  is_org: true,  base_dmg: 11,  cost: 210     },

            Gag { name: "marbles",          gag_type: TrapGag,   is_org: false, base_dmg: 35,  cost: 1000    },
            Gag { name: "bugle",            gag_type: SoundGag,  is_org: false, base_dmg: 11,  cost: 1100    },
            Gag { name: "anvil",            gag_type: DropGag,   is_org: false, base_dmg: 30,  cost: 1200    },
            Gag { name: "squirtgun",        gag_type: SquirtGag, is_org: false, base_dmg: 12,  cost: 1400    },
            Gag { name: "cream_pie_slice",  gag_type: ThrowGag,  is_org: false, base_dmg: 17,  cost: 1600    },

            Gag { name: "marbles",          gag_type: TrapGag,   is_org: true,  base_dmg: 38,  cost: 1500    },
            Gag { name: "bugle",            gag_type: SoundGag,  is_org: true,  base_dmg: 12,  cost: 1600    },
            Gag { name: "anvil",            gag_type: DropGag,   is_org: true,  base_dmg: 33,  cost: 1700    },
            Gag { name: "squirtgun",        gag_type: SquirtGag, is_org: true,  base_dmg: 13,  cost: 1900    },
            Gag { name: "cream_pie_slice",  gag_type: ThrowGag,  is_org: true,  base_dmg: 18,  cost: 2100    },

            Gag { name: "aoogah",           gag_type: SoundGag,  is_org: false, base_dmg: 16,  cost: 4000    },
            Gag { name: "quicksand",        gag_type: TrapGag,   is_org: false, base_dmg: 50,  cost: 4500    },
            Gag { name: "big_weight",       gag_type: DropGag,   is_org: false, base_dmg: 45,  cost: 6000    },
            Gag { name: "seltzer_bottle",   gag_type: SquirtGag, is_org: false, base_dmg: 21,  cost: 7000    },
            Gag { name: "fruit_pie",        gag_type: ThrowGag,  is_org: false, base_dmg: 27,  cost: 8000    },

            Gag { name: "aoogah",           gag_type: SoundGag,  is_org: true,  base_dmg: 17,  cost: 6000    },
            Gag { name: "quicksand",        gag_type: TrapGag,   is_org: true,  base_dmg: 55,  cost: 6500    },
            Gag { name: "big_weight",       gag_type: DropGag,   is_org: true,  base_dmg: 49,  cost: 8000    },
            Gag { name: "seltzer_bottle",   gag_type: SquirtGag, is_org: true,  base_dmg: 23,  cost: 9000    },
            Gag { name: "fruit_pie",        gag_type: ThrowGag,  is_org: true,  base_dmg: 29,  cost: 10000   },

            Gag { name: "elephant_trunk",   gag_type: SoundGag,  is_org: false, base_dmg: 21,  cost: 12000   },
            Gag { name: "trap_door",        gag_type: TrapGag,   is_org: false, base_dmg: 70,  cost: 18000   },
            Gag { name: "safe",             gag_type: DropGag,   is_org: false, base_dmg: 60,  cost: 32000   },
            Gag { name: "fire_hose",        gag_type: SquirtGag, is_org: false, base_dmg: 30,  cost: 48000   },
            Gag { name: "cream_pie",        gag_type: ThrowGag,  is_org: false, base_dmg: 40,  cost: 50000   },

            Gag { name: "elephant_trunk",   gag_type: SoundGag,  is_org: true,  base_dmg: 23,  cost: 16000   },
            Gag { name: "trap_door",        gag_type: TrapGag,   is_org: true,  base_dmg: 77,  cost: 24000   },
            Gag { name: "safe",             gag_type: DropGag,   is_org: true,  base_dmg: 66,  cost: 36000   },
            Gag { name: "fire_hose",        gag_type: SquirtGag, is_org: true,  base_dmg: 33,  cost: 56000   },
            Gag { name: "cream_pie",        gag_type: ThrowGag,  is_org: true,  base_dmg: 44,  cost: 60000   },

            Gag { name: "storm_cloud",      gag_type: SquirtGag, is_org: false, base_dmg: 80,  cost: 400000  },
            Gag { name: "cake",             gag_type: ThrowGag,  is_org: false, base_dmg: 100, cost: 500000  },
            Gag { name: "grand_piano",      gag_type: DropGag,   is_org: false, base_dmg: 170, cost: 640000  },
            Gag { name: "tnt",              gag_type: TrapGag,   is_org: false, base_dmg: 180, cost: 666000  },
            Gag { name: "foghorn",          gag_type: SoundGag,  is_org: false, base_dmg: 50,  cost: 700000  },

            Gag { name: "storm_cloud",      gag_type: SquirtGag, is_org: true,  base_dmg: 88,  cost: 470000  },
            Gag { name: "cake",             gag_type: ThrowGag,  is_org: true,  base_dmg: 110, cost: 600000  },
            Gag { name: "grand_piano",      gag_type: DropGag,   is_org: true,  base_dmg: 187, cost: 680000  },
            Gag { name: "tnt",              gag_type: TrapGag,   is_org: true,  base_dmg: 198, cost: 690000  },
            Gag { name: "foghorn",          gag_type: SoundGag,  is_org: true,  base_dmg: 55,  cost: 820000  },

            Gag { name: "geyser",           gag_type: SquirtGag, is_org: false, base_dmg: 105, cost: 4000000 },
            Gag { name: "opera_singer",     gag_type: SoundGag,  is_org: false, base_dmg: 90,  cost: 6000000 },
            Gag { name: "wedding_cake",     gag_type: ThrowGag,  is_org: false, base_dmg: 120, cost: 6010000 },
            Gag { name: "toontanic",        gag_type: DropGag,   is_org: false, base_dmg: 180, cost: 7000000 },
            Gag { name: "railroad",         gag_type: TrapGag,   is_org: false, base_dmg: 195, cost: 7010000 },

            Gag { name: "geyser",           gag_type: SquirtGag, is_org: true,  base_dmg: 115, cost: 4500000 },
            Gag { name: "opera_singer",     gag_type: SoundGag,  is_org: true,  base_dmg: 99,  cost: 6500000 },
            Gag { name: "wedding_cake",     gag_type: ThrowGag,  is_org: true,  base_dmg: 132, cost: 6510000 },
            Gag { name: "toontanic",        gag_type: DropGag,   is_org: true,  base_dmg: 198, cost: 7010000 },
            Gag { name: "railroad",         gag_type: TrapGag,   is_org: true,  base_dmg: 214, cost: 7777777 },
        ];

        assert_eq!(
            opt_combo(&default_gags.iter().filter(|g| g.gag_type == ThrowGag).cloned().collect(), 2, true, false, 1, 0),
            Some(vec![
                Gag { name: "fruit_pie_slice", gag_type: ThrowGag, is_org: false, base_dmg: 10, cost: 160 },
            ])
        );
    }

    /*
    #[test]
    fn test_test() {
        println!();

        for i in 0..7 {
            println!("{:?}", DROP_GAGS[i]);
            println!("{:?}", TRAP_GAGS[i]);
            println!("{:?}", SOUND_GAGS[i]);
            println!("{:?}", SQUIRT_GAGS[i]);
            println!("{:?}", THROW_GAGS[i]);

            println!("{:?}", Gag::get_org(DROP_GAGS[i].clone()));
            println!("{:?}", Gag::get_org(TRAP_GAGS[i].clone()));
            println!("{:?}", Gag::get_org(SOUND_GAGS[i].clone()));
            println!("{:?}", Gag::get_org(SQUIRT_GAGS[i].clone()));
            println!("{:?}", Gag::get_org(THROW_GAGS[i].clone()));
        }

        panic!()
    }

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
    */
}
