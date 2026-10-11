//! The Monarchs Erupt
use super::prelude::*;

pub const ID: u32 = 48716527;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_SELF_TOGRAVE)
    duel::enable_global_flag(d, GLOBALFLAG_SELF_TOGRAVE);
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(actcon)));
    card::register_effect(d, c, e1, false);
    // disable
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(distg)));
    effect::set_code(d, e2, EFFECT_DISABLE);
    card::register_effect(d, c, e2, false);
    // (self to grave)
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e3, EFFECT_SELF_TOGRAVE);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_condition(d, e3, Some(condition(tgcon)));
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTributeSummoned()
    card::is_tribute_summoned(d, c)
}
/// `s.actcon(e,tp,eg,ep,ev,re,r,rp)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_EXTRA,0)==0
    //     and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::get_field_group_count(d, a.tp, LOCATION_EXTRA, 0) == 0 && duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsTributeSummoned()
    !card::is_tribute_summoned(d, c)
}
/// `s.tgcon(e)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_END)
    //     and not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_turn_player(d, tp) && duel::is_phase(d, PHASE_END) && !duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
