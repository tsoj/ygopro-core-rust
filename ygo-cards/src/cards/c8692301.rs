//! Gem-Knight Zirconia
use super::prelude::*;

pub const ID: u32 = 8692301;
pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_GEM as u32, SET_GEM_KNIGHT as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GEM_KNIGHT),aux.FilterBoolFunctionEx(Card.IsRace,RACE_ROCK))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_GEM_KNIGHT as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_race_ctx(d, x, RACE_ROCK, Some(fc), st, Some(tp)))),
        ],
    );
}
