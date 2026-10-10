//! Mushroom Man #2
use super::prelude::*;

pub const ID: u32 = 93900406;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(damcon)));
    effect::set_target(d, e1, Some(cost_target(damtg)));
    effect::set_operation(d, e1, Some(operation(damop)));
    card::register_effect(d, c, e1, false);
    // control
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_category(d, e2, CATEGORY_CONTROL);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(ctlcon)));
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    effect::set_target(d, e2, Some(cost_target(ctltg)));
    effect::set_operation(d, e2, Some(operation(ctlop)));
    card::register_effect(d, c, e2, false);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,0,0,tp,300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 300);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(tp,300,REASON_EFFECT)
    duel::damage(d, a.tp, 300, REASON_EFFECT, false, None);
}
/// `s.ctlcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctlcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.ctltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ctltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsControlerCanBeChanged() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_controler_can_be_changed(d, h, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.ctlop(e,tp,eg,ep,ev,re,r,rp)`
fn ctlop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Duel.GetControl(c,1-tp)
        duel::get_control(d, CardSet::from_card(c), 1 - a.tp, 0, 0, None, None);
    }
}
