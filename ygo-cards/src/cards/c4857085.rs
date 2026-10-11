//! Omega Goggles
use super::prelude::*;

pub const ID: u32 = 4857085;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,0)
    aux::add_equip_procedure(d, c, Some(0), None, None, None, None, None, None, None);
    // confirm
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(s.cfcost)
    effect::set_cost(d, e3, Some(cost_target(cfcost)));
    // e3:SetTarget(s.cftg)
    effect::set_target(d, e3, Some(cost_target(cftg)));
    // e3:SetOperation(s.cfop)
    effect::set_operation(d, e3, Some(operation(cfop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cfcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():GetEquipTarget():GetAttackAnnouncedCount()==0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let tc = card::get_equip_target(d, h).nil();
        return card::get_attack_announced_count(d, tc) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_OATH, 0);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e:GetHandler():RegisterEffect(e1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_effect(d, h, e1, false);
    true
}
/// `s.cftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0;
    }
    true
}
/// `s.cfop(e,tp,eg,ep,ev,re,r,rp)`
fn cfop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=g:RandomSelect(tp,1)
    let sg = group::random_select(d, &g, a.tp, 1);
    // Duel.ConfirmCards(tp,sg)
    duel::confirm_cards(d, a.tp, sg);
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - a.tp);
}
