use crate::gags::SIMPLE_PASS;
use std::{cmp::Ordering, mem::swap};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GagType {
    TrapGag = 0,
    SoundGag = 1,
    ThrowGag = 2,
    SquirtGag = 3,
    DropGag = 4,
    PassGag = 5,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GagHistory(
    pub SimpleGag,
    pub SimpleGag,
    pub SimpleGag,
    pub SimpleGag,
);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Gag {
    pub name:     &'static str,
    pub gag_type: GagType,
    pub is_org:   bool,
    pub base_dmg: i16,
    pub cost:     i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SimpleGag {
    pub gag_type: GagType,
    pub dmg:      i16,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Combo(pub i32, pub Vec<Gag>);

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
        let n: u8 = (*self).into();

        Some(n.cmp(&(*other).into()))
    }
}

impl Ord for GagType {
    fn cmp(&self, other: &Self) -> Ordering {
        let n: u8 = (*self).into();

        n.cmp(&(*other).into())
    }
}

impl Into<u8> for GagType {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Default for GagHistory {
    fn default() -> Self {
        Self::new()
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
                cost:     g.cost,
            }
        }
    }

    pub fn org_dmg(non_org_dmg: i16) -> i16 {
        non_org_dmg + non_org_dmg / 10
    }

    pub fn simple(&self) -> SimpleGag {
        SimpleGag {
            gag_type: self.gag_type,
            dmg:      self.base_dmg,
        }
    }
}

impl PartialOrd for Gag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.cost.cmp(&other.cost) {
            Ordering::Equal => match self.base_dmg.cmp(&other.base_dmg) {
                Ordering::Equal => self.gag_type.cmp(&other.gag_type),
                o => o,
            },
            o => o,
        })
    }
}

impl Ord for Gag {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.cost.cmp(&other.cost) {
            Ordering::Equal => match self.base_dmg.cmp(&other.base_dmg) {
                Ordering::Equal => self.gag_type.cmp(&other.gag_type),
                o => o,
            },
            o => o,
        }
    }
}

impl PartialOrd for Combo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for Combo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for SimpleGag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.gag_type.cmp(&other.gag_type) {
            Ordering::Equal => Some(self.dmg.cmp(&other.dmg)),
            o => Some(o),
        }
    }
}

impl Ord for SimpleGag {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.gag_type.cmp(&other.gag_type) {
            Ordering::Equal => self.dmg.cmp(&other.dmg),
            o => o,
        }
    }
}
