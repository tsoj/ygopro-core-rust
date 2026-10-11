//! Runick Smiting Storm
use super::prelude::*;

pub const ID: u32 = 93229151;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RUNICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateMysteruneQPEffect(c,id,0,s.rmtg,s.rmop,1)
    let (_, e1) = aux::mysterune::create_mysterune_qp_effect(
        d,
        c,
        ID,
        Some(0),
        Some(cost_target(rmtg)),
        Some(operation(rmop)),
        Some(aux::RmCount::N(1)),
        None,
        None,
    );
    card::register_effect(d, c, e1, false);
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) > 0;
    }
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let max = std::cmp::min(duel::get_field_group_count(d, tp, 0, LOCATION_DECK), duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD));
    if max < 1 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
    let ct = duel::announce_number_range(d, tp, Some(1), Some(max as u32), &[]).nil().0;
    if ct < 1 {
        return;
    }
    let rg = duel::get_decktop_group(d, 1 - tp, ct as u32);
    if !rg.is_empty() {
        duel::disable_shuffle_check(d, true);
        duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
