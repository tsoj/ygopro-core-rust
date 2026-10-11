//! Runick Slumber
use super::prelude::*;

pub const ID: u32 = 67835547;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RUNICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateMysteruneQPEffect(c,id,0,s.natktg,s.natkop,3,EFFECT_FLAG_CARD_TARGET)
    let (_, e1) = aux::mysterune::create_mysterune_qp_effect(
        d,
        c,
        ID,
        Some(0),
        Some(cost_target(natktg)),
        Some(condition(natkop)),
        Some(aux::RmCount::N(3)),
        Some(EFFECT_FLAG_CARD_TARGET),
        None,
    );
    card::register_effect(d, c, e1, false);
}
/// `s.natktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn natktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.natkop(e,tp,eg,ep,ev,re,r,rp)`: its return value is read by the Mysterune helper
fn natkop(d: &mut Duel, a: &EffArgs) -> bool {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
        let c = effect::get_handler(d, a.e).nil();
        // Cannot attack this turn
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, aux::stringid(ID, 3));
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        // Destruction protection
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        effect::set_value_fn(d, e2, lua_fn(destruction_value));
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
        return true;
    }
    false
}
/// `function(_,_,r) return (r&REASON_BATTLE+REASON_EFFECT)~=0 end`
fn destruction_value(_d: &mut Duel, a: Args) -> bool {
    let r = a.int(2) as u32;
    (r & (REASON_BATTLE + REASON_EFFECT)) != 0
}
