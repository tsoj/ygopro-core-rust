//! Miracle Contact
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 35255456;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_NEOS], listed_series: &[SET_ELEMENTAL_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // c:RegisterEffect(Fusion.CreateSummonEff(c,s.spfilter,Card.IsAbleToDeck,s.fextra,Fusion.ShuffleMaterial,nil,nil,nil,0,nil,FUSPROC_NOTFUSION|FUSPROC_LISTEDMATS))
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(Arc::new(|d: &mut Duel, c: CardId, _tp: u8| spfilter(d, c))),
            matfilter: Some(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_able_to_deck(d, c))),
            extrafil: Some(Arc::new(|d: &mut Duel, e: EffectId, tp: u8, mg: &CardSet| fextra(d, e, tp, mg))),
            extraop: Some(fusion::shuffle_material()),
            value: Some(0),
            chkf: Some((FUSPROC_NOTFUSION | FUSPROC_LISTEDMATS) as u64),
            ..Default::default()
        },
    );
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]) && card::lists_code_as_material(d, c, &[CARD_NEOS])
}
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    let g = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, c| card::is_monster(d, c) && card::is_able_to_deck(d, c))),
        tp,
        LOCATION_GRAVE,
        0,
        NO_EX,
    );
    (Some(g), None)
}
