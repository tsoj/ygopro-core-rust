//! Moulinglacia the Elemental Lord
use super::prelude::*;

pub const ID: u32 = 13959634;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // handes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e3, CATEGORY_HANDES);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.hdtg)
    effect::set_target(d, e3, Some(cost_target(hdtg)));
    // e3:SetOperation(s.hdop)
    effect::set_operation(d, e3, Some(operation(hdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // leave
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD_P);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    // e4:SetOperation(s.leaveop)
    effect::set_operation(d, e4, Some(operation(leaveop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0 and
    //     Duel.GetMatchingGroupCount(Card.IsAttribute,c:GetControler(),LOCATION_GRAVE,0,nil,ATTRIBUTE_WATER)==5
    let p = card::get_controler(d, c);
    duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_matching_group_count(d, Some(&|d, x| card::is_attribute(d, x, ATTRIBUTE_WATER)), p, LOCATION_GRAVE, 0, NO_EX) == 5
}
/// `s.hdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,2)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 2);
    true
}
/// `s.hdop(e,tp,eg,ep,ev,re,r,rp)`
fn hdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND):RandomSelect(tp,2)
    let field = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    let g = group::random_select(d, &field, a.tp, 2);
    // Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
}
/// `s.leaveop(e,tp,eg,ep,ev,re,r,rp)`
fn leaveop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsFacedown() then return end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_facedown(d, h) {
        return;
    }
    // local effp=e:GetHandler():GetControler()
    let effp = card::get_controler(d, h);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SKIP_BP)
    effect::set_code(d, e1, EFFECT_SKIP_BP);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // if Duel.GetTurnPlayer()==effp then
    if duel::get_turn_player(d) == effp {
        // e1:SetLabel(Duel.GetTurnCount())
        let tc = duel::get_turn_count(d, None);
        effect::set_label(d, e1, &[tc as i64]);
        // e1:SetCondition(s.skipcon)
        effect::set_condition(d, e1, Some(lua_fn(skipcon)));
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,1)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 1);
    }
    // Duel.RegisterEffect(e1,effp)
    duel::register_effect(d, e1, effp);
}
/// `s.skipcon(e)`
fn skipcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetTurnCount()~=e:GetLabel()
    let e = a.e(0);
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, e)
}
