//! Supreme King Dragon Dark Rebellion
use super::prelude::*;

pub const ID: u32 = 42160203;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPREME_KING_GATE, SET_SUPREME_KING_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, xyz, sumtype, tp| card::is_type_ctx(d, c, TYPE_PENDULUM, xyz, sumtype, Some(tp)) && card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, xyz, sumtype, Some(tp)))), Some(4), 2, None, None, None, None, false, None);
    // change atk
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_CONFIRM);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_BATTLE_START as u32));
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    match bc {
        Some(bc) => card::is_faceup(d, bc) && card::is_relate_to_battle(d, bc) && card::get_attack(d, bc) > 0,
        None => false,
    }
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, c);
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_battle(d, tc) && !card::is_immune_to_effect(d, tc, a.e) {
            let atk = card::get_base_attack(d, tc);
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            effect::set_value(d, e1, 0);
            card::register_effect(d, tc, e1, false);
            if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
                let e2 = effect::create_effect(d, c);
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
                effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
                effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
                effect::set_value(d, e2, atk as i64);
                card::register_effect(d, c, e2, false);
            }
        }
    }
}
fn spcon(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_extra_as_cost(d, c);
    }
    duel::send_to_deck(d, c, None, SEQ_DECKTOP as i32, REASON_COST, None);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SUPREME_KING_GATE as u16, SET_SUPREME_KING_DRAGON as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count_from_ex(d, tp, Some(tp), h.into(), duel::ExtraCard::None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let mut ft = std::cmp::min(duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0, 2);
    if ft <= 0 {
        return;
    }
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    let gate = aux::check_summon_gate_limit(d, tp).unwrap_or(ft as i64);
    ft = std::cmp::min(ft as i64, gate) as i32;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, ft as u16, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
