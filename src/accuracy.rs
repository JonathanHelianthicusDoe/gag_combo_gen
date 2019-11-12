use crate::{
    gag_types::{Gag, GagType::*},
    opt::Luring,
};

/// We assume that everyone has maxed their gag tracks.
const TRACK_EXP: u8 = (7 - 1) * 10;

fn get_tgt_def(cog_level: u8, is_tier_1: bool) -> i8 {
    if is_tier_1 {
        match cog_level {
            1 => -2,
            4 => -12,
            5 => -15,
            _ => (cog_level as i8 - 1) * -5,
        }
    } else {
        if cog_level == 1 {
            -2
        } else {
            (cog_level as i8 - 1) * -5
        }
    }
}

/// Returns the probability that **all** attacks hit, assuming that the cog has
/// infinite HP.
pub fn combo_accuracy(
    cog_level: u8,
    is_tier_1: bool,
    luring: Luring,
    combo: &[Gag],
) -> f32 {
    let mut acc = 1.0;

    let (trapped, sounds, throws, squirts, drops) = {
        let mut trapped = false;
        let mut sounds = 0;
        let mut throws = 0;
        let mut squirts = 0;
        let mut drops = 0;

        for gag in combo {
            match gag.gag_type {
                Trap => {
                    if trapped || luring == Luring::NoLure {
                        return 0.0;
                    }
                    trapped = true;
                },
                Sound => sounds += 1,
                Throw => throws += 1,
                Squirt => squirts += 1,
                Drop => drops += 1,
                Pass => (),
            }
        }

        (trapped, sounds, throws, squirts, drops)
    };

    let tgt_def = i16::from(get_tgt_def(cog_level, is_tier_1));
    // We assume that no other pre-trap sources of stun like e.g. toonup,
    // fires, &c. affect this cog. We also assume `lured_ratio = 0`.
    let mut bonus = if trapped { 20 } else { 0 };

    if let Luring::Luring(lure_gag) = luring {
        let lure_acc = (i16::from(lure_gag.prop_acc())
            + i16::from(TRACK_EXP)
            + tgt_def
            + bonus)
            .min(95);

        acc *= f32::from(lure_acc) / 100.0;

        bonus += 20;
    }
    let mut lured = luring != Luring::NoLure && !trapped;

    if sounds > 0 {
        // Since we assume that the highest cog level is 12 and that all toons
        // have their gag tracks maxed, `atk_acc` is guaranteed to be `95` for
        // sound. See the worst case (level 12 cog, so `tgt_def = -55`):
        //
        // ```
        // atk_acc = min(95 + 60 + -55 + 0, 95)
        // atk_acc = min(100, 95)
        // atk_acc = 95
        // ```
        //
        // We also assume that [rules as written
        // here](https://github.com/QED1224/Toontown-Resources/tree/ce5b6ec60fd230d93773b29b345871a5462b7844)
        // are correct, which implies that sound can miss when all cogs in the
        // battle are lured.
        acc *= 0.95;

        lured = false;
    }
    bonus += 20 * sounds;

    if throws > 0 && !lured {
        let throw_acc = (75 + i16::from(TRACK_EXP) + tgt_def + bonus).min(95);

        acc *= f32::from(throw_acc) / 100.0;
    }
    bonus += 20 * throws;
    if throws > 0 {
        lured = false;
    }

    if squirts > 0 && !lured {
        // Assume `atk_acc = 95` here for the same reasons as sound explained
        // above.
        acc *= 0.95;
    }
    bonus += 20 * squirts;
    if squirts > 0 {
        lured = false;
    }

    if drops > 0 {
        if lured {
            return 0.0;
        }

        let drop_acc = (50 + i16::from(TRACK_EXP) + tgt_def + bonus).min(95);

        acc *= f32::from(drop_acc) / 100.0;
    }

    acc
}
