//! Dark Zebra
use super::prelude::*;

pub const ID: u32 = 59784896;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to defense
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer() and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==1
    a.tp == duel::get_turn_player(d) && duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 1
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_POSITION, h, 1, 0, 0);
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsPosition(POS_FACEUP_ATTACK) and Duel.ChangePosition(c,POS_FACEUP_DEFENSE)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_position(d, c, POS_FACEUP_ATTACK) && duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false) != 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
        effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_COPY_INHERIT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_COPY_INHERIT, 0);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
