use crate::{
    gag_types::{Combo, Gag, GagHistory},
    gags::{hash_picks, PASS},
    hp::Hp,
};
use fxhash::{FxHashMap as Map, FxHashSet as Set};
use std::{self, collections::BinaryHeap as Heap};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LureGag {
    BlueMagnet,
    Hypno,
    OrgHypno,
    Presentation,
    OrgPresentation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Luring {
    NoLure,
    Luring(LureGag),
    Lured,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Args {
    toon_count:  u8,
    cog_hp:      Hp,
    org_count:   u8,
    gag_history: GagHistory,
}

impl LureGag {
    pub fn prop_acc(self) -> u8 {
        match self {
            Self::BlueMagnet => 60,
            Self::Hypno => 70,
            Self::OrgHypno => 80,
            Self::Presentation => 90,
            Self::OrgPresentation => 100,
        }
    }
}

impl Args {
    fn new(
        toon_count: u8,
        cog_hp: Hp,
        org_count: u8,
        gag_history: GagHistory,
    ) -> Self {
        Self {
            toon_count,
            cog_hp,
            org_count,
            gag_history,
        }
    }
}

fn use_gag(hp: &mut Hp, luring: Luring, used: &mut GagHistory, gag: &Gag) {
    used.add_gag(gag);
    hp.apply_all_gags(luring, used);
}

#[allow(clippy::too_many_arguments)]
fn k_opt(
    // Constant
    cache: &mut Map<Args, Vec<Combo>>,
    gags: &[Gag],
    luring: Luring,
    k: u8,
    // Non-constant
    toon_count: u8,
    hp: Hp,
    orgs: u8,
    used: GagHistory,
) -> Vec<Combo> {
    // Base cases
    if hp.is_dead() {
        return vec![Combo {
            cost:  0,
            picks: Vec::new(),
        }];
    }
    if toon_count == 0 {
        return Vec::new();
    }

    // Consult the cache
    let args = Args::new(toon_count, hp.clone(), orgs, used.clone());
    if let Some(cached) = cache.get(&args) {
        return cached.clone();
    }

    // Calculate recursively
    let mut k_best: Heap<Combo> = Heap::with_capacity(k as usize);
    let mut pick_hashes_found: Set<u32> = Set::default();
    for gag in gags {
        if orgs == 0 && gag.is_org {
            continue;
        }

        let child_toon_count = toon_count - 1;
        let mut child_hp = hp.clone();
        let child_orgs = orgs - gag.is_org as u8;
        let mut child_used = used.clone();
        use_gag(&mut child_hp, luring, &mut child_used, gag);

        for Combo {
            cost: child_cost,
            picks: child_picks,
        } in k_opt(
            cache,
            gags,
            luring,
            k,
            child_toon_count,
            child_hp,
            child_orgs,
            child_used,
        )
        .into_iter()
        {
            let new_cost = child_cost + gag.cost;
            if new_cost >= k_best.peek().map_or(std::i32::MAX, |c| c.cost) {
                continue;
            }

            let mut new_picks = child_picks;
            new_picks.push(gag.clone());
            new_picks.sort_unstable();
            let new_picks_hash = hash_picks(&new_picks);
            if !pick_hashes_found.insert(new_picks_hash) {
                continue;
            }

            let new_combo = Combo {
                cost:  new_cost,
                picks: new_picks,
            };

            if k_best.len() >= k as usize {
                k_best.pop();
            }
            k_best.push(new_combo);
        }
    }

    // Update cache
    let res = k_best.into_sorted_vec();
    cache.insert(args, res.clone());

    // Done
    res
}

pub fn k_opt_combos(
    k: u8,
    gags: &[Gag],
    cog_level: u8,
    luring: Luring,
    is_v2: bool,
    toon_count: u8,
    org_count: u8,
) -> Vec<Vec<Gag>> {
    // This logic is here because if the user specifies that we are currently
    // luring, then (at least) one toon has to do said luring, which means that
    // the lure gag being part of the combo is implicit and we find combos that
    // have `n - 1` gags.
    let toon_count = if let Luring::Luring(_) = luring {
        toon_count - 1
    } else {
        toon_count
    };

    let mut cache = Map::default();

    k_opt(
        &mut cache,
        gags,
        luring,
        k,
        toon_count,
        Hp::new(cog_level, is_v2),
        org_count,
        GagHistory::new(),
    )
    .into_iter()
    .map(|c| c.picks)
    .collect()
}

fn opt(
    // Constant
    cache: &mut Map<Args, Option<(i32, Gag)>>,
    gags: &[Gag],
    luring: Luring,
    // Non-constant
    toon_count: u8,
    hp: Hp,
    orgs: u8,
    used: GagHistory,
) -> Option<(i32, Gag)> {
    // Base cases
    if hp.is_dead() {
        return Some((0, PASS));
    }
    if toon_count == 0 {
        return None;
    }

    // Consult the cache
    let args = Args::new(toon_count, hp.clone(), orgs, used.clone());
    if let Some(cached) = cache.get(&args) {
        return cached.clone();
    }

    // Calculate recursively
    let mut min_cost = std::i32::MAX;
    let mut min_cost_gag = None;
    for gag in gags {
        if orgs == 0 && gag.is_org {
            continue;
        }

        let child_toon_count = toon_count - 1;
        let mut child_hp = hp.clone();
        let child_orgs = orgs - gag.is_org as u8;
        let mut child_used = used.clone();
        use_gag(&mut child_hp, luring, &mut child_used, gag);

        if let Some((child_cost, _)) = opt(
            cache,
            gags,
            luring,
            child_toon_count,
            child_hp,
            child_orgs,
            child_used,
        ) {
            let new_min_cost = child_cost + gag.cost;
            if new_min_cost < min_cost {
                min_cost = new_min_cost;
                min_cost_gag = Some(gag);
            } else if new_min_cost == min_cost {
                if let Some(mcg) = min_cost_gag {
                    if gag.cost < mcg.cost {
                        min_cost_gag = Some(gag);
                    }
                } else {
                    min_cost_gag = Some(gag);
                }
            }
        }
    }

    // Update cache
    let res = if min_cost == std::i32::MAX {
        None
    } else if let Some(g) = min_cost_gag {
        Some((min_cost, g.clone()))
    } else {
        None
    };
    cache.insert(args, res.clone());

    // Done
    res
}

pub fn opt_combo(
    gags: &[Gag],
    cog_level: u8,
    luring: Luring,
    is_v2: bool,
    toon_count: u8,
    org_count: u8,
) -> Option<Vec<Gag>> {
    // This logic is here because if the user specifies that we are currently
    // luring, then (at least) one toon has to do said luring, which means that
    // the lure gag being part of the combo is implicit and we find combos that
    // have `n - 1` gags.
    let toon_count = if let Luring::Luring(_) = luring {
        toon_count - 1
    } else {
        toon_count
    };

    let mut cache = Map::default();
    let hp = Hp::new(cog_level, is_v2);
    let mut res = Vec::with_capacity(toon_count as usize);
    if let Some((_, first_gag)) = opt(
        &mut cache,
        gags,
        luring,
        toon_count,
        hp.clone(),
        org_count,
        GagHistory::new(),
    ) {
        let mut next_gag = Some(first_gag);
        let mut args = Args::new(toon_count, hp, org_count, GagHistory::new());
        while let Some(gag) = next_gag {
            res.push(gag.clone());

            args.toon_count -= 1;
            args.org_count -= gag.is_org as u8;
            use_gag(&mut args.cog_hp, luring, &mut args.gag_history, &gag);

            if args.cog_hp.is_dead() {
                break;
            }

            next_gag = cache[&args].clone().map(|(_, g)| g);
        }

        while res.len() < toon_count as usize {
            res.push(PASS);
        }

        Some(res)
    } else {
        None
    }
}
