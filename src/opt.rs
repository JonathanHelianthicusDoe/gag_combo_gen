use fnv::FnvHashMap as Map;
use gag_types::{Gag, GagUsage, GagUsages};
use gags::{PASS};
use std;


fn dmg(gag: Gag, is_lured: bool, used: &GagUsages) -> i16 {
    let mut res = gag.base_dmg;

    if is_lured {
        res += (res - 1) / 2 + 1;
    }

    res += match used[gag.gag_type] {
        GagUsage::UsedOnce(d) => (d + gag.base_dmg - 1) / 5 + 1,
        GagUsage::Used(d)     => (d + gag.base_dmg - 1) / 5 + 1 -
                                     ((d - 1) / 5 + 1),
        _                     => 0,
    };

    res
}

fn update_usages(used: &mut GagUsages, gag: &Gag) {
    let ref mut u = used[gag.gag_type.clone()];
    match u {
        &mut GagUsage::NotUsed     => *u = GagUsage::UsedOnce(gag.base_dmg),
        &mut GagUsage::UsedOnce(d) => *u = GagUsage::Used(d + gag.base_dmg),
        &mut GagUsage::Used(d)     => *u = GagUsage::Used(d + gag.base_dmg),
    }
}

fn opt(// Constant
       cache:     &mut Map<(u8, i16, u8, GagUsages), Option<(u8, Gag)>>,
       gags:      &Map<Gag, u8>,
       is_lured:  bool,
       // Non-constant
       n:         u8,
       hp:        i16,
       orgs:      u8,
       used:      GagUsages) -> Option<(u8, Gag)>
{
    // Base cases
    if hp <= 0 {
        return Some((0, PASS));
    }
    if n == 0 {
        return None;
    }

    // Consult the cache
    let args = (n, hp, orgs, used.clone());
    if let Some(cached) = cache.get(&args) {
        return cached.clone();
    }

    // Calculate recursively
    let mut min_max_rank = std::u8::MAX;
    //let mut min_max_args = None;
    let mut min_max_gag = None;
    let mut min_max_gag_rank = std::u8::MAX;
    for (gag, gag_rank) in gags {
        if orgs == 0 && gag.is_org {
            continue;
        }

        let mut child_used = used.clone();
        update_usages(&mut child_used, &gag);
        let child_n = n - 1;
        let child_hp = hp - dmg(gag.clone(), is_lured, &used);
        let child_orgs = orgs - if gag.is_org { 1 } else { 0 };

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
                /*
                min_max_args = Some((child_n,
                                     child_hp,
                                     child_orgs,
                                     child_used));
                */
                min_max_gag = Some(gag);
                min_max_gag_rank = *gag_rank;
            } else if new_min_max_rank == min_max_rank {
                if gag_rank < &min_max_gag_rank {
                    /*
                    min_max_args = Some((child_n,
                                        child_hp,
                                        child_orgs,
                                        child_used));
                    */
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
    let hp = if cog_level >= 12 {
        200
    } else {
        (cog_level + 1) * (cog_level + 2)
    } as i16;
    let mut res = Vec::with_capacity(toon_count as usize);
    if let Some((_, first_gag)) = opt(&mut cache,
                                      gags,
                                      is_lured,
                                      toon_count,
                                      hp,
                                      org_count,
                                      GagUsages::new())
    {
        let mut next_gag = Some(first_gag);
        let mut args = (toon_count, hp, org_count, GagUsages::new());
        while let Some(gag) = next_gag {
            //println!("cache[{:?}]: {:?}", args, cache[&args]);

            res.place_back() <- gag.clone();

            args.0 -= 1;
            args.1 -= dmg(gag.clone(), is_lured, &args.3);
            args.2 -= if gag.is_org { 1 } else { 0 };
            update_usages(&mut args.3, &gag);

            if args.1 <= 0 {
                break;
            }

            next_gag = cache[&args].clone().map(|(_, g)| g);
        }

        while res.len() < toon_count as usize {
            res.place_back() <- PASS;
        }

        //println!();

        Some(res)
    } else {
        None
    }
}
