//! Musto, Oracle of Gusto
use super::prelude::*;

pub const ID: u32 = 9837195;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GUSTO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --disable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_TODECK);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GUSTO) and c:IsMonster() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_GUSTO as u16]) && card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_GRAVE,0,1,nil)
    //     and Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter1), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter2), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_GRAVE,0,1,1,nil)
    let g1 = duel::select_target(d, tp, Some(&filter1), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g2 = duel::select_target(d, tp, Some(&filter2), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g1,1,0,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g1, 1, 0, LOCATION_GRAVE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g2,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g2, 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local ex,g1=Duel.GetOperationInfo(0,CATEGORY_TODECK)
    let g1 = duel::get_operation_info(d, 0, CATEGORY_TODECK).and_then(|x| x.op_cards);
    // local ex,g2=Duel.GetOperationInfo(0,CATEGORY_DISABLE)
    let g2 = duel::get_operation_info(d, 0, CATEGORY_DISABLE).and_then(|x| x.op_cards);
    // if g1:GetFirst():IsRelateToEffect(e) then
    let g1_first = group::get_first(d, &g1.clone().nil()).nil();
    if card::is_relate_to_effect(d, g1_first, e) {
        // Duel.SendtoDeck(g1,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g1.clone().nil(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // if og:GetFirst():IsLocation(LOCATION_DECK) or og:GetFirst():IsLocation(LOCATION_EXTRA) then
        let og_first = group::get_first(d, &og).nil();
        if card::is_location(d, og_first, LOCATION_DECK) || card::is_location(d, og_first, LOCATION_EXTRA) {
            // local tc=g2:GetFirst()
            let tc = group::get_first(d, &g2.clone().nil()).nil();
            // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
            if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) {
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_DISABLE)
                effect::set_code(d, e1, EFFECT_DISABLE);
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
                // local e2=Effect.CreateEffect(e:GetHandler())
                let e2 = effect::create_effect(d, h);
                // e2:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                // e2:SetCode(EFFECT_DISABLE_EFFECT)
                effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                // e2:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
                // tc:RegisterEffect(e2)
                card::register_effect(d, tc, e2, false);
            }
        }
    }
}
