//! Nine Pillars of Yang Zing
use super::prelude::*;

pub const ID: u32 = 57831349;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YANG_ZING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_TODECK+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE | CATEGORY_TODECK | CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`: `return c:IsFaceup() and c:IsSetCard(SET_YANG_ZING)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_YANG_ZING as u16])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil)
    //   and (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE)) and Duel.IsChainNegatable(ev)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && (effect::is_monster_effect(d, a.re.nil()) || effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE))
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsRelateToEffect(re) then
    let rc = effect::get_handler(d, a.re.nil()).nil();
    if card::is_relate_to_effect(d, rc, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.desfilter(c)`: `return c:IsFaceup() and c:IsSetCard(SET_YANG_ZING)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_YANG_ZING as u16])
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ec=re:GetHandler()
    let ec = effect::get_handler(d, a.re.nil()).nil();
    // if Duel.NegateActivation(ev) and ec:IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, ec, a.re.nil()) {
        // ec:CancelToGrave()
        card::cancel_to_grave(d, ec, true);
        // if Duel.SendtoDeck(ec,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)~=0 and ec:IsLocation(LOCATION_DECK|LOCATION_EXTRA) then
        if duel::send_to_deck(d, ec, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 0
            && card::is_location(d, ec, LOCATION_DECK | LOCATION_EXTRA)
        {
            // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_ONFIELD,0,e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let g = duel::get_matching_group(d, Some(&desfilter), a.tp, LOCATION_ONFIELD, 0, Exclude::Card(h));
            // if #g>0 then
            if !g.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
                // local sg=g:Select(tp,1,1,nil)
                let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
                // Duel.Destroy(sg,REASON_EFFECT)
                duel::destroy(d, sg, REASON_EFFECT, None, None);
            }
        }
    }
}
