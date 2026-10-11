//! Column Switch
use super::prelude::*;

pub const ID: u32 = 37480144;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsInMainMZone(tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_in_main_m_zone(d, chkc, Some(a.tp));
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_MMZONE,0,1,nil)
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_MMZONE, 0, 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // Duel.SelectTarget(tp,nil,tp,LOCATION_MMZONE,0,1,1,nil)
    let _ = duel::select_target(d, a.tp, None, a.tp, LOCATION_MMZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsControler(1-tp) or tc:IsImmuneToEffect(e) or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if !card::is_relate_to_effect(d, tc, a.e)
        || card::is_controler(d, tc, 1 - a.tp)
        || card::is_immune_to_effect(d, tc, a.e)
        || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOZONE as u64);
    // Duel.MoveSequence(tc,math.log(Duel.SelectDisableField(tp,1,LOCATION_MZONE,0,0),2))
    let zone = duel::select_disable_field(d, a.tp, 1, LOCATION_MZONE, 0, Some(0), false);
    // A nil zone (no free zone) makes math.log raise a Lua error.
    if zone == 0 {
        crate::lua_error("bad argument #1 to 'log' (number expected, got nil)");
    }
    let seq = (zone as f64).log2() as u8;
    duel::move_sequence(d, tc, seq, None);
}
