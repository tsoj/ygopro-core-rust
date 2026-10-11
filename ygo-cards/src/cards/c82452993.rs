//! Lone Wolf
use super::prelude::*;

pub const ID: u32 = 82452993;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
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
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(8508055,3810071,49814180)
    card::is_faceup(d, c) && card::is_code(d, c, &[8508055, 3810071, 49814180])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    // if chk==0 then return #g==1 and s.filter(g:GetFirst()) end
    if a.chk == Some(0) {
        return g.len() == 1 && filter(d, group::get_first(d, &g).nil());
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, tc.nil()) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc.nil());
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_OWNER_RELATE)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_OWNER_RELATE, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetCondition(s.rcon)
        effect::set_condition(d, e1, Some(lua_fn(rcon)));
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc.nil(), e1, true);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_IMMUNE_EFFECT)
        effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
        // e2:SetValue(s.efilter)
        effect::set_value_fn(d, e2, lua_fn(efilter));
        // tc:RegisterEffect(e2,true)
        card::register_effect(d, tc.nil(), e2, true);
    }
}
/// `s.rcon(e)`
fn rcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return e:GetOwner():IsHasCardTarget(e:GetHandler())
    let owner = effect::get_owner(d, e).nil();
    let h = effect::get_handler(d, e).nil();
    card::is_has_card_target(d, owner, h)
}
/// `s.efilter(e,re)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.v(1).as_effect().nil();
    // return re:IsMonsterEffect() and re:GetOwnerPlayer()~=e:GetOwnerPlayer()
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    effect::get_owner_player(d, re) != effect::get_owner_player(d, e)
}
