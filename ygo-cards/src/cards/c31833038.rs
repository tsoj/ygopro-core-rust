//! Borreload Dragon
use super::prelude::*;

pub const ID: u32 = 31833038;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),3)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 3, None, None, None, None);
    // Cannot be targeted by monster effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,re,rp) return re:IsMonsterEffect() end)
    effect::set_value_fn(d, e1, lua_fn(value1));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Target monster loses 500 ATK/DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP|TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP | TIMING_END_PHASE, None);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e2, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Take control of an opponent's monster this card attacks
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e3, CATEGORY_CONTROL);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e3, EVENT_BATTLE_START);
    // e3:SetCondition(s.ctcon)
    effect::set_condition(d, e3, Some(condition(ctcon)));
    // e3:SetTarget(s.cttg)
    effect::set_target(d, e3, Some(cost_target(cttg)));
    // e3:SetOperation(s.ctop)
    effect::set_operation(d, e3, Some(operation(ctop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,re,rp) return re:IsMonsterEffect() end`
fn value1(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    effect::is_monster_effect(d, re)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetChainLimit(function(_e,_ep,_tp) return _tp==_ep end)
    duel::set_chain_limit(d, lua_fn(|_d, a: Args| a.p(2) == a.p(1)));
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // --It loses 500 ATK/DEF
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(-500)
        effect::set_value(d, e1, -500);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d);
    // local zone=c:GetLinkedZone()&ZONES_MMZ
    let zone = card::get_linked_zone(d, c, None) & ZONES_MMZ;
    // return Duel.GetAttacker()==c and tc and tc:IsControler(1-tp) and tc:IsControlerCanBeChanged(false,zone)
    if duel::get_attacker(d) != Some(c) {
        return false;
    }
    match tc {
        Some(tc) => card::is_controler(d, tc, 1 - a.tp) && card::is_controler_can_be_changed(d, tc, false, Some(zone)),
        None => false,
    }
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local tc=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d);
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc.nil());
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, tc.map(CardSet::from_card), 1, a.tp, 0);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not (c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and tc:IsControler(1-tp)) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tc = tc.nil();
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_controler(d, tc, 1 - a.tp)) {
        return;
    }
    // local zone=c:GetLinkedZone()&ZONES_MMZ
    let zone = card::get_linked_zone(d, c, None) & ZONES_MMZ;
    // if Duel.GetControl(tc,tp,0,0,zone) then
    if duel::get_control(d, tc, a.tp, 0, 0, Some(zone), None) != 0 {
        // local turn_ct=Duel.GetTurnCount()
        let turn_ct = duel::get_turn_count(d, None);
        // --Send it to the GY during the End Phase of the next turn
        // aux.DelayedOperation(tc,PHASE_END,id,e,tp,
        //     function(ag) Duel.SendtoGrave(ag,REASON_EFFECT) end,
        //     function() return Duel.GetTurnCount()==turn_ct+1 end,
        //     nil,2)
        aux::delayed_operation(
            d,
            tc,
            PHASE_END,
            ID,
            a.e,
            a.tp,
            Some(std::sync::Arc::new(send_oper)),
            Some(std::sync::Arc::new(move |d: &mut Duel, _ag: &CardSet, _a: &EffArgs| duel::get_turn_count(d, None) == turn_ct + 1)),
            None,
            Some(2),
            None,
            None,
        );
    }
}
/// `function(ag) Duel.SendtoGrave(ag,REASON_EFFECT) end`
fn send_oper(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::send_to_grave(d, ag.clone(), REASON_EFFECT, None, None);
    true
}
