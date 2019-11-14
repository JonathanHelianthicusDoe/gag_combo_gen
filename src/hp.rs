use crate::{
    gag_types::{GagHistory, GagType},
    gags::SIMPLE_PASS,
    opt::Luring,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Hp {
    pub fst_shell: i16,
    pub snd_shell: i16,
    pub fst_max:   i16,
    pub snd_max:   i16,
}

impl Hp {
    pub fn new(cog_level: u8, is_v2: bool) -> Self {
        let fst_shell = i16::from(if cog_level >= 12 {
            200
        } else {
            (cog_level + 1) * (cog_level + 2)
        });

        let snd_shell = if is_v2 { fst_shell } else { 0 };

        Hp {
            fst_shell,
            snd_shell,
            fst_max: fst_shell,
            snd_max: snd_shell,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.fst_shell <= 0 && self.snd_shell <= 0
    }

    pub fn do_dmg(&mut self, dmg: i16) {
        if self.fst_shell > 0 {
            self.fst_shell -= dmg;
        } else if self.snd_shell > 0 {
            self.snd_shell -= dmg;
        }
    }

    pub fn apply_all_gags(&mut self, luring: Luring, used: &GagHistory) {
        self.fst_shell = self.fst_max;
        self.snd_shell = self.snd_max;

        let mut is_lured = luring != Luring::NoLure;
        let mut trapped = false;
        let (mut multi_sound, mut sound_dmg) = (false, 0);
        let (mut multi_throw, mut throw_dmg) = (false, 0);
        let (mut multi_squirt, mut squirt_dmg) = (false, 0);
        let (mut multi_drop, mut drop_dmg) = (false, 0);
        let gags = [&used.0, &used.1, &used.2, &used.3, &SIMPLE_PASS];
        for g in gags.iter() {
            if g.gag_type != GagType::Sound && multi_sound {
                self.do_dmg((sound_dmg - 1) / 5 + 1);
                multi_sound = false;
            }
            if g.gag_type != GagType::Throw {
                if multi_throw {
                    self.do_dmg((throw_dmg - 1) / 5 + 1);
                    multi_throw = false;
                }
                if is_lured && throw_dmg != 0 {
                    self.do_dmg((throw_dmg - 1) / 2 + 1);
                    is_lured = false;
                }
            }
            if g.gag_type != GagType::Squirt {
                if multi_squirt {
                    self.do_dmg((squirt_dmg - 1) / 5 + 1);
                    multi_squirt = false;
                }
                if is_lured && squirt_dmg != 0 {
                    self.do_dmg((squirt_dmg - 1) / 2 + 1);
                    is_lured = false;
                }
            }

            match g.gag_type {
                GagType::Trap =>
                // Trap only works if we are luring, not if the cog is
                // already lured!
                    if let Luring::Luring(_) = luring {
                        if trapped {
                            self.fst_shell = self.fst_max;
                            is_lured = true;
                        } else {
                            self.fst_shell -= g.dmg;
                            trapped = true;
                            is_lured = false;
                        }
                    },
                GagType::Sound => {
                    if sound_dmg > 0 {
                        multi_sound = true;
                    }
                    self.do_dmg(g.dmg);
                    sound_dmg += g.dmg;
                    is_lured = false;
                },
                GagType::Throw => {
                    if throw_dmg > 0 {
                        multi_throw = true;
                    }
                    self.do_dmg(g.dmg);
                    throw_dmg += g.dmg;
                },
                GagType::Squirt => {
                    if squirt_dmg > 0 {
                        multi_squirt = true;
                    }
                    self.do_dmg(g.dmg);
                    squirt_dmg += g.dmg;
                },
                GagType::Drop =>
                    if !is_lured {
                        if drop_dmg > 0 {
                            multi_drop = true;
                        }
                        self.do_dmg(g.dmg);
                        drop_dmg += g.dmg;
                    },
                _ => break,
            }
        }

        if multi_drop {
            self.do_dmg((drop_dmg - 1) / 5 + 1);
        }
    }
}
