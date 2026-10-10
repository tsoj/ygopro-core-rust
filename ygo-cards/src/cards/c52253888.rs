//! Light Law Medium
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 52253888;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Targets must attack this card, if able
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE_START);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(e1_condition)));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // Negate the attack and inflict damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(negcon)));
    effect::set_target(d, e2, Some(cost_target(negtg)));
    effect::set_operation(d, e2, Some(operation(negop)));
    card::register_effect(d, c, e2, false);
}
/// `function(_,tp) return Duel.IsTurnPlayer(1-tp) end`
fn e1_condition(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.atkfilter(c,e)`
fn atkfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_attack_pos(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.atktg(e,tp,...,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_attack_pos(d, chkc);
    }
    let g = duel::get_matching_group(d, Some(&|d, c| atkfilter(d, c, a.e)), a.tp, 0, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        return g.len() > 0;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    duel::select_target(d, a.tp, Some(&|d, c| atkfilter(d, c, a.e)), a.tp, 0, LOCATION_MZONE, 1, g.len() as u16, false, NO_EX);
    true
}
/// `s.atkop(e,tp,...)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tg = duel::get_target_cards(d, a.e).nil();
    if tg.is_empty() {
        return;
    }
    for tc in group::iter(d, &tg) {
        let label = card::get_field_id(d, c) as i64;
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, label, 0);
    }
    // Must attack this card, if able
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_MUST_ATTACK);
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(effect_card_fn(musttg)));
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_MUST_ATTACK_MONSTER);
    effect::set_value_fn(d, e2, lua_fn(must_attack_monster_value));
    card::register_effect(d, c, e2, false);
}
/// `function(e,c) return c==e:GetHandler() end`
fn must_attack_monster_value(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    a.card(1) == Some(h)
}
/// `s.musttg(e,c)`
fn musttg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let label = card::get_flag_effect_label(d, c, ID).first().copied();
    let h = effect::get_handler(d, e).nil();
    match label {
        Some(l) => l == card::get_field_id(d, h) as i64,
        None => false,
    }
}
/// `s.negcon(e,tp,...)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (bc0, bc1) = duel::get_battle_monster(d, a.tp);
    let h = effect::get_handler(d, a.e).nil();
    if bc0 != Some(h) {
        return false;
    }
    match bc1 {
        Some(b) => card::is_attack_pos(d, b) && card::get_base_attack(d, b) > 0,
        None => false,
    }
}
/// `s.negtg(e,tp,...,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let (_, bc1) = duel::get_battle_monster(d, a.tp);
    let bc1 = bc1.nil();
    let amount = lua_int(card::get_base_attack(d, bc1) as f64 / 2.0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, amount as i32);
    true
}
/// `s.negop(e,tp,...)`
fn negop(d: &mut Duel, a: &EffArgs) {
    if !duel::negate_attack(d) {
        return;
    }
    let (_, bc1) = duel::get_battle_monster(d, a.tp);
    let bc1 = bc1.nil();
    if !(card::is_faceup(d, bc1) && card::is_controler(d, bc1, 1 - a.tp) && card::is_relate_to_battle(d, bc1)) {
        return;
    }
    let amount = lua_int(card::get_base_attack(d, bc1) as f64 / 2.0);
    duel::damage(d, 1 - a.tp, amount, REASON_EFFECT, false, None);
}
