//! Pendulum Hole
use super::prelude::*;

pub const ID: u32 = 68477598;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate(summon)
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SPSUMMON);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPendulumSummoned()
    card::is_pendulum_summoned(d, c)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)==0 and eg:IsExists(s.cfilter,1,nil)
    let eg = a.eg.clone().nil();
    duel::get_current_chain(d, true) == 0 && group::is_exists(d, &eg, &cfilter, 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,0,0)
    let eg = a.eg.clone().nil();
    let n = eg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(eg), n, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, a.eg());
    // Duel.SendtoDeck(eg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, a.eg(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
