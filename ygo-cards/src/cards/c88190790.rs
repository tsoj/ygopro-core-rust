//! Assault Armor
use super::prelude::*;

pub const ID: u32 = 88190790;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip
    // aux.AddEquipProcedure(c,0,s.eqfilter,s.eqlimit)
    aux::add_equip_procedure(
        d,
        c,
        Some(0),
        Some(aux::equip_filter_fn(eqfilter)),
        Some(lua_fn(eqlimit)),
        None,
        None,
        None,
        None,
        None,
    );
    // Atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(300)
    effect::set_value(d, e1, 300);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can make a second attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.macon)
    effect::set_condition(d, e2, Some(condition(macon)));
    // e2:SetCost(s.macost)
    effect::set_cost(d, e2, Some(cost_target(macost)));
    // e2:SetOperation(s.maop)
    effect::set_operation(d, e2, Some(operation(maop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    let (e, c) = (a.e(0), a.c(1));
    // if e:GetHandler():GetEquipTarget()==c then return true end
    let h = effect::get_handler(d, e).nil();
    if card::get_equip_target(d, h) == Some(c) {
        return true;
    }
    // local g=Duel.GetFieldGroup(e:GetHandlerPlayer(),LOCATION_MZONE,0)
    let hp = effect::get_handler_player(d, e);
    let g = duel::get_field_group(d, hp, LOCATION_MZONE, 0);
    // return c:IsRace(RACE_WARRIOR) and #g==1 and g:GetFirst()==c
    card::is_race(d, c, RACE_WARRIOR) && g.len() == 1 && group::get_first(d, &g) == Some(c)
}
/// `s.eqfilter(c,e,tp)`
fn eqfilter(d: &mut Duel, c: CardId, _e: EffectId, tp: u8) -> bool {
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
    // return c:IsRace(RACE_WARRIOR) and #g==1 and g:GetFirst()==c
    card::is_race(d, c, RACE_WARRIOR) && g.len() == 1 && group::get_first(d, &g) == Some(c)
}
/// `s.macon(e,tp,eg,ep,ev,re,r,rp)`
fn macon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsAbleToEnterBP()
    duel::is_able_to_enter_bp(d)
}
/// `s.macost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn macost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c);
    }
    // Duel.SetTargetCard(c:GetEquipTarget())
    let target = card::get_equip_target(d, c).nil();
    duel::set_target_card(d, target);
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
/// `s.maop(e,tp,eg,ep,ev,re,r,rp)`
fn maop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_EXTRA_ATTACK)
            effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
