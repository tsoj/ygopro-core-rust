//! Numbing Grub in the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 92065772;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --dis field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE,PLAYER_NONE,0)<=0 and Duel.GetLocationCount(1-tp,LOCATION_MZONE,PLAYER_NONE,0)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0 <= 0
        && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0 <= 0
    {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_DISABLE_FIELD)
    effect::set_code(d, e1, EFFECT_DISABLE_FIELD);
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(lua_fn(disop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
}
/// `s.disop(e,tp)`
fn disop(d: &mut Duel, a: Args) -> u32 {
    let tp = a.p(1);
    // local dis1=Duel.SelectDisableField(tp,1,LOCATION_MZONE,LOCATION_MZONE,0)
    // return dis1
    duel::select_disable_field(d, tp, 1, LOCATION_MZONE, LOCATION_MZONE, Some(0), false)
}
