use crate::gags::SIMPLE_PASS;
use std::{cmp::Ordering, mem};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GagType {
    TrapGag = 0,
    SoundGag = 1,
    ThrowGag = 2,
    SquirtGag = 3,
    DropGag = 4,
    PassGag = 5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GagName {
    Pass = 0,

    BananaPeel = 1,
    Rake = 6,
    Marbles = 11,
    Quicksand = 16,
    TrapDoor = 21,
    Tnt = 26,
    Railroad = 31,

    Bikehorn = 2,
    Whistle = 7,
    Bugle = 12,
    Aoogah = 17,
    ElephantTrunk = 22,
    Foghorn = 27,
    OperaSinger = 32,

    Cupcake = 3,
    FruitPieSlice = 8,
    CreamPieSlice = 13,
    FruitPie = 18,
    CreamPie = 23,
    Cake = 28,
    WeddingCake = 33,

    SquirtingFlower = 4,
    GlassOfWater = 9,
    Squirtgun = 14,
    SeltzerBottle = 19,
    FireHose = 24,
    StormCloud = 29,
    Geyser = 34,

    Flowerpot = 5,
    Sandbag = 10,
    Anvil = 15,
    BigWeight = 20,
    Safe = 25,
    GrandPiano = 30,
    Toontanic = 35,
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
    pub name:     GagName,
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
pub struct Combo {
    pub cost:  i32,
    pub picks: Vec<Gag>,
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
            mem::swap(&mut self.3, &mut self.2);
            mem::swap(&mut self.2, &mut self.1);
            mem::swap(&mut self.1, &mut self.0);
            self.0 = simple;
        } else if self.1 > simple {
            mem::swap(&mut self.3, &mut self.2);
            mem::swap(&mut self.2, &mut self.1);
            self.1 = simple;
        } else if self.2 > simple {
            mem::swap(&mut self.3, &mut self.2);
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
        non_org_dmg + (non_org_dmg / 10).max(1)
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
        Some(self.cost.cmp(&other.cost))
    }
}

impl Ord for Combo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
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
