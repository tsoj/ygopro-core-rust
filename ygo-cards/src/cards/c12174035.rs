//! Hydro Pressure Cannon
//! ハイドロプレッシャーカノン
use super::prelude::*;

pub const ID: u32 = 12174035;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,s.filter)
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))), None, None, None, None, None, None);
    // handes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetCondition(s.hdcon)
    effect::set_condition(d, e3, Some(condition(hdcon)));
    // e3:SetTarget(s.hdtg)
    effect::set_target(d, e3, Some(cost_target(hdtg)));
    // e3:SetOperation(s.hdop)
    effect::set_operation(d, e3, Some(operation(hdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(3) and c:IsAttribute(ATTRIBUTE_WATER)
    card::is_level_below(d, c, 3) && card::is_attribute(d, c, ATTRIBUTE_WATER)
}
// function s.hdcon(e,tp,eg,ep,ev,re,r,rp)
fn hdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst()==e:GetHandler():GetEquipTarget() and eg:GetFirst():IsStatus(STATUS_OPPO_BATTLE)
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg);
    let h = effect::get_handler(d, a.e).nil();
    if first != card::get_equip_target(d, h) {
        return false;
    }
    card::is_status(d, first.nil(), STATUS_OPPO_BATTLE)
}
// function s.hdtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn hdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, LOCATION_HAND as i32);
    true
}
// function s.hdop(e,tp,eg,ep,ev,re,r,rp)
fn hdop(d: &mut Duel, a: &EffArgs) {
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
    // Duel.SendtoGrave(sg,REASON_EFFECT)
    duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
}
