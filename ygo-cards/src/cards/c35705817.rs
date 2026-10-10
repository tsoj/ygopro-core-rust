//! Ghost Fusion
use super::prelude::*;

pub const ID: u32 = 35705817;

const LOCATION_HAND_DECK_GRAVE: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Fusion.CreateSummonEff(c,nil,Fusion.OnFieldMat(aux.FilterBoolFunction(Card.IsRace,RACE_ZOMBIE)),
    //     s.fextra,s.extraop,nil,nil,nil,nil,nil,nil,nil,nil,nil,s.extratg)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            matfilter: Some(fusion::on_field_mat_with(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| card::is_race(d, c, RACE_ZOMBIE)))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::extra_op(extraop)),
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
fn fcheck_fn() -> LuaFn {
    lua_fn(|d: &mut Duel, a: Args| {
        // return sg:FilterCount(Card.IsLocation,nil,LOCATION_HAND_DECK_GRAVE)<=1
        let sg = a.g(1);
        group::filter_count(d, &sg, &|d, c| card::is_location(d, c, LOCATION_HAND_DECK_GRAVE), NO_EX) <= 1
    })
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if Duel.GetLP(tp)<Duel.GetLP(1-tp) and not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::get_lp(d, tp) < duel::get_lp(d, 1 - tp) && duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // local eg=Duel.GetMatchingGroup(s.exfilter0,tp,LOCATION_MZONE|LOCATION_HAND_DECK_GRAVE,0,nil)
        let eg = duel::get_matching_group(d, Some(&exfilter0), tp, LOCATION_MZONE | LOCATION_HAND_DECK_GRAVE, 0, NO_EX);
        // if #eg>0 then return eg,s.fcheck end
        if !eg.is_empty() {
            return (Some(eg), Some(fcheck_fn()));
        }
    }
    // return nil
    (None, None)
}
/// `s.exfilter0(c)`
fn exfilter0(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsAbleToRemove()
    card::is_race(d, c, RACE_ZOMBIE) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.extraop(e,tc,tp,sg)`
fn extraop(d: &mut Duel, _e: EffectId, _tc: Option<CardId>, _tp: u8, sg: &mut CardSet) -> bool {
    // local rg=sg:Filter(Card.IsLocation,nil,LOCATION_HAND_DECK_GRAVE)
    let rg = group::filter(d, &*sg, &|d, c| card::is_location(d, c, LOCATION_HAND_DECK_GRAVE), NO_EX);
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT+REASON_MATERIAL+REASON_FUSION)
        duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT + REASON_MATERIAL + REASON_FUSION, None, None);
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
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,0,tp,LOCATION_HAND_DECK_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 0, a.tp, LOCATION_HAND_DECK_GRAVE as i32);
    true
}
