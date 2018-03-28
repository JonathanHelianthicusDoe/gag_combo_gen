use gag_types::{Gag, GagType, SimpleGag};


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
    Gag { name: "banana_peel", gag_type: GagType::TrapGag, is_org: false, base_dmg: 12,  cost: 0 },
    Gag { name: "rake",        gag_type: GagType::TrapGag, is_org: false, base_dmg: 20,  cost: 0 },
    Gag { name: "marbles",     gag_type: GagType::TrapGag, is_org: false, base_dmg: 35,  cost: 0 },
    Gag { name: "quicksand",   gag_type: GagType::TrapGag, is_org: false, base_dmg: 50,  cost: 0 },
    Gag { name: "trap_door",   gag_type: GagType::TrapGag, is_org: false, base_dmg: 70,  cost: 0 },
    Gag { name: "tnt",         gag_type: GagType::TrapGag, is_org: false, base_dmg: 180, cost: 0 },
    Gag { name: "railroad",    gag_type: GagType::TrapGag, is_org: false, base_dmg: 195, cost: 0 },
];
pub const SOUND_GAGS: [Gag; 7] = [
    Gag { name: "bikehorn",       gag_type: GagType::SoundGag, is_org: false, base_dmg: 4,  cost: 0 },
    Gag { name: "whistle",        gag_type: GagType::SoundGag, is_org: false, base_dmg: 7,  cost: 0 },
    Gag { name: "bugle",          gag_type: GagType::SoundGag, is_org: false, base_dmg: 11, cost: 0 },
    Gag { name: "aoogah",         gag_type: GagType::SoundGag, is_org: false, base_dmg: 16, cost: 0 },
    Gag { name: "elephant_trunk", gag_type: GagType::SoundGag, is_org: false, base_dmg: 21, cost: 0 },
    Gag { name: "foghorn",        gag_type: GagType::SoundGag, is_org: false, base_dmg: 50, cost: 0 },
    Gag { name: "opera_singer",   gag_type: GagType::SoundGag, is_org: false, base_dmg: 90, cost: 0 },
];
pub const THROW_GAGS: [Gag; 7] = [
    Gag { name: "cupcake",         gag_type: GagType::ThrowGag, is_org: false, base_dmg: 6,   cost: 0 },
    Gag { name: "fruit_pie_slice", gag_type: GagType::ThrowGag, is_org: false, base_dmg: 10,  cost: 0 },
    Gag { name: "cream_pie_slice", gag_type: GagType::ThrowGag, is_org: false, base_dmg: 17,  cost: 0 },
    Gag { name: "fruit_pie",       gag_type: GagType::ThrowGag, is_org: false, base_dmg: 27,  cost: 0 },
    Gag { name: "cream_pie",       gag_type: GagType::ThrowGag, is_org: false, base_dmg: 40,  cost: 0 },
    Gag { name: "cake",            gag_type: GagType::ThrowGag, is_org: false, base_dmg: 100, cost: 0 },
    Gag { name: "wedding_cake",    gag_type: GagType::ThrowGag, is_org: false, base_dmg: 120, cost: 0 },
];
pub const SQUIRT_GAGS: [Gag; 7] = [
    Gag { name: "squirting_flower", gag_type: GagType::SquirtGag, is_org: false, base_dmg: 4,   cost: 0 },
    Gag { name: "glass_of_water",   gag_type: GagType::SquirtGag, is_org: false, base_dmg: 8,   cost: 0 },
    Gag { name: "squirtgun",        gag_type: GagType::SquirtGag, is_org: false, base_dmg: 12,  cost: 0 },
    Gag { name: "seltzer_bottle",   gag_type: GagType::SquirtGag, is_org: false, base_dmg: 21,  cost: 0 },
    Gag { name: "fire_hose",        gag_type: GagType::SquirtGag, is_org: false, base_dmg: 30,  cost: 0 },
    Gag { name: "storm_cloud",      gag_type: GagType::SquirtGag, is_org: false, base_dmg: 80,  cost: 0 },
    Gag { name: "geyser",           gag_type: GagType::SquirtGag, is_org: false, base_dmg: 105, cost: 0 },
];
pub const DROP_GAGS: [Gag; 7] = [
    Gag { name: "flowerpot",   gag_type: GagType::DropGag, is_org: false, base_dmg: 10,  cost: 0 },
    Gag { name: "sandbag",     gag_type: GagType::DropGag, is_org: false, base_dmg: 18,  cost: 0 },
    Gag { name: "anvil",       gag_type: GagType::DropGag, is_org: false, base_dmg: 30,  cost: 0 },
    Gag { name: "big_weight",  gag_type: GagType::DropGag, is_org: false, base_dmg: 45,  cost: 0 },
    Gag { name: "safe",        gag_type: GagType::DropGag, is_org: false, base_dmg: 60,  cost: 0 },
    Gag { name: "grand_piano", gag_type: GagType::DropGag, is_org: false, base_dmg: 170, cost: 0 },
    Gag { name: "toontanic",   gag_type: GagType::DropGag, is_org: false, base_dmg: 180, cost: 0 },
];
