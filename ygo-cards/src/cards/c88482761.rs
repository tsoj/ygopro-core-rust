//! Dice Roll Battle
use super::prelude::*;

pub const ID: u32 = 88482761;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPEEDROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    //
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(atktg)));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn rmfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !(card::is_set_card(d, c, &[SET_SPEEDROID as u16]) && card::is_able_to_remove(d, c, None, None, None)) {
        return false;
    }
    let lv = card::get_original_level(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| rmfilter2(d, x, e, tp, lv)), tp, LOCATION_HAND, 0, 1, NO_EX)
}
fn rmfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    if !(card::is_set_card(d, c, &[SET_SPEEDROID as u16]) && card::is_type(d, c, TYPE_TUNER) && card::is_able_to_remove(d, c, None, None, None)) {
        return false;
    }
    let lv2 = card::get_original_level(d, c) + lv;
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, lv2)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    card::get_level(d, c) == lv
        && card::is_type(d, c, TYPE_SYNCHRO)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && rmfilter1(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rmfilter1(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| rmfilter1(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    let mut lv = card::get_original_level(d, tc);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let mut g = duel::select_matching_card(d, a.tp, Some(&|d, c| rmfilter2(d, c, a.e, a.tp, lv)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        let first = group::get_first(d, &g).nil();
        lv += card::get_original_level(d, first);
        group::add_card(&mut g, &CardSet::from_card(tc));
        if duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) == 2 {
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, lv)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX);
            if let Some(sg) = sg {
                duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && duel::is_phase(d, PHASE_BATTLE_STEP)
}
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_SYNCHRO) && card::is_position(d, c, POS_FACEUP_ATTACK)
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&atkfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    duel::select_target(d, a.tp, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let g2 = duel::select_target(d, a.tp, Some(&atkfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let first = group::get_first(d, &g2);
    effect::set_label_object(d, a.e, first.map_or(Value::Nil, Value::Card));
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_target_cards(d, a.e).nil();
    if g.len() == 2 {
        let order = group::iter(d, &g);
        let (mut c1, mut c2) = (order[0], order[1]);
        if Some(c1) != effect::get_label_object(d, a.e).as_card() {
            std::mem::swap(&mut c1, &mut c2);
        }
        if card::is_controler(d, c1, 1 - a.tp) && card::is_position(d, c1, POS_FACEUP_ATTACK) && !card::is_immune_to_effect(d, c1, a.e) && card::is_controler(d, c2, a.tp) {
            duel::calculate_damage(d, c1, Some(c2), true);
        }
    }
}
