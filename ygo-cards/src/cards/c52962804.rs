//! Dragunity Draft
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 52962804;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGUNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_code(d, e1, CATEGORY_TOHAND as u32);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Your Level 5 or higher "Dragunity" monsters are unaffected when attacking
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(imcon)));
    effect::set_operation(d, e2, Some(operation(imop)));
    card::register_effect(d, c, e2, false);
}
fn thfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_set_card(d, c, &[SET_DRAGUNITY as u16])
        && card::is_monster(d, c)
        && card::is_level_below(d, c, 4)
        && card::is_able_to_hand(d, c, None)
        && card::is_can_be_effect_target(d, c, Some(e))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc, e);
    }
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c, e)), tp, LOCATION_GRAVE, 0, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        duel::set_target_card(d, sg.clone());
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(sg), 1, 0, LOCATION_GRAVE as i32);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if !card::is_relate_to_effect(d, effect::get_handler(d, a.e).nil(), a.e) {
        return;
    }
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
fn imcon(d: &mut Duel, a: &EffArgs) -> bool {
    match duel::get_attacker(d) {
        Some(ac) => {
            card::is_controler(d, ac, a.tp) && card::is_set_card(d, ac, &[SET_DRAGUNITY as u16]) && card::get_original_level(d, ac) >= 5
        }
        None => false,
    }
}
fn imop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_value_fn(d, e1, lua_fn(efilter));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
    let ac = duel::get_attacker(d).nil();
    card::register_effect(d, ac, e1, false);
    duel::adjust_instantly(d, Some(ac));
}
fn efilter(d: &mut Duel, a: Args) -> bool {
    let (e, te) = (a.e(0), a.e(1));
    effect::get_handler_player(d, te) != effect::get_owner_player(d, e)
}
