//! Castle of Dark Illusions
use super::prelude::*;

pub const ID: u32 = 62121;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetFieldID()<=e:GetLabel() and c:IsRace(RACE_ZOMBIE)
    (card::get_field_id(d, c) as i64) <= effect::get_label(d, e) && card::is_race(d, c, RACE_ZOMBIE)
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,LOCATION_MZONE)
        let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, LOCATION_MZONE);
        // local mg,fid=g:GetMaxGroup(Card.GetFieldID)
        let (_mg, fid) = group::get_max_group(d, &g, &|d, c| card::get_field_id(d, c) as i64).nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
        effect::set_target(d, e1, Some(effect_card_fn(atktg)));
        effect::set_value(d, e1, 200);
        effect::set_label(d, e1, &[fid]);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        effect::set_label_object(d, e2, Value::Effect(e1));
        card::register_effect(d, c, e2, false);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        effect::set_description(d, e3, aux::stringid(ID, 1));
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
        effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
        effect::set_range(d, e3, LOCATION_MZONE);
        effect::set_count_limit(d, e3, 1, 0, 0, 0);
        effect::set_condition(d, e3, Some(condition(atkcon)));
        effect::set_operation(d, e3, Some(operation(atkop)));
        effect::set_label_object(d, e3, Value::Effect(e2));
        effect::set_label(d, e3, &[2]);
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 4);
        card::register_effect(d, c, e3, false);
    }
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // e:GetLabelObject():SetValue(ct*200)
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_value(d, e2, ct * 200);
    // e:GetLabelObject():GetLabelObject():SetValue(ct*200)
    let e1 = effect::get_label_object(d, e2).as_effect().nil();
    effect::set_value(d, e1, ct * 200);
    // e:SetLabel(ct+1)
    effect::set_label(d, a.e, &[ct + 1]);
}
