use std::ops::{Index, IndexMut};


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GagType {
    PassGag,
    TrapGag,
    SoundGag,
    ThrowGag,
    SquirtGag,
    DropGag,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum GagUsage {
    NotUsed,
    UsedOnce(i16),
    Used(i16),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GagUsages {
    pass:   GagUsage,
    trap:   GagUsage,
    sound:  GagUsage,
    throw:  GagUsage,
    squirt: GagUsage,
    drop:   GagUsage,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Gag {
    pub name:     &'static str,
    pub gag_type: GagType,
    pub is_org:   bool,
    pub base_dmg: i16,
}


impl GagUsages {
    pub fn new() -> Self {
        GagUsages {
            pass:   GagUsage::NotUsed,
            trap:   GagUsage::NotUsed,
            sound:  GagUsage::NotUsed,
            throw:  GagUsage::NotUsed,
            squirt: GagUsage::NotUsed,
            drop:   GagUsage::NotUsed,
        }
    }
}

impl Index<GagType> for GagUsages {
    type Output = GagUsage;

    fn index(&self, t: GagType) -> &GagUsage {
        match t {
            GagType::TrapGag   => &self.trap,
            GagType::SoundGag  => &self.sound,
            GagType::ThrowGag  => &self.throw,
            GagType::SquirtGag => &self.squirt,
            GagType::DropGag   => &self.drop,
            _                  => &GagUsage::NotUsed,
        }
    }
}

impl IndexMut<GagType> for GagUsages {
    fn index_mut(&mut self, t: GagType) -> &mut GagUsage {
        match t {
            GagType::TrapGag   => &mut self.trap,
            GagType::SoundGag  => &mut self.sound,
            GagType::ThrowGag  => &mut self.throw,
            GagType::SquirtGag => &mut self.squirt,
            GagType::DropGag   => &mut self.drop,
            GagType::PassGag   => &mut self.pass,
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
}
