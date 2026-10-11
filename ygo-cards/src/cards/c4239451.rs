//! Performapal Turn Toad
use super::prelude::*;

pub const ID: u32 = 4239451;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // swap1
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.adtg1)
    effect::set_target(d, e2, Some(cost_target(adtg1)));
    // e2:SetOperation(s.adop1)
    effect::set_operation(d, e2, Some(operation(adop1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // swap2
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.adcon2)
    effect::set_condition(d, e3, Some(condition(adcon2)));
    // e3:SetTarget(s.adtg2)
    effect::set_target(d, e3, Some(cost_target(adtg2)));
    // e3:SetOperation(s.adop2)
    effect::set_operation(d, e3, Some(operation(adop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsDefenseAbove(0)
    card::is_faceup(d, c) && card::is_defense_above(d, c, 0)
}
/// `s.adtg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn adtg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.adop1(e,tp,eg,ep,ev,re,r,rp)`
fn adop1(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local atk=tc:GetAttack()
        let atk = card::get_attack(d, tc) as i64;
        // local def=tc:GetDefense()
        let def = card::get_defense(d, tc) as i64;
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SWAP_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SWAP_ATTACK_FINAL);
        // e1:SetValue(def)
        effect::set_value(d, e1, def);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SWAP_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SWAP_DEFENSE_FINAL);
        // e2:SetValue(atk)
        effect::set_value(d, e2, atk);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.adcon2(e,tp,eg,ep,ev,re,r,rp)`
fn adcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.IsBattlePhase() and Duel.GetCurrentChain()==0
    duel::is_turn_player(d, a.tp) && duel::is_battle_phase(d, None) && duel::get_current_chain(d, false) == 0
}
/// `s.adtg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn adtg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsDefenseAbove(0) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && card::is_defense_above(d, chkc, 0);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsDefenseAbove,tp,LOCATION_MZONE,0,1,nil,0) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_defense_above(d, c, 0)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,Card.IsDefenseAbove,tp,LOCATION_MZONE,0,1,1,nil,0)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| card::is_defense_above(d, c, 0)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.adop2(e,tp,eg,ep,ev,re,r,rp)`
fn adop2(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e)
    //     and Duel.ChangePosition(tc,POS_FACEUP_DEFENSE,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)~=0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, Some(POS_FACEUP_ATTACK as u8), Some(POS_FACEUP_ATTACK as u8), Some(POS_FACEUP_ATTACK as u8), false) != 0
    {
        // local atk=tc:GetAttack()
        let atk = card::get_attack(d, tc) as i64;
        // local def=tc:GetDefense()
        let def = card::get_defense(d, tc) as i64;
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SWAP_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SWAP_ATTACK_FINAL);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetValue(def)
        effect::set_value(d, e1, def);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SWAP_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SWAP_DEFENSE_FINAL);
        // e2:SetValue(atk)
        effect::set_value(d, e2, atk);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
