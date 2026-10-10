//! Helpoemer
use super::prelude::*;

pub const ID: u32 = 76052811;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // reg
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // handes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e3, CATEGORY_HANDES);
    // e3:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.hdcon)
    effect::set_condition(d, e3, Some(condition(hdcon)));
    // e3:SetTarget(s.hdtg)
    effect::set_target(d, e3, Some(cost_target(hdtg)));
    // e3:SetOperation(s.hdop)
    effect::set_operation(d, e3, Some(operation(hdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.regop(e,tp,...)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsLocation(LOCATION_GRAVE) and c:IsReason(REASON_BATTLE) then
    if card::is_location(d, c, LOCATION_GRAVE) && card::is_reason(d, c, REASON_BATTLE) {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
    }
}
/// `s.hdcon(e,tp,...)`
fn hdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and e:GetHandler():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, 1 - a.tp) && card::get_flag_effect(d, h, ID) != 0
}
/// `s.hdtg(e,tp,...,chk)`
fn hdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:GetHandler():ResetFlagEffect(id)
    let h = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, h, ID);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,0,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.hdop(e,tp,...)`
fn hdop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
        // local g=Duel.GetFieldGroup(1-tp,LOCATION_HAND,0)
        let g = duel::get_field_group(d, 1 - a.tp, LOCATION_HAND, 0);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // local sg=g:RandomSelect(1-tp,1)
        let sg = group::random_select(d, &g, 1 - a.tp, 1);
        // Duel.SendtoGrave(sg,REASON_DISCARD|REASON_EFFECT)
        duel::send_to_grave(d, sg, REASON_DISCARD | REASON_EFFECT, None, None);
    }
}
