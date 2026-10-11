//! Lunalight Fusion
//! 月光融合
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 87931906;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LUNALIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Fusion.CreateSummonEff(c,aux.FilterBoolFunction(Card.IsSetCard,SET_LUNALIGHT),nil,s.fextra,nil,nil,nil,nil,nil,nil,nil,nil,nil,nil,s.extratg)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, x, _tp| card::is_set_card(d, x, &[SET_LUNALIGHT as u16]))),
            extrafil: Some(Arc::new(fextra)),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_EXTRA|LOCATION_DECK)<=1
    let sg = a.g(1);
    group::filter_count(d, &sg, &|d, x| card::is_location(d, x, LOCATION_EXTRA | LOCATION_DECK), NO_EX) <= 1
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if Duel.IsExistingMatchingCard(Card.IsSummonLocation,tp,0,LOCATION_MZONE,1,nil,LOCATION_EXTRA) then
    if duel::is_existing_matching_card(d, Some(&|d, x| card::is_summon_location(d, x, LOCATION_EXTRA)), tp, 0, LOCATION_MZONE, 1, NO_EX) {
        // local eg=Duel.GetMatchingGroup(s.exfilter,tp,LOCATION_EXTRA|LOCATION_DECK,0,nil)
        let eg = duel::get_matching_group(d, Some(&exfilter), tp, LOCATION_EXTRA | LOCATION_DECK, 0, NO_EX);
        // if eg and #eg>0 then
        if !eg.is_empty() {
            // return eg,s.fcheck
            return (Some(eg), Some(lua_fn(fcheck)));
        }
    }
    // return nil
    (None, None)
}
/// `s.exfilter(c)`
fn exfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_LUNALIGHT) and c:IsAbleToGrave()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_LUNALIGHT as u16]) && card::is_able_to_grave(d, c, None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
