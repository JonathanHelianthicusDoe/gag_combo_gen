use fnv::FnvHashMap as Map;
use gag_types::{Gag, GagHistory, GagType};
use gags::{PASS, SIMPLE_PASS};
use std;


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Hp {
    pub fst_shell: i16,
    pub snd_shell: i16,
    pub fst_max:   i16,
    pub snd_max:   i16,
}


impl Hp {
    pub fn new(cog_level: u8, is_v2: bool) -> Self {
        let fst_shell = if cog_level >= 12 {
            200
        } else {
            (cog_level + 1) * (cog_level + 2)
        } as i16;

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

    pub fn apply_all_gags(&mut self, is_lured: bool, used: &GagHistory) {
        self.fst_shell = self.fst_max;
        self.snd_shell = self.snd_max;

        let mut is_lured = is_lured;
        let mut trapped = false;
        let (mut multi_sound, mut sound_dmg) = (false, 0);
        let (mut multi_throw, mut throw_dmg) = (false, 0);
        let (mut multi_squirt, mut squirt_dmg) = (false, 0);
        let (mut multi_drop, mut drop_dmg) = (false, 0);
        let gags = [&used.0, &used.1, &used.2, &used.3, &SIMPLE_PASS];
        for g in gags.into_iter() {
            if g.gag_type != GagType::SoundGag && multi_sound {
                self.do_dmg((sound_dmg - 1) / 5 + 1);
                multi_sound = false;
            }
            if g.gag_type != GagType::ThrowGag && multi_throw {
                self.do_dmg((throw_dmg - 1) / 5 + 1);
                if is_lured {
                    self.do_dmg((throw_dmg - 1) / 2 + 1);
                    is_lured = false;
                }
                multi_throw = false;
            }
            if g.gag_type != GagType::SquirtGag && multi_squirt {
                self.do_dmg((squirt_dmg - 1) / 5 + 1);
                if is_lured {
                    self.do_dmg((squirt_dmg - 1) / 2 + 1);
                    is_lured = false;
                }
                multi_squirt = false;
            }

            match g.gag_type {
                GagType::TrapGag => if is_lured {
                    if trapped {
                        self.fst_shell = self.fst_max;
                        is_lured = is_lured;
                    } else {
                        self.fst_shell -= g.dmg;
                        trapped = true;
                        is_lured = false;
                    }
                },
                GagType::SoundGag => {
                    if sound_dmg > 0 {
                        multi_sound = true;
                    }
                    self.do_dmg(g.dmg);
                    sound_dmg += g.dmg;
                    is_lured = false;
                },
                GagType::ThrowGag => {
                    if throw_dmg > 0 {
                        multi_throw = true;
                    }
                    self.do_dmg(g.dmg);
                    throw_dmg += g.dmg;
                },
                GagType::SquirtGag => {
                    if squirt_dmg > 0 {
                        multi_squirt = true;
                    }
                    self.do_dmg(g.dmg);
                    squirt_dmg += g.dmg;
                },
                GagType::DropGag => if !is_lured {
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

fn use_gag(hp: &mut Hp, is_lured: bool, used: &mut GagHistory, gag: &Gag) {
    used.add_gag(gag);
    hp.apply_all_gags(is_lured, used);
}

fn opt(// Constant
       cache:     &mut Map<(u8, Hp, u8, GagHistory), Option<(u8, Gag)>>,
       gags:      &Map<Gag, u8>,
       is_lured:  bool,
       // Non-constant
       n:         u8,
       hp:        Hp,
       orgs:      u8,
       used:      GagHistory) -> Option<(u8, Gag)>
{
    // Base cases
    if hp.is_dead() {
        return Some((0, PASS));
    }
    if n == 0 {
        return None;
    }

    // Consult the cache
    let args = (n, hp.clone(), orgs, used.clone());
    if let Some(cached) = cache.get(&args) {
        return cached.clone();
    }

    // Calculate recursively
    let mut min_max_rank = std::u8::MAX;
    let mut min_max_gag = None;
    let mut min_max_gag_rank = std::u8::MAX;
    for (gag, gag_rank) in gags {
        if orgs == 0 && gag.is_org {
            continue;
        }

        let child_n = n - 1;
        let mut child_hp = hp.clone();
        let child_orgs = orgs - gag.is_org as u8;
        let mut child_used = used.clone();
        use_gag(&mut child_hp, is_lured, &mut child_used, gag);

        if let Some((child_rank, _)) = opt(cache,
                                           gags,
                                           is_lured,
                                           child_n,
                                           child_hp,
                                           child_orgs,
                                           child_used)
        {
            let new_min_max_rank = child_rank.max(*gag_rank);
            if new_min_max_rank < min_max_rank {
                min_max_rank = new_min_max_rank;
                min_max_gag = Some(gag);
                min_max_gag_rank = *gag_rank;
            } else if new_min_max_rank == min_max_rank {
                if gag_rank < &min_max_gag_rank {
                    min_max_gag = Some(gag);
                    min_max_gag_rank = *gag_rank;
                }
            }
        }
    }

    // Update cache
    let res = if min_max_rank == std::u8::MAX {
        None
    } else if let Some(g) = min_max_gag {
        Some((min_max_rank, g.clone()))
    } else {
        None
    };
    cache.insert(args, res.clone());

    // Done
    res
}

pub fn opt_combo(gags:       &Map<Gag, u8>,
                 cog_level:  u8,
                 is_lured:   bool,
                 is_v2:      bool,
                 toon_count: u8,
                 org_count:  u8) -> Option<Vec<Gag>>
{
    let mut cache = Map::default();
    let hp = Hp::new(cog_level, is_v2);
    let mut res = Vec::with_capacity(toon_count as usize);
    if let Some((_, first_gag)) = opt(&mut cache,
                                      gags,
                                      is_lured,
                                      toon_count,
                                      hp.clone(),
                                      org_count,
                                      GagHistory::new())
    {
        let mut next_gag = Some(first_gag);
        let mut args = (toon_count, hp, org_count, GagHistory::new());
        while let Some(gag) = next_gag {
            res.place_back() <- gag.clone();

            args.0 -= 1;
            args.2 -= gag.is_org as u8;
            use_gag(&mut args.1, is_lured, &mut args.3, &gag);

            if args.1.is_dead() {
                break;
            }

            next_gag = cache[&args].clone().map(|(_, g)| g);
        }

        while res.len() < toon_count as usize {
            res.place_back() <- PASS;
        }

        Some(res)
    } else {
        None
    }
}
