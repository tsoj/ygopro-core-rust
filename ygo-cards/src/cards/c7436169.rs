//! Trivikarma
use super::prelude::*;

pub const ID: u32 = 7436169;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_VISAS_STARFROST, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate target's effect and increase the ATK of "Visas Starfrost"
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Search 1 card that lists "Visas Starfrost"
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
        && ((card::is_code(d, c, &[CARD_VISAS_STARFROST]) && card::is_controler(d, c, tp))
            || (card::is_controler(d, c, 1 - tp) && card::is_type(d, c, TYPE_EFFECT) && card::is_negatable_monster(d, c)))
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX) == 1, false)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    let rg = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    let tg = aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    duel::set_target_card(d, tg.clone());
    let hg = group::filter(d, &tg, &|d, c| card::is_controler(d, c, a.tp), NO_EX);
    let lbl = group::get_first(d, &hg).map_or(Value::Nil, Value::Card);
    effect::set_label_object(d, a.e, lbl);
    let dg = group::filter(d, &tg, &|d, c| card::is_controler(d, c, 1 - a.tp), NO_EX);
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(dg), n, 0, 0);
    let hn = hg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(hg), hn, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_target_cards(d, a.e).nil();
    if g.is_empty() {
        return;
    }
    let v: Vec<CardId> = group::iter(d, &g);
    let mut tc1 = v.first().copied();
    let mut tc2 = v.get(1).copied();
    let lbl = effect::get_label_object(d, a.e).as_card();
    if tc2 == lbl {
        std::mem::swap(&mut tc1, &mut tc2);
    }
    if let Some(tc2) = tc2 {
        if card::is_controler(d, tc2, 1 - a.tp) && card::is_faceup(d, tc2) && card::is_can_be_disabled_by_effect(d, tc2, a.e, true) {
            let c = effect::get_handler(d, a.e).nil();
            duel::negate_related_chain(d, tc2, RESET_TURN_SET);
            // Negate the effects of the opponent's monster
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc2, e1, false);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc2, e2, false);
            let val = card::get_base_attack(d, tc2).max(card::get_base_defense(d, tc2)) as f64 / 2.0;
            if val == 0.0 {
                return;
            }
            // Your monster gains half of the original ATK/DEF (whichever is higher)
            if let Some(tc1) = tc1 {
                if card::is_controler(d, tc1, a.tp) && card::is_relate_to_effect(d, tc1, a.e) && card::is_faceup(d, tc1) {
                    card::update_attack(d, tc1, lua_int(val), Some(RESET_EVENT | RESETS_STANDARD), Some(c), None);
                }
            }
        }
    }
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::lists_code(d, c, &[CARD_VISAS_STARFROST]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
