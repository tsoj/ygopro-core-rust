//! Yubel - The Ultimate Nightmare
use super::prelude::*;

pub const ID: u32 = 31764700;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_BATTLED)
    effect::set_code(d, e3, EVENT_BATTLED);
    // e3:SetOperation(s.batop)
    effect::set_operation(d, e3, Some(operation(batop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DAMAGE+CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DAMAGE + CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e4, EVENT_DAMAGE_STEP_END);
    // e4:SetTarget(s.damtg)
    effect::set_target(d, e4, Some(cost_target(damtg)));
    // e4:SetOperation(s.damop)
    effect::set_operation(d, e4, Some(operation(damop)));
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // cannot special summon
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e5, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.batop(e,tp,eg,ep,ev,re,r,rp)`
fn batop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    match bc {
        // if bc and c:IsAttackPos() then
        Some(bc) if card::is_attack_pos(d, c) => {
            // e:SetLabel(bc:GetAttack())
            let atk = card::get_attack(d, bc);
            effect::set_label(d, a.e, &[atk as i64]);
            // e:SetLabelObject(bc)
            effect::set_label_object(d, a.e, Value::Card(bc));
        }
        // else e:SetLabelObject(nil) end
        _ => effect::set_label_object(d, a.e, Value::Nil),
    }
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetLabelObject():GetLabelObject()
    let e3 = effect::get_label_object(d, a.e).as_effect().nil();
    let bc = effect::get_label_object(d, e3).as_card();
    // if chk==0 then return bc end
    if a.chk == Some(0) {
        return bc.is_some();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,e:GetLabelObject():GetLabel())
    let dam = effect::get_label(d, e3);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    // if bc:IsRelateToBattle() then
    let bc = bc.nil();
    if card::is_relate_to_battle(d, bc) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(bc)), 1, 0, 0);
    }
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(1-tp,e:GetLabelObject():GetLabel(),REASON_EFFECT)
    let e3 = effect::get_label_object(d, a.e).as_effect().nil();
    let dam = effect::get_label(d, e3);
    duel::damage(d, 1 - a.tp, dam, REASON_EFFECT, false, None);
    // local bc=e:GetLabelObject():GetLabelObject()
    let bc = effect::get_label_object(d, e3).as_card().nil();
    // if bc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, bc) {
        // Duel.Destroy(bc,REASON_EFFECT)
        duel::destroy(d, bc, REASON_EFFECT, None, None);
    }
}
