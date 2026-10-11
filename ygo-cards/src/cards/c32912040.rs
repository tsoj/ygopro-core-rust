//! Triamid Master
use super::prelude::*;

pub const ID: u32 = 32912040;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRIAMID as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 Set card on the field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(descost)));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
    // Activate 1 "Triamid" Field Spell from the Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(turn_cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_TRIAMID) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_TRIAMID as u16]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsFacedown() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_facedown(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFacedown,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_facedown(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFacedown,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_facedown(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
fn turn_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // function(_,tp) return Duel.IsTurnPlayer(1-tp) end
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.filter(c,tp,code)`
fn filter(d: &mut Duel, c: CardId, tp: u8, code: u32) -> bool {
    // return c:IsFieldSpell() and c:IsSetCard(SET_TRIAMID) and c:GetActivateEffect():IsActivatable(tp,true,true) and not c:IsCode(code)
    if !(card::is_field_spell(d, c) && card::is_set_card(d, c, &[SET_TRIAMID as u16])) {
        return false;
    }
    let e = card::get_activate_effect(d, c).first().copied().nil();
    effect::is_activatable(d, e, tp, true, true) && !card::is_code(d, c, &[code])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let tc = duel::get_field_card(d, a.tp, LOCATION_FZONE, 0);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return tc and tc:IsFaceup() and ... end
    if a.chk == Some(0) {
        let Some(tc) = tc else {
            return false;
        };
        if !(card::is_faceup(d, tc)
            && card::is_set_card(d, tc, &[SET_TRIAMID as u16])
            && card::is_able_to_grave(d, tc, None)
            && card::is_can_be_effect_target(d, tc, Some(a.e)))
        {
            return false;
        }
        let code = card::get_code(d, tc)[0];
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.tp, code)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetTargetCard(tc)
    let tc = tc.nil();
    duel::set_target_card(d, tc);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(tc)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoGrave(tc,REASON_EFFECT)~=0 and tc:IsLocation(LOCATION_GRAVE) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_grave(d, tc, REASON_EFFECT, None, None) != 0
        && card::is_location(d, tc, LOCATION_GRAVE)
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
        // local fc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,tp,tc:GetCode()):GetFirst()
        let code = card::get_code(d, tc)[0];
        let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.tp, code)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let fc = group::get_first(d, &sel);
        // Duel.ActivateFieldSpell(fc,e,tp,eg,ep,ev,re,r,rp)
        duel::activate_field_spell(d, fc, a, None);
    }
}
