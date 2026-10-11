//! Pyroxene Fusion
use super::prelude::*;

pub const ID: u32 = 55824220;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GEM_KNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // c:RegisterEffect(Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_GEM_KNIGHT)))
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff { fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_GEM_KNIGHT as u16]))), ..Default::default() },
    );
    card::register_effect(d, c, e1, false);
}
