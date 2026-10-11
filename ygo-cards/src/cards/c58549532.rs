//! Parametalfoes Fusion
//! Scripted by edo9300
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 58549532;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_METALFOES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_METALFOES),extrafil=s.extrafil})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(Arc::new(|d: &mut Duel, c: CardId, _tp: u8| card::is_set_card(d, c, &[SET_METALFOES as u16]))),
            extrafil: Some(Arc::new(|d: &mut Duel, e: EffectId, tp: u8, mg: &CardSet| extrafil(d, e, tp, mg))),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.check(tp,sg,fc)`
fn check(d: &mut Duel, a: Args) -> bool {
    // return sg:GetClassCount(function(c) return c:GetLocation()&~(LOCATION_ONFIELD) end)==#sg
    let sg = a.g(1);
    let n = group::get_class_count(d, &sg, &|d, c| (card::get_location(d, c) & !LOCATION_ONFIELD) as i64);
    n == sg.len()
}
/// `s.extrafil(e,tp,mg)`
fn extrafil(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_EXTRA,0,nil),s.check
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_EXTRA, 0, NO_EX);
    (Some(g), Some(lua_fn(check)))
}
