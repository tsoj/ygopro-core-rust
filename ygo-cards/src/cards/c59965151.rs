//! Ghost Gardna
use super::prelude::*;

pub const ID: u32 = 59965151;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // change battle target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.cbcon)
    effect::set_condition(d, e1, Some(condition(cbcon)));
    // e1:SetTarget(s.cbtg)
    effect::set_target(d, e1, Some(cost_target(cbtg)));
    // e1:SetOperation(s.cbop)
    effect::set_operation(d, e1, Some(operation(cbop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atkdown
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cbcon(e,tp,eg,ep,ev,re,r,rp)`
fn cbcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bt=eg:GetFirst()
    let bt = group::get_first(d, &a.eg.clone().nil());
    // return c~=bt and bt:IsFaceup() and bt:GetControler()==c:GetControler()
    Some(c) != bt && {
        let bt = bt.nil();
        card::is_faceup(d, bt) && card::get_controler(d, bt) == card::get_controler(d, c)
    }
}
/// `s.cbtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cbtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetAttacker():GetAttackableTarget():IsContains(e:GetHandler()) end
    if a.chk == Some(0) {
        let attacker = duel::get_attacker(d).nil();
        let (g, _direct) = card::get_attackable_target(d, attacker);
        let h = effect::get_handler(d, a.e).nil();
        return group::is_contains(&g, h);
    }
    true
}
/// `s.cbop(e,tp,eg,ep,ev,re,r,rp)`
fn cbop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and not Duel.GetAttacker():IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        let attacker = duel::get_attacker(d).nil();
        if !card::is_immune_to_effect(d, attacker, a.e) {
            // Duel.ChangeAttackTarget(c)
            duel::change_attack_target(d, Some(c), false);
        }
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_DESTROY)
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_DESTROY)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(-1000)
            effect::set_value(d, e1, -1000);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
