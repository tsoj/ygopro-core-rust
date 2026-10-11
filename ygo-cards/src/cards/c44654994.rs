//! Artmage Impasto -Recapture-
use super::prelude::*;

pub const ID: u32 = 44654994;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARTMAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you control an "Artmage" monster, you can activate this card the turn it was Set
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e0, EFFECT_TRAP_ACT_IN_SET_TURN);
    effect::set_condition(d, e0, Some(condition(e0_cond)));
    card::register_effect(d, c, e0, false);
    // When your opponent activates a monster effect: Banish 1 Fusion Monster you control (until the End Phase), and if you do, negate the activation, and if you do that, destroy that card, then if you have 3 or more different Monster Types on your field, you can return all Spells/Traps your opponent controls to the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn e0_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ARTMAGE as u16])), p, LOCATION_MZONE, 0, 1, NO_EX)
}
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && duel::is_chain_negatable(d, a.ev as u8)
}
fn banfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_fusion_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&banfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    let rc = effect::get_handler(d, a.re.nil()).nil();
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_MZONE as i32);
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, a.re.nil()) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, (1 - a.tp) as u8, LOCATION_ONFIELD as i32);
    true
}
fn spelltrap_hand(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&banfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    duel::hint_selection(d, g.clone(), true);
    if aux::remove_until(d, g, None, REASON_EFFECT, PHASE_END, ID, a.e, a.tp, Some(aux::default_field_return()), None, None, None, None, None).is_some()
        && duel::negate_activation(d, a.ev as u8)
        && card::is_relate_to_effect(d, effect::get_handler(d, a.re.nil()).nil(), a.re.nil())
        && duel::destroy(d, a.eg(), REASON_EFFECT, None, None) > 0
        && {
            let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX);
            group::get_bin_class_count(d, &g, &|d, c| card::get_race(d, c) as i64) >= 3
        }
        && duel::is_existing_matching_card(d, Some(&spelltrap_hand), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        let rg = duel::get_matching_group(d, Some(&spelltrap_hand), a.tp, 0, LOCATION_ONFIELD, NO_EX);
        if !rg.is_empty() {
            duel::break_effect(d);
            duel::send_to_hand(d, rg, None, REASON_EFFECT, None);
        }
    }
}
