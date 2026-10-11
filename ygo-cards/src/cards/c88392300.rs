//! Wattkinetic Puppeteer
//! 電送擬人エレキネシス
use super::prelude::*;

pub const ID: u32 = 88392300;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // move
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_HAND | LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e2, Some(cost::self_to_grave()));
    // e2:SetTarget(s.seqtg)
    effect::set_target(d, e2, Some(cost_target(seqtg)));
    // e2:SetOperation(s.seqop)
    effect::set_operation(d, e2, Some(operation(seqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.seqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn seqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // Duel.SelectTarget(tp,nil,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.seqop(e,tp,eg,ep,ev,re,r,rp)`
fn seqop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsControler(tp) or tc:IsImmuneToEffect(e)
    //     or Duel.GetLocationCount(1-tp,LOCATION_MZONE)<=0 then return end
    if !card::is_relate_to_effect(d, tc, a.e)
        || card::is_controler(d, tc, a.tp)
        || card::is_immune_to_effect(d, tc, a.e)
        || duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOZONE as u64);
    // Duel.MoveSequence(tc,math.log(Duel.SelectDisableField(tp,1,0,LOCATION_MZONE,0)>>16,2))
    let zone = duel::select_disable_field(d, a.tp, 1, 0, LOCATION_MZONE, Some(0), false);
    // A nil zone is a Lua error at the shift
    if zone == 0 {
        crate::lua_error("attempt to perform bitwise operation on a nil value");
    }
    let seq = ((zone >> 16) as f64).log2() as u8;
    duel::move_sequence(d, tc, seq, None);
}
