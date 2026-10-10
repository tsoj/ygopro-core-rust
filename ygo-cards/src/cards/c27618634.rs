//! The Unhappy Girl
use super::prelude::*;

pub const ID: u32 = 27618634;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle indestructable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetCondition(s.indcon)
    effect::set_condition(d, e1, Some(condition(indcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // battled
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetOperation(s.atop)
    effect::set_operation(d, e2, Some(operation(atop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e2:SetLabelObject(g)
    // (the shared group lives in the label object of e2; e3 and e4 reach it through e2)
    effect::set_label_object(d, e2, Value::Group(CardSet::new()));
    // atk limit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCondition(s.atlcon)
    effect::set_condition(d, e3, Some(condition(atlcon)));
    // e3:SetTarget(s.atltg)
    effect::set_target(d, e3, Some(effect_card_fn(atltg)));
    // e3:SetLabelObject(g)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
    effect::set_code(d, e4, EFFECT_CANNOT_CHANGE_POSITION);
    // e4:SetLabelObject(g)
    effect::set_label_object(d, e4, Value::Effect(e2));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_CHANGE_POS)
    effect::set_code(d, e5, EVENT_CHANGE_POS);
    // e5:SetOperation(s.posop)
    effect::set_operation(d, e5, Some(operation(posop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPosition(POS_FACEUP_ATTACK)
    let h = effect::get_handler(d, a.e).nil();
    card::is_position(d, h, POS_FACEUP_ATTACK as u32)
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // if bc and c:IsPosition(POS_FACEUP_ATTACK) then
    if let Some(bc) = bc {
        if card::is_position(d, c, POS_FACEUP_ATTACK as u32) {
            // if c:GetFlagEffect(id)==0 then
            if card::get_flag_effect(d, c, ID) == 0 {
                // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
                card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
                // e:GetLabelObject():Clear()
                let mut g = effect::get_label_object(d, a.e).as_group().cloned().nil();
                g.clear();
                effect::set_label_object(d, a.e, Value::Group(g));
            }
            // e:GetLabelObject():AddCard(bc)
            let mut g = effect::get_label_object(d, a.e).as_group().cloned().nil();
            group::add_card(&mut g, &CardSet::from_card(bc));
            effect::set_label_object(d, a.e, Value::Group(g));
            // bc:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, bc, ID + 1, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        }
    }
}
/// `s.atlcon(e)`
fn atlcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
/// `s.atltg(e,c)`
fn atltg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetLabelObject():IsContains(c) and c:GetFlagEffect(id+1)~=0
    let g_src = effect::get_label_object(d, e).as_effect().nil();
    let g = effect::get_label_object(d, g_src).as_group().cloned().nil();
    group::is_contains(&g, c) && card::get_flag_effect(d, c, ID + 1) != 0
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():ResetFlagEffect(id)
    let h = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, h, ID);
}
