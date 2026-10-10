//! The Shallow Grave
use super::prelude::*;

pub const ID: u32 = 43434803;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
        //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingTarget(s.filter,1-tp,LOCATION_GRAVE,0,1,nil,e,1-tp)
        //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE,1-tp)>0
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, 1 - tp)), 1 - tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(1 - tp), None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let sg = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
    // local og=Duel.SelectTarget(1-tp,s.filter,1-tp,LOCATION_GRAVE,0,1,1,nil,e,1-tp)
    let og = duel::select_target(d, 1 - tp, Some(&|d, c| filter(d, c, e, 1 - tp)), 1 - tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local sc=sg:GetFirst()
    let sc = group::get_first(d, &sg);
    // local oc=og:GetFirst()
    let oc = group::get_first(d, &og);
    // local g=Group.FromCards(sc,oc)
    let cards: Vec<CardId> = [sc, oc].into_iter().flatten().collect();
    let g = group::create_group(&cards);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,2,PLAYER_ALL,g:GetFirst():GetOwner())
    let owner = card::get_owner(d, group::get_first(d, &g).nil());
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 2, PLAYER_ALL as u8, owner as i32);
    // e:SetLabelObject(sc)
    effect::set_label_object(d, e, Value::card(sc));
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local sc=e:GetLabelObject()
    let sc = effect::get_label_object(d, a.e).as_card().nil();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local oc=g:GetFirst()
    let mut oc = group::get_first(d, &g);
    // if oc==sc then oc=g:GetNext() end
    if oc == Some(sc) {
        oc = group::take_at_pos(d, &g, 1);
    }
    // if sc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, sc, a.e) {
        // Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)
        duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
    }
    // if oc:IsRelateToEffect(e) then
    let oc = oc.nil();
    if card::is_relate_to_effect(d, oc, a.e) {
        // Duel.SpecialSummonStep(oc,0,1-tp,1-tp,false,false,POS_FACEDOWN_DEFENSE)
        duel::special_summon_step(d, oc, 0, 1 - a.tp, 1 - a.tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
