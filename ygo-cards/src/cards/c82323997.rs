//! Red Nova Dragon/Assault Mode
use super::prelude::*;

pub const ID: u32 = 82323997;

const CARD_RED_NOVA_DRAGON: u32 = 97489701;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ASSAULT_MODE, CARD_RED_NOVA_DRAGON], assault_mode: Some(CARD_RED_NOVA_DRAGON), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    card::add_must_be_special_summoned(d, c);
    // Banish cards your opponent controls, up to the number of Tuners in your GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(|_d, a| a.rp == 1 - a.tp)));
    effect::set_cost(d, e1, Some(cost_target(tempbanishcost)));
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    card::register_effect(d, c, e1, false);
    // Negate the attack and end Battle Phase
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let at = duel::get_attacker(d).nil();
        card::is_controler(d, at, 1 - a.tp)
    })));
    effect::set_cost(d, e2, Some(cost_target(tempbanishcost)));
    effect::set_operation(d, e2, Some(operation(negbpop)));
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Red Nova Dragon" from your GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_DESTROYED);
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
fn returnop() -> aux::DelayedFn {
    std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
        aux::default_field_return_op(d, ag);
        true
    })
}
fn tempbanishcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None);
    }
    // Banish this card until the End Phase
    aux::remove_until(d, c, None, REASON_COST, PHASE_END, ID, a.e, a.tp, Some(returnop()), None, None, None, None, None);
    true
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_type(d, c, TYPE_TUNER)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let max_ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_type(d, c, TYPE_TUNER)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    if max_ct == 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD, 1, max_ct as u16, false, NO_EX).nil();
    if !g.is_empty() {
        duel::hint_selection(d, g.clone(), true);
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
fn negbpop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_attack(d) {
        duel::break_effect(d);
        duel::skip_phase(d, 1 - a.tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_RED_NOVA_DRAGON]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
