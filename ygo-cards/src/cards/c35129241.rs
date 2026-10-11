//! Stardust Wish
use super::prelude::*;

pub const ID: u32 = 35129241;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_STARDUST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_RELEASE);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Cannot be destroyed by battle
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e3, Some(effect_card_fn(intg)));
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let Some(re) = card::get_reason_effect(d, c) else {
        return false;
    };
    let rc = effect::get_handler(d, re).nil();
    card::is_set_card(d, c, &[SET_STARDUST as u16])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_COST)
        && rc == c
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, e, tp), 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        let eg = a.eg.clone().nil();
        return group::is_contains(&eg, chkc) && filter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    duel::set_target_card(d, a.eg());
    let eg = a.eg.clone().nil();
    let n = eg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(eg), n, 0, 0);
    duel::set_chain_limit(d, lua_fn(chainlm));
    true
}
/// `function(e,rp,tp) return tp==rp end`
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    a.int(2) == a.int(1)
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tc = duel::first_target(d);
    let h = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, h, e) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, e) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
                // A number is always true here, so the flag is set once the summon runs
                duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
                card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
            }
        }
    }
}
fn intg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_attack_pos(d, c) && card::get_flag_effect(d, c, ID) != 0
}
