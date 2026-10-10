//! Apprentice Illusion Magician
use super::prelude::*;

pub const ID: u32 = 30603688;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    card::register_effect(d, c, e1, false);
    // search
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
    // atkup
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e4, EVENT_PRE_DAMAGE_CALCULATE);
    effect::set_range(d, e4, LOCATION_HAND | LOCATION_MZONE);
    effect::set_condition(d, e4, Some(condition(atkcon)));
    effect::set_cost(d, e4, Some(cost::self_to_grave()));
    effect::set_operation(d, e4, Some(operation(atkop)));
    card::register_effect(d, c, e4, false);
}
/// `s.discardable` filter: `Card.IsDiscardable`
fn discardable(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let Some(c) = a.card(1) else { return true };
    let tp = card::get_controler(d, c);
    let h = effect::get_handler(d, e).nil();
    let rg = duel::get_matching_group(d, Some(&discardable), tp, LOCATION_HAND, 0, h.into());
    let chkf = aux::chkf_mmz(1);
    matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&chkf), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let h = effect::get_handler(d, e).nil();
    let rg = duel::get_matching_group(d, Some(&discardable), tp, LOCATION_HAND, 0, h.into());
    let chkf = aux::chkf_mmz(1);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&chkf), 1, tp, Some(HINTMSG_DISCARD as u64), None, None, true).unwrap_or_default();
    if !g.is_empty() {
        // g:KeepAlive() (groups are values in Rust)
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    duel::send_to_grave(d, g, REASON_DISCARD | REASON_COST, None, None);
    // g:DeleteGroup() (groups are values in Rust)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_DARK_MAGICIAN]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=Duel.GetAttackTarget()
    let Some(at) = duel::get_attack_target(d) else { return false };
    let mut c = Some(at);
    // if c:IsControler(1-tp) then c=Duel.GetAttacker() end (nil when there is no attacker)
    if card::is_controler(d, at, 1 - tp) {
        c = duel::get_attacker(d);
    }
    // e:SetLabelObject(c)
    effect::set_label_object(d, e, c.map_or(Value::Nil, Value::Card));
    let Some(c) = c else { return false };
    let h = effect::get_handler(d, e).nil();
    c != h && card::is_race(d, c, RACE_SPELLCASTER) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_relate_to_battle(d, c)
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let c = effect::get_label_object(d, e).as_card().nil();
    if card::is_faceup(d, c) && card::is_relate_to_battle(d, c) {
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        effect::set_value(d, e1, 2000);
        card::register_effect(d, c, e1, false);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        card::register_effect(d, c, e2, false);
    }
}
