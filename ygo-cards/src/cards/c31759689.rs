//! Tindangle Hound
use super::prelude::*;

pub const ID: u32 = 31759689;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Increase its ATK and change a monster to face-down Defense Position
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
    // Descrease the ATK of monsters your opponent controls
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_value_fn(d, e2, effect_card_value(val));
    card::register_effect(d, c, e2, false);
    // Change 1 monster to face-up Defense Position
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_category(d, e3, CATEGORY_POSITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_condition(d, e3, Some(condition(poscon)));
    effect::set_target(d, e3, Some(cost_target(postg)));
    effect::set_operation(d, e3, Some(operation(posop)));
    card::register_effect(d, c, e3, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_can_turn_set(d, c) && card::get_base_attack(d, c) > 0
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let tc = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into()).nil();
    let first = group::get_first(d, &tc).nil();
    let atk = card::get_base_attack(d, first);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(c)), 1, tp, atk);
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(tc), 1, tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let atk = card::get_base_attack(d, tc);
        if card::update_attack(d, c, atk as i64, None, None, None) == atk {
            duel::break_effect(d);
            duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
fn valfilter(d: &mut Duel, c: CardId, rc: CardId) -> bool {
    let lg = card::get_linked_group(d, c);
    group::is_contains(&lg, rc)
}
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local lg=Duel.GetMatchingGroup(s.valfilter,0,LOCATION_MZONE,LOCATION_MZONE,c,c)
    let mut lg = duel::get_matching_group(d, Some(&|d, tc| valfilter(d, tc, c)), 0, LOCATION_MZONE, LOCATION_MZONE, c.into());
    // lg:Merge(c:GetLinkedGroup():Filter(Card.IsMonster,nil))
    let linked = card::get_linked_group(d, c);
    let mons = group::filter(d, &linked, &|d, x| card::is_monster(d, x), NO_EX);
    group::add_card(&mut lg, &mons);
    // return #lg*-1000
    lg.len() as i64 * -1000
}
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_DESTROY) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) && card::is_can_change_position(d, c)
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && posfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&posfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    let tc = duel::select_target(d, tp, Some(&posfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(tc), 1, tp, POS_FACEUP_DEFENSE as i32);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && !card::is_position(d, tc, POS_FACEUP_DEFENSE) {
        duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
