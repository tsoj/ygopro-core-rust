//! Stardust Phantom
use super::prelude::*;

pub const ID: u32 = 80244114;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_STARDUST_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // indes
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(indtg)));
    effect::set_operation(d, e2, Some(operation(indop)));
    card::register_effect(d, c, e2, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    a.rp != a.tp && card::is_previous_location(d, h, LOCATION_ONFIELD) && card::is_reason(d, h, REASON_DESTROY)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_STARDUST_DRAGON]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
fn indfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_SYNCHRO)
}
fn indtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && indfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&indfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let _ = duel::select_target(d, a.tp, Some(&indfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn valcon(d: &mut Duel, a: Args) -> bool {
    // function s.valcon(e,re,r,rp)
    if (a.int(2) as u32 & REASON_BATTLE) != 0 {
        let h = effect::get_handler(d, a.e(0)).nil();
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
        true
    } else {
        false
    }
}
fn addown(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, h, ID) == 0 {
        return;
    }
    let owner = effect::get_owner(d, a.e).nil();
    let e1 = effect::create_effect(d, owner);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, -800);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, h, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, h, e2, false);
}
fn indop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
            effect::set_range(d, e1, LOCATION_MZONE);
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            effect::set_value_fn(d, e1, lua_fn(valcon));
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
            let e2 = effect::create_effect(d, h);
            effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
            effect::set_operation(d, e2, Some(operation(addown)));
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e2, false);
        }
    }
}
