//! Bujingi Crane
use super::prelude::*;

pub const ID: u32 = 68601507;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BUJIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Double ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=Duel.GetAttackTarget()
    // if not c then return false end
    let Some(target) = duel::get_attack_target(d) else {
        return false;
    };
    // if c:IsControler(1-tp) then c=Duel.GetAttacker() end
    let c = if card::is_controler(d, target, 1 - a.tp) { duel::get_attacker(d) } else { Some(target) };
    // e:SetLabelObject(c)  (c may be nil)
    effect::set_label_object(d, a.e, c.map_or(Value::Nil, Value::Card));
    // return c and c:IsSetCard(SET_BUJIN) and c:IsRace(RACE_BEASTWARRIOR) and c:IsRelateToBattle() and not c:IsAttack(c:GetBaseAttack()*2)
    let Some(c) = c else {
        return false;
    };
    card::is_set_card(d, c, &[SET_BUJIN as u16])
        && card::is_race(d, c, RACE_BEASTWARRIOR)
        && card::is_relate_to_battle(d, c)
        && {
            let base = card::get_base_attack(d, c);
            !card::is_attack(d, c, &[(base * 2) as u32])
        }
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetLabelObject()
    let c = effect::get_label_object(d, a.e).as_card();
    // if c:IsFaceup() and c:IsRelateToBattle() then
    if card::is_faceup(d, c.nil()) && card::is_relate_to_battle(d, c.nil()) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // e1:SetValue(c:GetBaseAttack()*2)
        let base = card::get_base_attack(d, c.nil());
        effect::set_value(d, e1, (base * 2) as i64);
        // c:RegisterEffect(e1)
        card::register_effect(d, c.nil(), e1, false);
    }
}
