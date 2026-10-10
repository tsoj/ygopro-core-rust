//! The Breaking Ruin God
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 85182315;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_OBELISK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local EFFECT_FLAG_CANNOT_NEGATE_ACTIV_EFF=EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE+EFFECT_FLAG_CANNOT_INACTIVATE
    let effect_flag_cannot_negate_activ_eff = EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE + EFFECT_FLAG_CANNOT_INACTIVATE;
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, effect_flag_cannot_negate_activ_eff, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Banish all monsters in your opponent's GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, effect_flag_cannot_negate_activ_eff + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_RELEASE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(rmcon)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    effect::set_operation(d, e2, Some(operation(rmop)));
    card::register_effect(d, c, e2, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_OBELISK]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_GRAVE | LOCATION_HAND) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let f = aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp));
    let sg = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sg);
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // Unaffected by your opponent's card effects this turn
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_description(d, e1, 3110);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
            effect::set_range(d, e1, LOCATION_MZONE);
            effect::set_value_fn(d, e1, lua_fn(immune_value));
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `function(e,re) return e:GetOwnerPlayer()==1-re:GetOwnerPlayer() end`
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let (e, re) = (a.e(0), a.e(1));
    effect::get_owner_player(d, e) == 1 - effect::get_owner_player(d, re)
}
fn rmcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_previous_controler(d, c, tp)
}
fn oblskfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    rmcfilter(d, c, tp) && card::is_previous_position(d, c, POS_FACEUP) && card::get_previous_code_on_field(d, c).first().copied() == Some(CARD_OBELISK)
}
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if !(a.rp == tp && (a.r & REASON_COST) == REASON_COST) {
        return false;
    }
    let re = match a.re {
        Some(re) => re,
        None => return false,
    };
    if !effect::is_activated(d, re) {
        return false;
    }
    let eg = a.eg.clone().nil();
    if !group::is_exists(d, &eg, &|d, c| rmcfilter(d, c, tp), 2, NO_EX) {
        return false;
    }
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_OBELISK]))), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        || group::is_exists(d, &eg, &|d, c| oblskfilter(d, c, tp), 1, NO_EX)
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&rmfilter), tp, 0, LOCATION_GRAVE, NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, n as i32 * 500);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&rmfilter), tp, 0, LOCATION_GRAVE, NO_EX);
    if duel::remove(d, &g, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        let operated = duel::get_operated_group(d);
        let ct = group::filter_count(d, &operated, &|d, c| card::is_location(d, c, LOCATION_REMOVED), NO_EX);
        duel::damage(d, 1 - tp, ct as i64 * 500, REASON_EFFECT, false, None);
    }
}
