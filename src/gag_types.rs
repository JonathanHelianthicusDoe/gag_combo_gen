use gags::SIMPLE_PASS;
use std::cmp::Ordering;
use std::mem::swap;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GagType {
    TrapGag   = 0,
    SoundGag  = 1,
    ThrowGag  = 2,
    SquirtGag = 3,
    DropGag   = 4,
    PassGag   = 5,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GagHistory(pub SimpleGag,
                      pub SimpleGag,
                      pub SimpleGag,
                      pub SimpleGag);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Gag {
    pub name:     &'static str,
    pub gag_type: GagType,
    pub is_org:   bool,
    pub base_dmg: i16,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SimpleGag {
    pub gag_type: GagType,
    pub dmg:      i16,
}


pub const GAG_TYPES: [GagType; 6] = [
    GagType::TrapGag,
    GagType::SoundGag,
    GagType::ThrowGag,
    GagType::SquirtGag,
    GagType::DropGag,
    GagType::PassGag,
];


impl PartialOrd for GagType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.as_u8().cmp(&other.as_u8()))
    }
}

impl Ord for GagType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_u8().cmp(&other.as_u8())
    }
}

impl GagType {
    pub fn as_u8(&self) -> u8 {
        match self {
            &GagType::TrapGag   => 0,
            &GagType::SoundGag  => 1,
            &GagType::ThrowGag  => 2,
            &GagType::SquirtGag => 3,
            &GagType::DropGag   => 4,
            &GagType::PassGag   => 5,
        }
    }
}

impl GagHistory {
    pub fn new() -> Self {
        GagHistory(SIMPLE_PASS, SIMPLE_PASS, SIMPLE_PASS, SIMPLE_PASS)
    }

    pub fn add_gag(&mut self, gag: &Gag) {
        let simple = gag.simple();
        if self.0 > simple {
            swap(&mut self.3, &mut self.2);
            swap(&mut self.2, &mut self.1);
            swap(&mut self.1, &mut self.0);
            self.0 = simple;
        } else if self.1 > simple {
            swap(&mut self.3, &mut self.2);
            swap(&mut self.2, &mut self.1);
            self.1 = simple;
        } else if self.2 > simple {
            swap(&mut self.3, &mut self.2);
            self.2 = simple;
        } else {
            self.3 = simple;
        }
    }
}

impl Gag {
    pub fn get_org(g: Self) -> Self {
        if g.is_org {
            g
        } else {
            Gag {
                name:     g.name,
                gag_type: g.gag_type,
                is_org:   true,
                base_dmg: Gag::org_dmg(g.base_dmg),
            }
        }
    }

    pub fn org_dmg(non_org_dmg: i16) -> i16 {
        non_org_dmg + non_org_dmg / 10
    }

    pub fn simple(&self) -> SimpleGag {
        SimpleGag {
            gag_type: self.gag_type.clone(),
            dmg:      self.base_dmg,
        }
    }
}

impl PartialOrd for SimpleGag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.gag_type.cmp(&other.gag_type) {
            Ordering::Equal => Some(self.dmg.cmp(&other.dmg)),
            o               => Some(o),
        }
    }
}

impl Ord for SimpleGag {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.gag_type.cmp(&other.gag_type) {
            Ordering::Equal => self.dmg.cmp(&other.dmg),
            o               => o,
        }
    }
}
