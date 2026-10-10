//! Odd-Eyes Fusion
use super::prelude::*;

pub const ID: u32 = 48144509;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ODD_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsRace,RACE_DRAGON),nil,s.fextra)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_DRAGON))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.fcheck(tp,sg,fc)
//     return sg:FilterCount(Card.IsLocation,nil,LOCATION_EXTRA)<=2
// end
fn fcheck(d: &mut Duel, a: Args) -> bool {
    let sg = a.g(1);
    group::filter_count(d, &sg, &|d, c| card::is_location(d, c, LOCATION_EXTRA), NO_EX) <= 2
}
// function s.fextra(e,tp,mg)
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>1 then
    if duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0 && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 1 {
        // local sg=Duel.GetMatchingGroup(s.exfilter,tp,LOCATION_EXTRA,0,nil)
        let sg = duel::get_matching_group(d, Some(&exfilter), tp, LOCATION_EXTRA, 0, NO_EX);
        // if #sg>0 then
        if !sg.is_empty() {
            // return sg,s.fcheck
            return (Some(sg), Some(lua_fn(fcheck)));
        }
    }
    // return nil
    (None, None)
}
// function s.exfilter(c)
fn exfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ODD_EYES) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_ODD_EYES as u16]) && card::is_able_to_grave(d, c, None)
}
