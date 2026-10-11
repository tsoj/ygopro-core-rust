//! Ancient Gear Fusion
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 64061284;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANCIENT_GEAR], listed_names: &[CARD_ANCIENT_GEAR_GOLEM, 95735217], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 "Ancient Gear" Fusion Monster from your Extra Deck, using monsters from your hand or field as Fusion Material
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=function(c) return c:IsSetCard(SET_ANCIENT_GEAR) end,extrafil=s.fextra,extratg=s.extratg})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_ANCIENT_GEAR as u16]))),
            extrafil: Some(Arc::new(fextra)),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filterchk(c)`
fn filterchk(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_ANCIENT_GEAR_GOLEM,95735217) and c:IsOnField()
    card::is_code(d, c, &[CARD_ANCIENT_GEAR_GOLEM, 95735217]) && card::is_on_field(d, c)
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    // if sg:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then
    let sg = a.g(1);
    if group::is_exists(d, &sg, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        // return sg:IsExists(s.filterchk,1,nil) end
        return group::is_exists(d, &sg, &filterchk, 1, NO_EX);
    }
    // return true
    true
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if mg:IsExists(s.filterchk,1,nil) then
    if group::is_exists(d, mg, &filterchk, 1, NO_EX) {
        // local eg=Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToGrave),tp,LOCATION_DECK,0,nil)
        let eg = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None)), tp, LOCATION_DECK, 0, NO_EX);
        // if eg and #eg>0 then
        if !eg.is_empty() {
            // return eg,s.fcheck
            return (Some(eg), Some(lua_fn(fcheck)));
        }
    }
    // return nil
    (None, None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
