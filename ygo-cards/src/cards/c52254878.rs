//! Dimensional Allotrope Varis
use super::prelude::*;

pub const ID: u32 = 52254878;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by battle with a monster with the same Type/Attribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(s.indval)
    effect::set_value_fn(d, e1, effect_card_fn(indval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change its Type and Attribute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.indval(e,c)`
fn indval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local handler=e:GetHandler()
    let handler = effect::get_handler(d, e).nil();
    // return c:IsAttribute(handler:GetAttribute()) or c:IsRace(handler:GetRace())
    let att = card::get_attribute(d, handler);
    if card::is_attribute(d, c, att) {
        true
    } else {
        let race = card::get_race(d, handler);
        card::is_race(d, c, race)
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=Duel.AnnounceRace(tp,1,RACE_ALL)
    let rc = duel::announce_race(d, a.tp, 1, RACE_ALL);
    // local att=c:IsRaceExcept(rc) and Duel.AnnounceAttribute(tp,1,ATTRIBUTE_ALL) or c:AnnounceAnotherAttribute(tp)
    let att = if card::is_race_except(d, c, rc, None, 0, None) {
        duel::announce_attribute(d, a.tp, 1, ATTRIBUTE_ALL)
    } else {
        card::announce_another_attribute(d, c, a.tp)
    };
    // e:SetLabel(rc,att)
    effect::set_label(d, a.e, &[rc as i64, att as i64]);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if (not c:IsFaceup() and c:IsRelateToEffect(e)) then return end
    if !card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local rc,att=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let rc = labels.first().copied().unwrap_or(0);
    let att = labels.get(1).copied().unwrap_or(0);
    // if c:IsRaceExcept(rc) then
    if card::is_race_except(d, c, rc as u64, None, 0, None) {
        // Change monster type
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_RACE)
        effect::set_code(d, e1, EFFECT_CHANGE_RACE);
        // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
        effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
        // e1:SetValue(rc)
        effect::set_value(d, e1, rc);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END,2)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 2);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // if c:IsAttributeExcept(att) then
    if card::is_attribute_except(d, c, att as u32, None, 0, None) {
        // Change Attribute
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e2, EFFECT_CHANGE_ATTRIBUTE);
        // e2:SetProperty(EFFECT_FLAG_COPY_INHERIT)
        effect::set_property(d, e2, EFFECT_FLAG_COPY_INHERIT, 0);
        // e2:SetValue(att)
        effect::set_value(d, e2, att);
        // e2:SetReset(RESETS_STANDARD_DISABLE_PHASE_END,2)
        effect::set_reset(d, e2, RESETS_STANDARD_DISABLE_PHASE_END, 2);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
