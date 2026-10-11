//! Kozmojo
use super::prelude::*;

pub const ID: u32 = 90452877;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_KOZMO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_KOZMO)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_KOZMO as u16])
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and (c:IsLocation(LOCATION_SZONE) or aux.SpElimFilter(c,true,true))
    card::is_able_to_remove(d, c, None, None, None) && (card::is_location(d, c, LOCATION_SZONE) || aux::sp_elim_filter(d, c, true, true))
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.desfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && desfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.rmfilter,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&desfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&rmfilter), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&desfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)~=0 then
    let Some(tc) = tc else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, a.e) || duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local b1=Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    // local b2=not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,1,nil)
    let b2 = duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none()
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    // local op=0
    // if b1 and b2 then op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
    // else op=2 end
    let op = if b1 && b2 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)])
    } else {
        2
    };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=nil
    // if op==0 then
    let g = if op == 0 {
        // g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,1,nil)
        duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX)
    } else if op == 1 {
        // g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,1,1,nil)
        duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX)
    } else {
        // g=Duel.SelectMatchingCard(tp,s.rmfilter,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,1,nil)
        duel::select_matching_card(d, tp, Some(&rmfilter), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, 1, false, NO_EX)
    }
    .nil();
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
