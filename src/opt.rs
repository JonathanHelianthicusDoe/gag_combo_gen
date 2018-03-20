use fnv::FnvHashMap as Map;
use gag_types::{Gag, GagHistory};
use gags::PASS;
use hp::Hp;
use std;


fn use_gag(hp: &mut Hp, is_lured: bool, used: &mut GagHistory, gag: &Gag) {
    used.add_gag(gag);
    hp.apply_all_gags(is_lured, used);
}

fn opt(// Constant
       cache:     &mut Map<(u8, Hp, u8, GagHistory), Option<(u8, Gag)>>,
       gags:      &Vec<Gag>,
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
    for (gag_rank, gag) in gags.iter().enumerate() {
        let gag_rank = gag_rank as u8;
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
            let new_min_max_rank = child_rank.max(gag_rank);
            if new_min_max_rank < min_max_rank {
                min_max_rank = new_min_max_rank;
                min_max_gag = Some(gag);
                min_max_gag_rank = gag_rank;
            } else if new_min_max_rank == min_max_rank {
                if gag_rank < min_max_gag_rank {
                    min_max_gag = Some(gag);
                    min_max_gag_rank = gag_rank;
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

pub fn opt_combo(gags:       &Vec<Gag>,
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
