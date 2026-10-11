//! Fullmetalfoes Fusion
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 39564736;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_METALFOES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_METALFOES))
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(Arc::new(|d: &mut Duel, c: CardId, _tp: u8| card::is_set_card(d, c, &[SET_METALFOES as u16]))),
            ..Default::default()
        },
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
