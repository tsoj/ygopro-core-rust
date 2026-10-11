//! SPYRAL GEAR - Last Resort
use super::prelude::*;

pub const ID: u32 = 37433748;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPYRAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip this card to 1 "SPYRAL" monster you control
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SPYRAL as u16])
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && eqfilter(d, chkc) && chkc != h;
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&eqfilter), tp, LOCATION_MZONE, 0, 1, h.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    duel::select_target(d, tp, Some(&eqfilter), tp, LOCATION_MZONE, 0, 1, 1, false, h.into());
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    if card::is_location(d, c, LOCATION_MZONE) && card::is_facedown(d, c) {
        return;
    }
    let tc = duel::first_target(d);
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0
        || card::is_controler(d, tc.nil(), 1 - tp)
        || card::is_facedown(d, tc.nil())
        || !card::is_relate_to_effect(d, tc.nil(), e)
    {
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        return;
    }
    let tc = tc.nil();
    duel::equip(d, tp, c, tc, true, false);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
    effect::set_label_object(d, e1, Value::Card(tc));
    card::register_effect(d, c, e1, false);
    // The equipped monster cannot be destroyed by battle or card effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value(d, e2, 1);
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    card::register_effect(d, c, e3, false);
    // Your opponent cannot target the equipped monster with card effects
    let e4 = effect::clone(d, e2);
    effect::set_code(d, e4, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_value_fn(d, e4, aux::tgoval());
    card::register_effect(d, c, e4, false);
    // Allo the equipped monster to attack directly this turn
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_condition(d, e5, Some(condition(|d, _a| duel::is_able_to_enter_bp(d))));
    effect::set_cost(d, e5, Some(cost_target(dircost)));
    effect::set_operation(d, e5, Some(operation(dirop)));
    effect::set_reset(d, e5, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e5, false);
}
/// `s.eqlimit(e,c)`: `return c==e:GetLabelObject()`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label_object(d, e).as_card() == Some(c)
}
/// `s.dircost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dircost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let mut cards = vec![c];
    if let Some(t) = card::get_equip_target(d, c) {
        cards.push(t);
    }
    let g = group::create_group(&cards);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_ONFIELD, 0, 1, (&g).into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let tg = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_ONFIELD, 0, 1, 1, false, (&g).into()).nil();
    duel::send_to_grave(d, tg, REASON_COST, None, None);
    true
}
/// `s.dirop(e,tp,eg,ep,ev,re,r,rp)`
fn dirop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_property(d, e3, EFFECT_CANNOT_DISABLE, 0);
    effect::set_code(d, e3, EFFECT_DIRECT_ATTACK);
    effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, c, e3, false);
}
