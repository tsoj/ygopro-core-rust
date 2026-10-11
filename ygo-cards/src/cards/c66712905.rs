//! Runick Dispelling
use super::prelude::*;

pub const ID: u32 = 66712905;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RUNICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    // local e1,e2=Effect.CreateMysteruneQPEffect(c,id,CATEGORY_TOGRAVE+CATEGORY_HANDES,s.tgtg,s.tgop,2,EFFECT_FLAG_DELAY,EVENT_TO_HAND)
    let (e1, e2) = aux::mysterune::create_mysterune_qp_effect(
        d,
        c,
        ID,
        Some(CATEGORY_TOGRAVE | CATEGORY_HANDES),
        Some(cost_target(tgtg)),
        Some(condition(tgop)),
        Some(aux::RmCount::N(2)),
        Some(EFFECT_FLAG_DELAY),
        Some(EVENT_TO_HAND),
    );
    let e1 = e1.nil();
    effect::set_condition(d, e1, Some(condition(tgcon)));
    card::register_effect(d, c, e1, false);
    card::register_effect(d, c, e2, false);
}
fn tgconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_controler(d, c, 1 - tp) && card::is_previous_location(d, c, LOCATION_DECK)
}
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::get_current_phase(d) != PHASE_DRAW && group::is_exists(d, &a.eg.clone().nil(), &|d, c| tgconfilter(d, c, tp), 1, NO_EX)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.tgop(e,tp,...)`: its return value is read by the Mysterune helper
fn tgop(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    if !g.is_empty() {
        let sg = group::random_select(d, &g, 1 - a.tp, 1);
        return !sg.is_empty() && duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None) > 0;
    }
    false
}
