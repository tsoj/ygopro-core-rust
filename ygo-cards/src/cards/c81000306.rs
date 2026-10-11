//! Full-Force Strike
use super::prelude::*;

pub const ID: u32 = 81000306;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:HasFlagEffect(id) and c:IsFaceup()
    !card::has_flag_effect(d, c, ID, None) && card::is_faceup(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_APPLYTO as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:HasFlagEffect(id) then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::has_flag_effect(d, tc, ID, None) {
        return;
    }
    // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
    card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // Players take no battle damage from its attacks
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    // e2:SetCondition(function(e) return e:GetHandler()==Duel.GetAttacker() end)
    effect::set_condition(d, e2, Some(condition(avoid_cond)));
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_NO_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_NO_BATTLE_DAMAGE);
    // tc:RegisterEffect(e3)
    card::register_effect(d, tc, e3, false);
    // Destroy an opponent's monster it attacks after damage calculation
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_BATTLED)
    effect::set_code(d, e4, EVENT_BATTLED);
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // e4:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e4, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e4)
    card::register_effect(d, tc, e4, false);
}
/// `function(e) return e:GetHandler()==Duel.GetAttacker() end`
fn avoid_cond(d: &mut Duel, a: &EffArgs) -> bool {
    effect::get_handler(d, a.e) == duel::get_attacker(d)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // local tc=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d);
    // if c==Duel.GetAttacker() and tc and tc:IsRelateToBattle() then
    if c == duel::get_attacker(d) {
        if let Some(tc) = tc {
            if card::is_relate_to_battle(d, tc) {
                // Duel.Hint(HINT_CARD,tp,id)
                duel::hint(d, HINT_CARD as u8, tp, ID as u64);
                // Duel.Hint(HINT_CARD,1-tp,id)
                duel::hint(d, HINT_CARD as u8, 1 - tp, ID as u64);
                // Duel.Destroy(tc,REASON_EFFECT)
                duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
            }
        }
    }
}
