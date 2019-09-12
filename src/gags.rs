use crate::gag_types::{Gag, GagType, SimpleGag};
use fxhash::FxHashMap as Map;
use lazy_static::lazy_static;

pub const PASS: Gag = Gag {
    name:     "pass",
    gag_type: GagType::PassGag,
    is_org:   false,
    base_dmg: 0,
    cost:     0,
};

pub const SIMPLE_PASS: SimpleGag = SimpleGag {
    gag_type: GagType::PassGag,
    dmg:      0,
};

pub const TRAP_GAGS: [Gag; 7] = [
    Gag {
        name:     "banana_peel",
        gag_type: GagType::TrapGag,
        is_org:   false,
        base_dmg: 12,
        cost:     0,
    },
    Gag {
        name:     "rake",
        gag_type: GagType::TrapGag,
        is_org:   false,
        base_dmg: 20,
        cost:     0,
    },
    Gag {
        name:     "marbles",
        gag_type: GagType::TrapGag,
        is_org:   false,
        base_dmg: 35,
        cost:     0,
    },
    Gag {
        name:     "quicksand",
        gag_type: GagType::TrapGag,
        is_org:   false,
        base_dmg: 50,
        cost:     0,
    },
    Gag {
        name:     "trap_door",
        gag_type: GagType::TrapGag,
        is_org:   false,
        base_dmg: 70,
        cost:     0,
    },
    Gag {
        name:     "tnt",
        gag_type: GagType::TrapGag,
        is_org:   false,
        base_dmg: 180,
        cost:     0,
    },
    Gag {
        name:     "railroad",
        gag_type: GagType::TrapGag,
        is_org:   false,
        base_dmg: 195,
        cost:     0,
    },
];

pub const SOUND_GAGS: [Gag; 7] = [
    Gag {
        name:     "bikehorn",
        gag_type: GagType::SoundGag,
        is_org:   false,
        base_dmg: 4,
        cost:     0,
    },
    Gag {
        name:     "whistle",
        gag_type: GagType::SoundGag,
        is_org:   false,
        base_dmg: 7,
        cost:     0,
    },
    Gag {
        name:     "bugle",
        gag_type: GagType::SoundGag,
        is_org:   false,
        base_dmg: 11,
        cost:     0,
    },
    Gag {
        name:     "aoogah",
        gag_type: GagType::SoundGag,
        is_org:   false,
        base_dmg: 16,
        cost:     0,
    },
    Gag {
        name:     "elephant_trunk",
        gag_type: GagType::SoundGag,
        is_org:   false,
        base_dmg: 21,
        cost:     0,
    },
    Gag {
        name:     "foghorn",
        gag_type: GagType::SoundGag,
        is_org:   false,
        base_dmg: 50,
        cost:     0,
    },
    Gag {
        name:     "opera_singer",
        gag_type: GagType::SoundGag,
        is_org:   false,
        base_dmg: 90,
        cost:     0,
    },
];

pub const THROW_GAGS: [Gag; 7] = [
    Gag {
        name:     "cupcake",
        gag_type: GagType::ThrowGag,
        is_org:   false,
        base_dmg: 6,
        cost:     0,
    },
    Gag {
        name:     "fruit_pie_slice",
        gag_type: GagType::ThrowGag,
        is_org:   false,
        base_dmg: 10,
        cost:     0,
    },
    Gag {
        name:     "cream_pie_slice",
        gag_type: GagType::ThrowGag,
        is_org:   false,
        base_dmg: 17,
        cost:     0,
    },
    Gag {
        name:     "fruit_pie",
        gag_type: GagType::ThrowGag,
        is_org:   false,
        base_dmg: 27,
        cost:     0,
    },
    Gag {
        name:     "cream_pie",
        gag_type: GagType::ThrowGag,
        is_org:   false,
        base_dmg: 40,
        cost:     0,
    },
    Gag {
        name:     "cake",
        gag_type: GagType::ThrowGag,
        is_org:   false,
        base_dmg: 100,
        cost:     0,
    },
    Gag {
        name:     "wedding_cake",
        gag_type: GagType::ThrowGag,
        is_org:   false,
        base_dmg: 120,
        cost:     0,
    },
];

