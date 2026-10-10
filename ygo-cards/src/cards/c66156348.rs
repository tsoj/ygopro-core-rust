//! Saga of the Dragon Emperor
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 66156348;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Double ATK
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // Special Summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
/// `aux.FaceupFilter(Card.IsRace,RACE_DRAGON)`
fn faceup_dragon(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON)
}
/// `s.atktg(e,tp,...,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, a.tp)
            && card::is_faceup(d, chkc)
            && card::is_race(d, chkc, RACE_DRAGON);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&faceup_dragon), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    duel::select_target(d, a.tp, Some(&faceup_dragon), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,...)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    // Double ATK
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    let atk = card::get_attack(d, tc) as i64;
    effect::set_value(d, e1, atk * 2);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, tc, e1, false);
    if duel::is_existing_matching_card(d, Some(&faceup_dragon), a.tp, 0, LOCATION_MZONE, 1, NO_EX) {
        // Cannot negate activation of effects
        let e2 = effect::create_effect(d, c);
        effect::set_description(d, e2, aux::stringid(ID, 2));
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e2, EFFECT_CANNOT_INACTIVATE);
        effect::set_range(d, e2, LOCATION_MZONE);
        effect::set_value_fn(d, e2, lua_fn(cannot_inactivate));
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
    }
}
/// `function(e,ct) return e:GetHandler()==Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT):GetHandler() end`
fn cannot_inactivate(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    let ct = a.int(1) as u8;
    let te = duel::get_chain_info(d, ct, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
    let th = effect::get_handler(d, te).nil();
    h == th
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_race(d, c, RACE_DRAGON)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
