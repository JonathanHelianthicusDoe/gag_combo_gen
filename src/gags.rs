use gag_types::{Gag, GagType, SimpleGag};


pub const PASS: Gag = Gag {
    name:     "pass",
    gag_type: GagType::PassGag,
    is_org:   false,
    base_dmg: 0,
};
pub const SIMPLE_PASS: SimpleGag = SimpleGag {
    gag_type: GagType::PassGag,
    dmg:      0,
};
pub const SQUIRT_GAGS: [Gag; 7] = [
    Gag { name: "squirting_flower", gag_type: GagType::SquirtGag, is_org: false, base_dmg: 4   },
    Gag { name: "glass_of_water",   gag_type: GagType::SquirtGag, is_org: false, base_dmg: 8   },
    Gag { name: "squirtgun",        gag_type: GagType::SquirtGag, is_org: false, base_dmg: 12  },
    Gag { name: "seltzer_bottle",   gag_type: GagType::SquirtGag, is_org: false, base_dmg: 21  },
    Gag { name: "fire_hose",        gag_type: GagType::SquirtGag, is_org: false, base_dmg: 30  },
    Gag { name: "storm_cloud",      gag_type: GagType::SquirtGag, is_org: false, base_dmg: 80  },
    Gag { name: "geyser",           gag_type: GagType::SquirtGag, is_org: false, base_dmg: 105 },
];
pub const DROP_GAGS: [Gag; 7] = [
    Gag { name: "flowerpot",   gag_type: GagType::DropGag, is_org: false, base_dmg: 10  },
    Gag { name: "sandbag",     gag_type: GagType::DropGag, is_org: false, base_dmg: 18  },
    Gag { name: "anvil",       gag_type: GagType::DropGag, is_org: false, base_dmg: 30  },
    Gag { name: "big_weight",  gag_type: GagType::DropGag, is_org: false, base_dmg: 45  },
    Gag { name: "safe",        gag_type: GagType::DropGag, is_org: false, base_dmg: 60  },
    Gag { name: "grand_piano", gag_type: GagType::DropGag, is_org: false, base_dmg: 170 },
    Gag { name: "toontanic",   gag_type: GagType::DropGag, is_org: false, base_dmg: 180 },
];