pub const SQUIRT_GAGS: [Gag; 7] = [
    Gag {
        name:     "squirting_flower",
        gag_type: GagType::SquirtGag,
        is_org:   false,
        base_dmg: 4,
        cost:     0,
    },
    Gag {
        name:     "glass_of_water",
        gag_type: GagType::SquirtGag,
        is_org:   false,
        base_dmg: 8,
        cost:     0,
    },
    Gag {
        name:     "squirtgun",
        gag_type: GagType::SquirtGag,
        is_org:   false,
        base_dmg: 12,
        cost:     0,
    },
    Gag {
        name:     "seltzer_bottle",
        gag_type: GagType::SquirtGag,
        is_org:   false,
        base_dmg: 21,
        cost:     0,
    },
    Gag {
        name:     "fire_hose",
        gag_type: GagType::SquirtGag,
        is_org:   false,
        base_dmg: 30,
        cost:     0,
    },
    Gag {
        name:     "storm_cloud",
        gag_type: GagType::SquirtGag,
        is_org:   false,
        base_dmg: 80,
        cost:     0,
    },
    Gag {
        name:     "geyser",
        gag_type: GagType::SquirtGag,
        is_org:   false,
        base_dmg: 105,
        cost:     0,
    },
];

pub const DROP_GAGS: [Gag; 7] = [
    Gag {
        name:     "flowerpot",
        gag_type: GagType::DropGag,
        is_org:   false,
        base_dmg: 10,
        cost:     0,
    },
    Gag {
        name:     "sandbag",
        gag_type: GagType::DropGag,
        is_org:   false,
        base_dmg: 18,
        cost:     0,
    },
    Gag {
        name:     "anvil",
        gag_type: GagType::DropGag,
        is_org:   false,
        base_dmg: 30,
        cost:     0,
    },
    Gag {
        name:     "big_weight",
        gag_type: GagType::DropGag,
        is_org:   false,
        base_dmg: 45,
        cost:     0,
    },
    Gag {
        name:     "safe",
        gag_type: GagType::DropGag,
        is_org:   false,
        base_dmg: 60,
        cost:     0,
    },
    Gag {
        name:     "grand_piano",
        gag_type: GagType::DropGag,
        is_org:   false,
        base_dmg: 170,
        cost:     0,
    },
    Gag {
        name:     "toontanic",
        gag_type: GagType::DropGag,
        is_org:   false,
        base_dmg: 180,
        cost:     0,
    },
];

lazy_static! {
    pub static ref GAG_HASHES: Map<&'static str, u32> = {
        let mut m = Map::default();

        m.insert("pass", 0);

        m.insert("banana_peel", 1);
        m.insert("rake", 6);
        m.insert("marbles", 11);
        m.insert("quicksand", 16);
        m.insert("trap_door", 21);
        m.insert("tnt", 26);
        m.insert("railroad", 31);

        m.insert("bikehorn", 2);
        m.insert("whistle", 7);
        m.insert("bugle", 12);
        m.insert("aoogah", 17);
        m.insert("elephant_trunk", 22);
        m.insert("foghorn", 27);
        m.insert("opera_singer", 32);

        m.insert("cupcake", 3);
        m.insert("fruit_pie_slice", 8);
        m.insert("cream_pie_slice", 13);
        m.insert("fruit_pie", 18);
        m.insert("cream_pie", 23);
        m.insert("cake", 28);
        m.insert("wedding_cake", 33);

        m.insert("squirting_flower", 4);
        m.insert("glass_of_water", 9);
        m.insert("squirtgun", 14);
        m.insert("seltzer_bottle", 19);
        m.insert("fire_hose", 24);
        m.insert("storm_cloud", 29);
        m.insert("geyser", 34);

        m.insert("flowerpot", 5);
        m.insert("sandbag", 10);
        m.insert("anvil", 15);
        m.insert("big_weight", 20);
        m.insert("safe", 25);
        m.insert("grand_piano", 30);
        m.insert("toontanic", 35);

        m
    };
}

pub fn hash_gag(gag: &Gag) -> u32 {
    GAG_HASHES[gag.name] + if gag.is_org { 35 } else { 0 }
}

pub fn hash_picks(picks: &[Gag]) -> u32 {
    let mut hash = 0;
    for g in picks {
        hash <<= 8;
        hash |= hash_gag(g);
    }

    hash
}
