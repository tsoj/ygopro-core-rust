//! Contact with Gusto
//! ガスタの交信
use super::prelude::*;

pub const ID: u32 = 83544697;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GUSTO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GUSTO) and c:IsMonster() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_GUSTO as u16]) && card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local h=e:GetHandler()  (the Lua calls e:GetHandler() inside the target calls)
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_GRAVE,0,2,nil)
    //   and Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter1), tp, LOCATION_GRAVE, 0, 2, NO_EX)
            && duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_GRAVE,0,2,2,nil)
    let g1 = duel::select_target(d, tp, Some(&filter1), tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g2=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,1,1,e:GetHandler())
    let g2 = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g1, 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g2,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g2, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local ex,g1=Duel.GetOperationInfo(0,CATEGORY_TODECK)
    let g1 = duel::get_operation_info(d, 0, CATEGORY_TODECK).and_then(|o| o.op_cards).nil();
    // local ex,g2=Duel.GetOperationInfo(0,CATEGORY_DESTROY)
    let g2 = duel::get_operation_info(d, 0, CATEGORY_DESTROY).and_then(|o| o.op_cards).nil();
    // if g1:GetFirst():IsRelateToEffect(e) and g1:GetNext():IsRelateToEffect(e) then
    let first = group::get_first(d, &g1).nil();
    if card::is_relate_to_effect(d, first, e) {
        let second = group::iter(d, &g1).get(1).copied().nil();
        if card::is_relate_to_effect(d, second, e) {
            // Duel.SendtoDeck(g1,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, g1, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
            // if g2:GetFirst():IsRelateToEffect(e) then
            let f2 = group::get_first(d, &g2).nil();
            if card::is_relate_to_effect(d, f2, e) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Destroy(g2,REASON_EFFECT)
                duel::destroy(d, g2, REASON_EFFECT, None, None);
            }
        }
    }
}
