//! Royal Writ of Taxation
use super::prelude::*;

pub const ID: u32 = 86742443;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_STANDBY_PHASE, None);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// s.condition
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_STANDBY) and Duel.GetTurnPlayer()~=tp
    duel::is_phase(d, PHASE_STANDBY) && duel::get_turn_player(d) != a.tp
}
// s.target
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(1-tp,LOCATION_HAND,0)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, 1 - a.tp, LOCATION_HAND, 0) > 0;
    }
    true
}
// s.operation
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(1-tp,LOCATION_HAND,0)==0 then return end
    if duel::get_field_group_count(d, 1 - a.tp, LOCATION_HAND, 0) == 0 {
        return;
    }
    // local g=Duel.GetFieldGroup(1-tp,LOCATION_HAND,0):RandomSelect(tp,1)
    let hand = duel::get_field_group(d, 1 - a.tp, LOCATION_HAND, 0);
    let g = group::random_select(d, &hand, a.tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // Duel.ConfirmCards(tp,tc)
    duel::confirm_cards(d, a.tp, CardSet::from_card(tc));
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - a.tp);
    // local code=tc:GetCode()
    let code = card::get_code(d, tc)[0];
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(damcon)));
    effect::set_operation(d, e1, Some(operation(damop)));
    effect::set_label(d, e1, &[0]);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_operation(d, e2, Some(operation(damop2)));
    effect::set_label_object(d, e2, Value::Effect(e1));
    effect::set_label(d, e2, &[code as i64]);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 2);
    duel::register_effect(d, e2, a.tp);
    // local e3=Effect.CreateEffect(e:GetHandler())
    let e3 = effect::create_effect(d, h);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    effect::set_operation(d, e3, Some(operation(damop3)));
    effect::set_label_object(d, e3, Value::Effect(e1));
    effect::set_label(d, e3, &[code as i64]);
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 2);
    duel::register_effect(d, e3, a.tp);
}
// s.damcon
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and e:GetLabel()==0
    duel::is_turn_player(d, a.tp) && effect::get_label(d, a.e) == 0
}
// s.damop
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(1-tp,1000,REASON_EFFECT)
    duel::damage(d, 1 - a.tp, 1000, REASON_EFFECT, false, None);
}
// s.damop2
fn damop2(d: &mut Duel, a: &EffArgs) {
    // if eg:GetFirst():IsCode(e:GetLabel()) then
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    if card::is_code(d, first, &[effect::get_label(d, a.e) as u32]) {
        // Duel.Damage(tp,1000,REASON_EFFECT)
        duel::damage(d, a.tp, 1000, REASON_EFFECT, false, None);
        // e:GetLabelObject():SetLabel(1)
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        effect::set_label(d, lo, &[1]);
    }
}
// s.damop3
fn damop3(d: &mut Duel, a: &EffArgs) {
    // if re:GetHandler():IsCode(e:GetLabel()) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_code(d, rh, &[effect::get_label(d, a.e) as u32]) {
        // Duel.Damage(tp,1000,REASON_EFFECT)
        duel::damage(d, a.tp, 1000, REASON_EFFECT, false, None);
        // e:GetLabelObject():SetLabel(1)
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        effect::set_label(d, lo, &[1]);
    }
}
