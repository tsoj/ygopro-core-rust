//! Procession of the Tea Jar
//! 童妖 茶壺
use super::prelude::*;

pub const ID: u32 = 12612470;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Face-down Defense Position monsters in this card's column cannot change battle positions
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.coltg)
    effect::set_target(d, e1, Some(effect_card_fn(coltg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot activate Set cards in this card's column's Spell & Trap Zones
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetValue(function(e,re) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and s.coltg(e,re:GetHandler()) end)
    effect::set_value_fn(d, e2, lua_fn(|d, a| {
        let e = a.e(0);
        let re = a.e(1);
        effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && {
            let rc = effect::get_handler(d, re).nil();
            let h = effect::get_handler(d, e).nil();
            coltg_card(d, h, rc)
        }
    }));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Move this card to an unused adjacent Main Monster Zone
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(aux.seqmovcon)
    effect::set_condition(d, e3, Some(aux::seqmovcon()));
    // e3:SetOperation(aux.seqmovop)
    effect::set_operation(d, e3, Some(aux::seqmovop()));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Change all other monsters in this card's column to face-down Defense Position
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e4, CATEGORY_POSITION + CATEGORY_SET);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_MOVE)
    effect::set_code(d, e4, EVENT_MOVE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(s.poscon)
    effect::set_condition(d, e4, Some(condition(poscon)));
    // e4:SetTarget(s.postg)
    effect::set_target(d, e4, Some(cost_target(postg)));
    // e4:SetOperation(s.posop)
    effect::set_operation(d, e4, Some(operation(posop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.coltg(e,c)`: `return c:IsFacedown() and e:GetHandler():GetColumnGroup():IsContains(c)`
fn coltg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    coltg_card(d, h, c)
}
/// `s.coltg(e,c)` with the effect's handler `h`
fn coltg_card(d: &mut Duel, h: CardId, c: CardId) -> bool {
    card::is_facedown(d, c) && card::get_column_group(d, h, 0, 0).contains(c)
}
/// `s.poscon(e,tp,eg,ep,ev,re,r,rp)`
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLocation(LOCATION_MZONE) and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_location(d, c, LOCATION_MZONE) && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.posfilter(c)`
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCanTurnSet() and c:IsLocation(LOCATION_MZONE)
    card::is_can_turn_set(d, c) && card::is_location(d, c, LOCATION_MZONE)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=e:GetHandler():GetColumnGroup():Filter(s.posfilter,nil)
    let h = effect::get_handler(d, a.e).nil();
    let cg = card::get_column_group(d, h, 0, 0);
    let g = group::filter(d, &cg, &posfilter, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,tp,POS_FACEDOWN_DEFENSE)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=c:GetColumnGroup():Filter(s.posfilter,nil)
    let cg = card::get_column_group(d, c, 0, 0);
    let g = group::filter(d, &cg, &posfilter, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
