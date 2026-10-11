//! Assault Blackwing - Kusanagi the Gathering Storm
use super::prelude::*;

pub const ID: u32 = 31114334;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACKWING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Material check
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetValue(function(e,c) e:SetLabel(c:GetMaterial():IsExists(Card.IsSetCard,1,nil,SET_BLACKWING) and 1 or 0) end)
    effect::set_value_fn(d, e0, lua_fn(material_check));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_synchro_summoned(d, h)
        })),
    );
    // e1:SetLabelObject(e0)
    effect::set_label_object(d, e1, Value::Effect(e0));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can attack all your opponent's monsters, once each
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_ATTACK_ALL)
    effect::set_code(d, e2, EFFECT_ATTACK_ALL);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    // e3:SetCode(EFFECT_PIERCE)
    // c:RegisterEffect(e3)
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_PIERCE);
    card::register_effect(d, c, e3, false);
}
/// `function(e,c) e:SetLabel(c:GetMaterial():IsExists(Card.IsSetCard,1,nil,SET_BLACKWING) and 1 or 0) end` (the MATERIAL_CHECK value)
fn material_check(d: &mut Duel, a: Args) -> Value {
    let (e, c) = (a.e(0), a.c(1));
    let mat = card::get_material(d, c);
    let v = group::is_exists(d, &mat, &|d, x| card::is_set_card(d, x, &[SET_BLACKWING as u16]), 1, NO_EX);
    effect::set_label(d, e, &[if v { 1 } else { 0 }]);
    Value::Nil
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local mg=c:GetMaterial()
    let mg = card::get_material(d, c);
    // if #mg==0 then return end
    if mg.is_empty() {
        return;
    }
    // if e:GetLabelObject():GetLabel()==1 then
    let e0 = effect::get_label_object(d, e).as_effect().nil();
    if effect::get_label(d, e0) == 1 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e1, EFFECT_ADD_TYPE);
        // e1:SetValue(TYPE_TUNER)
        effect::set_value(d, e1, TYPE_TUNER as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // if c:IsDisabled() then return end
    if card::is_disabled(d, c) {
        return;
    }
    // local atk=mg:Match(Card.IsSynchroMonster,nil):GetSum(Card.GetBaseAttack,nil)
    let mut synchro_mats = mg;
    group::match_(d, &mut synchro_mats, &|d, x| card::is_synchro_monster(d, x), NO_EX);
    let atk = group::get_sum(d, &synchro_mats, &|d, x| card::get_base_attack(d, x) as i64);
    // if atk==0 then return end
    if atk == 0 {
        return;
    }
    // c:UpdateAttack(atk,RESETS_STANDARD_DISABLE_PHASE_END)
    card::update_attack(d, c, atk, Some(RESETS_STANDARD_DISABLE_PHASE_END), None, None);
}
