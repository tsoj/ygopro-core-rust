//! Runick Flashing Fire
use super::prelude::*;

pub const ID: u32 = 68957034;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RUNICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateMysteruneQPEffect(c,id,CATEGORY_DESTROY,s.destg,s.desop,2,EFFECT_FLAG_CARD_TARGET)
    let (_, e1) = aux::mysterune::create_mysterune_qp_effect(
        d,
        c,
        ID,
        Some(CATEGORY_DESTROY),
        Some(cost_target(destg)),
        Some(condition(desop)),
        Some(aux::RmCount::N(2)),
        Some(EFFECT_FLAG_CARD_TARGET),
        None,
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsSpecialSummoned() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_special_summoned(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsSummonType,tp,0,LOCATION_MZONE,1,nil,SUMMON_TYPE_SPECIAL) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL])), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsSummonType,tp,0,LOCATION_MZONE,1,1,nil,SUMMON_TYPE_SPECIAL)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL])), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`: its return value is read by the Mysterune helper
fn desop(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // return tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0
    if !card::is_relate_to_effect(d, tc, a.e) {
        return false;
    }
    duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
}
