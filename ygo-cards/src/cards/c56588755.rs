//! Dogmatika Genesis
use super::prelude::*;

pub const ID: u32 = 56588755;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return 1 banished Fusion, Synchro, Xyz, or Link monster to the Extra Deck, then negate the effects of 1 opponent's monster of the same type as the returned monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMINGS_CHECK_MONSTER_E) as u32));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.retfilter(c,tp)`: check for a Fusion, Synchro, Xyz, or Link monster
fn retfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local typ=c:GetType()&(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK)
    let typ = card::get_type(d, c) & (TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK);
    // return c:IsFaceup() and typ~=0 and c:IsAbleToExtra()
    //   and Duel.IsExistingTarget(s.disfilter,tp,0,LOCATION_MZONE,1,nil,typ)
    card::is_faceup(d, c)
        && typ != 0
        && card::is_able_to_extra(d, c)
        && duel::is_existing_target(d, Some(&|d: &mut Duel, c: CardId| disfilter(d, c, typ)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.disfilter(c,ex)`: check for an effect monster with same card type from filter1
fn disfilter(d: &mut Duel, c: CardId, ex: u32) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and c:IsType(ex) and not c:IsDisabled()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && card::is_type(d, c, ex) && !card::is_disabled(d, c)
}
/// `s.target(e,tp,...,chk,chkc)`: activation legality
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.retfilter,tp,LOCATION_REMOVED,LOCATION_REMOVED,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d: &mut Duel, c: CardId| retfilter(d, c, a.tp)), a.tp, LOCATION_REMOVED, LOCATION_REMOVED, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g1=Duel.SelectTarget(tp,s.retfilter,tp,LOCATION_REMOVED,LOCATION_REMOVED,1,1,nil,tp)
    let g1 = duel::select_target(d, a.tp, Some(&|d: &mut Duel, c: CardId| retfilter(d, c, a.tp)), a.tp, LOCATION_REMOVED, LOCATION_REMOVED, 1, 1, false, NO_EX).nil();
    // local typ=g1:GetFirst():GetType()&(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK)
    let first = group::get_first(d, &g1).nil();
    let typ = card::get_type(d, first) & (TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g2=Duel.SelectTarget(tp,s.disfilter,tp,0,LOCATION_MZONE,1,1,nil,typ)
    let g2 = duel::select_target(d, a.tp, Some(&|d: &mut Duel, c: CardId| disfilter(d, c, typ)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g1,1,0,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g1), 1, 0, LOCATION_REMOVED as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g2,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g2, 1, 0, 0);
    true
}
/// `s.activate(e,tp,...)`: return 1 banished monster to extra deck, and if you do, negate targeted monster's effects
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ex,g1=Duel.GetOperationInfo(0,CATEGORY_TODECK)
    let g1 = duel::get_operation_info(d, 0, CATEGORY_TODECK).and_then(|o| o.op_cards);
    // local ex,g2=Duel.GetOperationInfo(0,CATEGORY_DISABLE)
    let g2 = duel::get_operation_info(d, 0, CATEGORY_DISABLE).and_then(|o| o.op_cards);
    // if g1:GetFirst():IsRelateToEffect(e) then
    let g1 = g1.nil();
    let first = group::get_first(d, &g1).nil();
    if card::is_relate_to_effect(d, first, a.e) {
        // Duel.SendtoDeck(g1,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g1.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // if og:GetFirst():IsLocation(LOCATION_EXTRA) then
        let og_first = group::get_first(d, &og).nil();
        if card::is_location(d, og_first, LOCATION_EXTRA) {
            // local tc=g2:GetFirst()
            let tc = group::get_first(d, &g2.clone().nil());
            // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
            if let Some(tc) = tc {
                if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
                    // tc:NegateEffects(c)
                    card::negate_effects(d, tc, c, None, false, None);
                }
            }
        }
    }
}
