use crate::{
    gag_types::{Combo, Gag, GagHistory},
    gags::{hash_picks, PASS},
    hp::Hp,
};
use fxhash::{FxHashMap as Map, FxHashSet as Set};
use std::{self, collections::BinaryHeap as Heap};

fn use_gag(hp: &mut Hp, is_lured: bool, used: &mut GagHistory, gag: &Gag) {
    used.add_gag(gag);
    hp.apply_all_gags(is_lured, used);
}

fn k_opt(
    // Constant
    cache: &mut Map<(u8, Hp, u8, GagHistory), Vec<Combo>>,
    gags: &Vec<Gag>,
    is_lured: bool,
    k: u8,
    // Non-constant
    n: u8,
    hp: Hp,
    orgs: u8,
    used: GagHistory,
) -> Vec<Combo> {
    // Base cases
    if hp.is_dead() {
        return vec![Combo(0, Vec::new())];
    }
    if n == 0 {
        return Vec::new();
    }

    // Consult the cache
    let args = (n, hp.clone(), orgs, used.clone());
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

        let child_n = n - 1;
        let mut child_hp = hp.clone();
        let child_orgs = orgs - gag.is_org as u8;
        let mut child_used = used.clone();
        use_gag(&mut child_hp, is_lured, &mut child_used, gag);

        for Combo(child_cost, child_picks) in k_opt(
            cache, gags, is_lured, k, child_n, child_hp, child_orgs,
            child_used,
        )
        .into_iter()
        {
            let new_cost = child_cost + gag.cost;
            if new_cost >= k_best.peek().map_or(std::i32::MAX, |c| c.0) {
                continue;
            }

            let mut new_picks = child_picks;
            new_picks.push(gag.clone());
            new_picks.sort_unstable();
            let new_picks_hash = hash_picks(&new_picks);
            if !pick_hashes_found.insert(new_picks_hash) {
                continue;
            }

            let new_combo = Combo(new_cost, new_picks);

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
    gags: &Vec<Gag>,
    cog_level: u8,
    is_lured: bool,
    is_v2: bool,
    toon_count: u8,
    org_count: u8,
) -> Vec<Vec<Gag>> {
    let mut cache = Map::default();

    k_opt(
        &mut cache,
        gags,
        is_lured,
        k,
        toon_count,
        Hp::new(cog_level, is_v2),
        org_count,
        GagHistory::new(),
    )
    .into_iter()
    .map(|c| c.1)
    .collect()
}

fn opt(
    // Constant
    cache: &mut Map<(u8, Hp, u8, GagHistory), Option<(i32, Gag)>>,
    gags: &Vec<Gag>,
    is_lured: bool,
    // Non-constant
    n: u8,
    hp: Hp,
    orgs: u8,
    used: GagHistory,
) -> Option<(i32, Gag)> {
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
    let mut min_cost = std::i32::MAX;
    let mut min_cost_gag = None;
    for gag in gags {
        if orgs == 0 && gag.is_org {
            continue;
        }

        let child_n = n - 1;
        let mut child_hp = hp.clone();
        let child_orgs = orgs - gag.is_org as u8;
        let mut child_used = used.clone();
        use_gag(&mut child_hp, is_lured, &mut child_used, gag);

        if let Some((child_cost, _)) = opt(
            cache, gags, is_lured, child_n, child_hp, child_orgs, child_used,
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
    gags: &Vec<Gag>,
    cog_level: u8,
    is_lured: bool,
    is_v2: bool,
    toon_count: u8,
    org_count: u8,
) -> Option<Vec<Gag>> {
    let mut cache = Map::default();
    let hp = Hp::new(cog_level, is_v2);
    let mut res = Vec::with_capacity(toon_count as usize);
    if let Some((_, first_gag)) = opt(
        &mut cache,
        gags,
        is_lured,
        toon_count,
        hp.clone(),
        org_count,
        GagHistory::new(),
    ) {
        let mut next_gag = Some(first_gag);
        let mut args = (toon_count, hp, org_count, GagHistory::new());
        while let Some(gag) = next_gag {
            res.push(gag.clone());

            args.0 -= 1;
            args.2 -= gag.is_org as u8;
            use_gag(&mut args.1, is_lured, &mut args.3, &gag);

            if args.1.is_dead() {
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
