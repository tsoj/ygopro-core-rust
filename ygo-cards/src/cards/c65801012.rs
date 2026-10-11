//! Cynet Fusion
use super::prelude::*;

pub const ID: u32 = 65801012;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=aux.FilterBoolFunction(Card.IsRace,RACE_CYBERSE),
    //     extrafil=s.fextra,extraop=s.extraop,extratg=s.extratg})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_CYBERSE))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::extra_op(extraop)),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.fcheck(tp,sg,fc)`
fn fcheck(d: &mut Duel, a: Args) -> bool {
    // return sg:FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)<=1
    let sg = a.g(1);
    group::filter_count(d, &sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX) <= 1
}
/// `s.exfilter0(c)`
fn exfilter0(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLinkMonster() and c:IsRace(RACE_CYBERSE) and c:IsAbleToRemove()
    card::is_link_monster(d, c) && card::is_race(d, c, RACE_CYBERSE) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_EMZONE,0)==0 and not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::get_field_group_count(d, tp, LOCATION_EMZONE, 0) == 0 && duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // local eg=Duel.GetMatchingGroup(s.exfilter0,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
        let eg = duel::get_matching_group(d, Some(&exfilter0), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
        // if #eg>0 then return eg,s.fcheck end
        if !eg.is_empty() {
            return (Some(eg), Some(lua_fn(fcheck)));
        }
    }
    // return nil
    (None, None)
}
/// `s.extraop(e,tc,tp,sg)`
fn extraop(d: &mut Duel, _e: EffectId, _tc: Option<CardId>, _tp: u8, sg: &mut CardSet) -> bool {
    // local rg=sg:Filter(Card.IsLocation,nil,LOCATION_GRAVE)
    let rg = group::filter(d, sg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT|REASON_MATERIAL|REASON_FUSION)
        duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT | REASON_MATERIAL | REASON_FUSION, None, None);
        // sg:Sub(rg)
        group::remove_card(sg, &rg);
    }
    true
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, a.tp, LOCATION_GRAVE as i32);
    true
}
