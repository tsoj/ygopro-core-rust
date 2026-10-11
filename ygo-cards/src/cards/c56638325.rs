//! Stellarknight Delteros
use super::prelude::*;

pub const ID: u32 = 56638325;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TELLARKNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // act limit
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(limcon)));
    effect::set_operation(d, e1, Some(operation(limop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EVENT_CHAIN_END);
    effect::set_operation(d, e3, Some(operation(limop2)));
    card::register_effect(d, c, e3, false);
    // destroy
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_DESTROY);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_cost(d, e4, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
    // spsummon
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e5, EVENT_TO_GRAVE);
    effect::set_property(d, e5, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_condition(d, e5, Some(condition(spcon)));
    effect::set_target(d, e5, Some(cost_target(sptg)));
    effect::set_operation(d, e5, Some(operation(spop)));
    card::register_effect(d, c, e5, false);
}
fn limcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    card::get_overlay_count(d, h) > 0 && group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, tp), 1, NO_EX)
}
fn limop(d: &mut Duel, a: &EffArgs) {
    if duel::get_current_chain(d, false) == 0 {
        duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
    } else if duel::get_current_chain(d, false) == 1 {
        let h = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
fn limop2(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::get_overlay_count(d, h) > 0 && card::get_flag_effect(d, h, ID) != 0 {
        duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
    }
    card::reset_flag_effect(d, h, ID);
}
/// `s.chainlm(e,rp,tp)`: `tp==rp`
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    a.p(2) == a.p(1)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
