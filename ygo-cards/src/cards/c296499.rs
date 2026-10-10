//! The Regulation of Tribe
use super::prelude::*;

pub const ID: u32 = 296499;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    card::register_effect(d, c, e1, false);
    // cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(atktarget)));
    card::register_effect(d, c, e2, false);
    effect::set_label_object(d, e2, Value::Effect(e1));
    // maintain
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(mtcon)));
    effect::set_operation(d, e3, Some(operation(mtop)));
    card::register_effect(d, c, e3, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RACE as u64);
    let rc = duel::announce_race(d, a.tp, 1, RACE_ALL);
    effect::set_label(d, a.e, &[rc as i64]);
    let h = effect::get_handler(d, a.e).nil();
    card::set_hint(d, h, CHINT_RACE as u8, rc);
    true
}
fn atktarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetRace()==e:GetLabelObject():GetLabel()
    let le = effect::get_label_object(d, e).as_effect().nil();
    card::get_race(d, c) as i64 == effect::get_label(d, le)
}
fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
fn releasable(d: &mut Duel, c: CardId) -> bool {
    card::is_releasable(d, c, None, None)
}
fn mtop(d: &mut Duel, a: &EffArgs) {
    // if Duel.CheckReleaseGroupCost(tp,Card.IsReleasable,1,false,nil,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::check_release_group_cost(d, a.tp, Some(&releasable), 1, 1, false, None, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // local g=Duel.SelectReleaseGroupCost(tp,Card.IsReleasable,1,1,false,nil,nil)
        let g = duel::select_release_group_cost(d, a.tp, Some(&releasable), 1, 1, false, None, NO_EX);
        // Duel.Release(g,REASON_COST)
        duel::release(d, g, REASON_COST, None);
    } else {
        // Duel.Destroy(e:GetHandler(),REASON_COST)
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_COST, None, None);
    }
}
