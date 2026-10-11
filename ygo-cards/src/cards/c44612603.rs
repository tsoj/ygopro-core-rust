//! Survival's End
use super::prelude::*;

pub const ID: u32 = 44612603;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_NORMAL)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_DINOSAUR) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn desfilter2(d: &mut Duel, c: CardId, fid: i64) -> bool {
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&desfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&desfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    let g = duel::get_matching_group(d, Some(&desfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let n = duel::destroy(d, g, REASON_EFFECT, None, None);
    if n != 0 {
        let tg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
        let mut ct = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        if ct < 0 {
            return;
        }
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ct = 1;
        }
        ct = ct.min(n);
        if ct > 0 && !tg.is_empty() {
            duel::break_effect(d);
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let sg = group::select(d, &tg, tp, 1, ct as u16, false, NO_EX).nil();
            if duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
                let sg2 = duel::get_operated_group(d);
                let c = effect::get_handler(d, e).nil();
                let fid = card::get_field_id(d, c);
                let _ = group::get_first(d, &sg2);
                for tc in group::iter(d, &sg2) {
                    card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, fid as i64, 0);
                }
                let e1 = effect::create_effect(d, c);
                effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
                effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
                effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
                effect::set_count_limit(d, e1, 1, 0, 0, 0);
                effect::set_label(d, e1, &[fid as i64]);
                effect::set_label_object(d, e1, Value::Group(sg2));
                effect::set_condition(d, e1, Some(condition(descon2)));
                effect::set_operation(d, e1, Some(operation(desop2)));
                duel::register_effect(d, e1, tp);
            }
        }
    }
}
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    if !group::is_exists(d, &g, &|d, c| desfilter2(d, c, fid), 1, NO_EX) {
        effect::reset(d, a.e);
        return false;
    }
    true
}
fn desop2(d: &mut Duel, a: &EffArgs) {
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    let tg = group::filter(d, &g, &|d, c| desfilter2(d, c, fid), NO_EX);
    duel::destroy(d, tg, REASON_EFFECT, None, None);
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_DINOSAUR)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let mut g1 = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_DINOSAUR)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g2 = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    group::add_card(&mut g1, &g2);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1), 2, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let tg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    if !tg.is_empty() {
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
