//! Unending Nightmare
use super::prelude::*;

pub const ID: u32 = 69452756;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Destroy 1 face-up Spell/Trap on the field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E | TIMING_EQUIP));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=not c:IsStatus(STATUS_EFFECT_ENABLED) and c or nil
    let exc = if !card::is_status(d, c, STATUS_EFFECT_ENABLED) { Some(c) } else { None };
    // if chkc then return chkc:IsOnField() and chkc:IsSpellTrap() and chkc:IsFaceup() and (not exc or chkc~=exc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc)
            && card::is_spell_trap(d, chkc)
            && card::is_faceup(d, chkc)
            && exc.map_or(true, |x| chkc != x);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSpellTrap),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,exc) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_spell_trap(d, c))),
            a.tp,
            LOCATION_ONFIELD,
            LOCATION_ONFIELD,
            1,
            exc.map_or(NO_EX, |x| x.into()),
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsSpellTrap),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(
        d,
        a.tp,
        Some(&aux::faceup_filter(|d, c| card::is_spell_trap(d, c))),
        a.tp,
        LOCATION_ONFIELD,
        LOCATION_ONFIELD,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.Destroy(tc,REASON_EFFECT) end
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
