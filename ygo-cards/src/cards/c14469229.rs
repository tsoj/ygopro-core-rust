//! Crystal Keeper
use super::prelude::*;

pub const ID: u32 = 14469229;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CRYSTAL_BEAST, SET_ULTIMATE_CRYSTAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // avoid destruction
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(s.indtg)
    effect::set_target(d, e2, Some(effect_card_fn(indtg)));
    // e2:SetValue(s.indval)
    effect::set_value_fn(d, e2, lua_fn(indval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // double
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e3, EVENT_PRE_DAMAGE_CALCULATE);
    // e3:SetRange(LOCATION_MZONE|LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_MZONE | LOCATION_HAND);
    // e3:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e3, Some(cost::self_tribute()));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.indtg(e,c)`
fn indtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) or (c:IsLocation(LOCATION_MZONE) and c:IsSetCard(SET_ULTIMATE_CRYSTAL))
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        || (card::is_location(d, c, LOCATION_MZONE) && card::is_set_card(d, c, &[SET_ULTIMATE_CRYSTAL as u16]))
}
/// `s.indval(e,re,r,rp)`
fn indval(_d: &mut Duel, a: Args) -> i64 {
    // if (r&REASON_EFFECT)~=0 then
    if (a.int(2) as u32 & REASON_EFFECT) != 0 {
        // return 1
        1
    } else {
        // else return 0 end
        0
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let mut atk = Some(duel::get_attacker(d).nil());
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // if a:IsControler(1-tp) then a=d end
    if card::is_controler(d, atk.nil(), 1 - a.tp) {
        atk = dt;
    }
    // if chk==0 then return d and a:IsSetCard(SET_CRYSTAL_BEAST) end
    if a.chk == Some(0) {
        return dt.is_some() && card::is_set_card(d, atk.nil(), &[SET_CRYSTAL_BEAST as u16]);
    }
    // e:SetLabelObject(a)
    effect::set_label_object(d, a.e, atk.map_or(Value::Nil, Value::Card));
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:IsRelateToBattle() and tc:IsFaceup() then
    if card::is_relate_to_battle(d, tc) && card::is_faceup(d, tc) {
        // local atk=tc:GetBaseAttack()
        let atk = card::get_base_attack(d, tc);
        // local def=tc:GetBaseDefense()
        let def = card::get_base_defense(d, tc);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(atk*2)
        effect::set_value(d, e1, (atk * 2) as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // e2:SetValue(def*2)
        effect::set_value(d, e2, (def * 2) as i64);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local e3=Effect.CreateEffect(e:GetHandler())
        let e3 = effect::create_effect(d, h);
        // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e3:SetCode(EVENT_DAMAGE_STEP_END)
        effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
        // e3:SetOperation(s.desop)
        effect::set_operation(d, e3, Some(operation(desop)));
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToBattle() and c:IsFaceup() then
    if card::is_relate_to_battle(d, c) && card::is_faceup(d, c) {
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
