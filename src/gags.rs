use gag_types::{Gag, GagType};


pub const PASS: Gag = Gag {
    name:     "pass",
    gag_type: GagType::PassGag,
    is_org:   false,
    base_dmg: 0,
};

//pub static SOME_GAGS: [Gag; 70] = [

//];
pub const SQUIRT_GAGS: [Gag; 7] = [
    Gag { name: "squirting_flower", gag_type: GagType::SquirtGag, is_org: false, base_dmg: 4   },
    Gag { name: "glass_of_water",   gag_type: GagType::SquirtGag, is_org: false, base_dmg: 8   },
    Gag { name: "squirtgun",        gag_type: GagType::SquirtGag, is_org: false, base_dmg: 12  },
    Gag { name: "seltzer_bottle",   gag_type: GagType::SquirtGag, is_org: false, base_dmg: 21  },
    Gag { name: "fire_hose",        gag_type: GagType::SquirtGag, is_org: false, base_dmg: 30  },
    Gag { name: "storm_cloud",      gag_type: GagType::SquirtGag, is_org: false, base_dmg: 80  },
    Gag { name: "geyser",           gag_type: GagType::SquirtGag, is_org: false, base_dmg: 105 },
];
