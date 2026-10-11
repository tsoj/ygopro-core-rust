//! Valkyrie Brunhilde
//! Scripted by Naim and AlphaKretin
use super::prelude::*;

pub const ID: u32 = 2204038;

// s.listed_series={SET_VALKYRIE}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VALKYRIE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Unaffected by opponent's spells
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(efilter));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains 500 ATK per opponent's monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Lose 1000 DEF, and if you do, your "Valkyrie" monsters cannot be destroyed by battle
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.ptcon)
    effect::set_condition(d, e3, Some(condition(ptcon)));
    // e3:SetTarget(s.pttg)
    effect::set_target(d, e3, Some(cost_target(pttg)));
    // e3:SetOperation(s.ptop)
    effect::set_operation(d, e3, Some(operation(ptop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:IsSpellEffect() and te:GetOwnerPlayer()~=e:GetHandlerPlayer()
    let (e, te) = (a.e(0), a.e(1));
    effect::is_spell_effect(d, te) && effect::get_owner_player(d, te) != effect::get_handler_player(d, e)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetFieldGroupCount(c:GetControler(),0,LOCATION_MZONE)*500
    let p = card::get_controler(d, c);
    duel::get_field_group_count(d, p, 0, LOCATION_MZONE) as i64 * 500
}
/// `s.ptcon(e,tp,eg,ep,ev,re,r,rp)`
fn ptcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    // return at and at:IsControler(1-tp) and at:IsRelateToBattle()
    match duel::get_attacker(d) {
        Some(at) => card::is_controler(d, at, 1 - a.tp) && card::is_relate_to_battle(d, at),
        None => false,
    }
}
/// `s.pttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn pttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetDefense()>=1000 end
    if a.chk == Some(0) {
        return card::get_defense(d, c) >= 1000;
    }
    true
}
/// `s.ptop(e,tp,eg,ep,ev,re,r,rp)`
fn ptop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and c:UpdateDefense(-1000)==-1000 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::update_defense(d, c, -1000, None, None, None) == -1000 {
        // Your "Valkyrie" monsters cannot be destroyed by battle
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3000)
        effect::set_description(d, e1, 3000);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetTarget(function(e,cc) return cc:IsSetCard(SET_VALKYRIE) end)
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, cc| card::is_set_card(d, cc, &[SET_VALKYRIE as u16]))));
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
