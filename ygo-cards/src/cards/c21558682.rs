//! Jam Defender
use super::prelude::*;

pub const ID: u32 = 21558682;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[31709826], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetHintTiming(0,TIMING_BATTLE_START)
    effect::set_hint_timing(d, e0, 0, Some(TIMING_BATTLE_START));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // When an opponent's monster declares an attack on a monster you control: ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetTarget(s.atktg2)
    effect::set_target(d, e1, Some(cost_target(atktg2)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkcon(e,tp,...)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()~=nil
    let attacker = duel::get_attacker(d).nil();
    card::is_controler(d, attacker, 1 - a.tp) && duel::get_attack_target(d).is_some()
}
/// `s.filter(c,atg)`
fn filter(d: &mut Duel, c: CardId, atg: &CardSet) -> bool {
    // return c:IsFaceup() and c:IsCode(31709826) and atg:IsContains(c)
    card::is_faceup(d, c) && card::is_code(d, c, &[31709826]) && group::is_contains(atg, c)
}
/// `s.atktg2(e,tp,...,chk,chkc)`
fn atktg2(d: &mut Duel, a: &EffArgs) -> bool {
    // local atg=Duel.GetAttacker():GetAttackableTarget()
    let attacker = duel::get_attacker(d).nil();
    let atg = card::get_attackable_target(d, attacker).0;
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc~=at and s.filter(chkc,atg) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && Some(chkc) != at && filter(d, chkc, &atg);
    }
    let ex = at.map_or(NO_EX, |x| x.into());
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,at,atg) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, &atg)), a.tp, LOCATION_MZONE, 0, 1, ex);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,at,atg)
    duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, &atg)), a.tp, LOCATION_MZONE, 0, 1, 1, false, ex);
    true
}
/// `s.atkop(e,tp,...)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsCode(31709826) and tc:IsRelateToEffect(e) and not Duel.GetAttacker():IsImmuneToEffect(e) then
    if card::is_faceup(d, tc)
        && card::is_code(d, tc, &[31709826])
        && card::is_relate_to_effect(d, tc, a.e)
        && !{
            let attacker = duel::get_attacker(d).nil();
            card::is_immune_to_effect(d, attacker, a.e)
        }
    {
        // Duel.ChangeAttackTarget(tc)
        duel::change_attack_target(d, Some(tc), false);
    }
}
