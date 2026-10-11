//! Blackwing - Kogarashi the Wanderer
use super::prelude::*;

pub const ID: u32 = 49460512;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_BLACKWING],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // activate limit
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // (the Lua sets EFFECT_FLAG_IGNORE_IMMUNE first and then overwrites the property)
    effect::set_property(d, e2, EFFECT_FLAG_EVENT_PLAYER, 0);
    effect::set_condition(d, e2, Some(condition(limitcon)));
    effect::set_operation(d, e2, Some(operation(limitop)));
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_BLACKWING as u16])
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
        && (card::get_reason(d, c) & (REASON_DESTROY | REASON_EFFECT)) == (REASON_DESTROY | REASON_EFFECT)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
fn limitcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return r==REASON_SYNCHRO
    a.r == REASON_SYNCHRO
}
fn limitop(d: &mut Duel, _a: &EffArgs) {
    // Duel.SetChainLimitTillChainEnd(s.chainlm)
    duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
}
/// `s.chainlm(e,rp,tp)`: `return tp==rp`
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    a.p(2) == a.p(1)
}
