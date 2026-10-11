//! Full-Armored Xyz
use super::prelude::*;

pub const ID: u32 = 39733924;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_condition(d, e1, Some(condition(cond1)));
    effect::set_target(d, e1, Some(cost_target(xyztg)));
    effect::set_operation(d, e1, Some(operation(xyzop)));
    card::register_effect(d, c, e1, false);
    // Equip 1 of your Xyz Monsters to another of your Xyz Monsters
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_EQUIP);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER_E));
    effect::set_condition(d, e2, Some(condition(cond2)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    effect::set_operation(d, e2, Some(operation(eqop)));
    card::register_effect(d, c, e2, false);
}
fn cond1(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_XYZ))), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
fn cond2(d: &mut Duel, _a: &EffArgs) -> bool {
    !(duel::is_phase(d, PHASE_DAMAGE) && duel::is_damage_calculated(d))
}
fn xyztg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_xyz_summonable(d, c, None, None, 0, 0)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn xyzop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_xyz_summonable(d, c, None, None, 0, 0)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    if let Some(sc) = sc {
        duel::xyz_summon(d, a.tp, sc, None, None, 0, 0);
    }
}
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into())
}
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && !card::is_forbidden(d, c)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && tgfilter(d, chkc, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, a.tp, Some(&|d, x| tgfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
        let g = duel::select_matching_card(
            d,
            a.tp,
            Some(&aux::necro_valley_filter(|d, x| eqfilter(d, x))),
            a.tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            tc.into(),
        )
        .nil();
        let eq = group::get_first(d, &g);
        if let Some(eq) = eq {
            if duel::equip(d, a.tp, eq, tc, true, false) {
                let _c = effect::get_handler(d, a.e).nil();
                // Equip limit
                let e1 = effect::create_effect(d, eq);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
                effect::set_value_fn(d, e1, lua_fn(move |_d, args: Args| args.c(1) == tc));
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, eq, e1, false);
                // The equipped monster gains ATK equal to this card's
                let atk = card::get_text_attack(d, eq);
                let atk = if atk < 0 { 0 } else { atk };
                let e2 = effect::create_effect(d, eq);
                effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
                effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
                effect::set_value(d, e2, atk as i64);
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, eq, e2, false);
                // If the equipped monster would be destroyed by battle or card effect, destroy this card instead
                let e3 = effect::create_effect(d, eq);
                effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
                effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
                effect::set_code(d, e3, EFFECT_DESTROY_SUBSTITUTE);
                effect::set_value_fn(d, e3, lua_fn(|_d, args: Args| (args.int(2) & (REASON_BATTLE | REASON_EFFECT) as i64) > 0));
                effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, eq, e3, false);
            }
        }
    }
}
