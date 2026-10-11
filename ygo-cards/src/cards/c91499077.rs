//! Gagaga Samurai
use super::prelude::*;

pub const ID: u32 = 91499077;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GAGAGA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz summon procedure
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Targeted "Gagaga" monster can make a second attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atcon)
    effect::set_condition(d, e1, Some(condition(atcon)));
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.attg)
    effect::set_target(d, e1, Some(cost_target(attg)));
    // e1:SetOperation(s.atop)
    effect::set_operation(d, e1, Some(operation(atop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When your monster is targeted for attack, change the attack target to this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.cbcon)
    effect::set_condition(d, e2, Some(condition(cbcon)));
    // e2:SetOperation(s.cbop)
    effect::set_operation(d, e2, Some(operation(cbop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atcon(e,tp,eg,ep,ev,re,r,rp)`
fn atcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsAbleToEnterBP()
    duel::is_able_to_enter_bp(d)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GAGAGA) and c:GetEffectCount(EFFECT_EXTRA_ATTACK)==0
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GAGAGA as u16]) && card::get_effect_count(d, c, EFFECT_EXTRA_ATTACK) == 0
}
/// `s.attg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn attg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3201)
        effect::set_description(d, e1, 3201);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.cbcon(e,tp,eg,ep,ev,re,r,rp)`
fn cbcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return r~=REASON_REPLACE and bt~=e:GetHandler() and bt:IsControler(tp) and e:GetHandler():IsPosition(POS_ATTACK)
    if a.r == REASON_REPLACE {
        return false;
    }
    // local bt=eg:GetFirst()
    let bt = group::get_first(d, &a.eg.clone().nil());
    let h = effect::get_handler(d, a.e).nil();
    if bt == Some(h) {
        return false;
    }
    let bt = bt.nil();
    card::is_controler(d, bt, a.tp) && card::is_position(d, h, POS_ATTACK)
}
/// `s.cbop(e,tp,eg,ep,ev,re,r,rp)`
fn cbop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) and Duel.ChangePosition(c,POS_FACEUP_DEFENSE)~=0 then
    if card::is_faceup(d, c)
        && card::is_relate_to_effect(d, c, a.e)
        && duel::change_position(d, CardSet::from_card(c), POS_FACEUP_DEFENSE as u8, None, None, None, false) != 0
    {
        // local at=Duel.GetAttacker()
        let at = duel::get_attacker(d).nil();
        // if at:CanAttack() and not at:IsImmuneToEffect(e) and not c:IsImmuneToEffect(e) then
        if card::can_attack(d, at) && !card::is_immune_to_effect(d, at, a.e) && !card::is_immune_to_effect(d, c, a.e) {
            // Duel.CalculateDamage(at,c)
            duel::calculate_damage(d, at, Some(c), false);
        }
    }
}
