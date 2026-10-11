//! EMオッドアイズ・ユニコーン
//! Performapal Odd-Eyes Unicorn
use super::prelude::*;

pub const ID: u32 = 86157908;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAPAL, SET_ODD_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // atk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetProperty(EFFECT_FLAG_NO_TURN_RESET+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_NO_TURN_RESET + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // recover
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e3, CATEGORY_RECOVER);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_PERFORMAPAL)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PERFORMAPAL as u16])
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // return at:IsControler(tp) and at:IsSetCard(SET_ODD_EYES)
    card::is_controler(d, at, a.tp) && card::is_set_card(d, at, &[SET_ODD_EYES as u16])
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.atkfilter(chkc) and chkc~=at end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && atkfilter(d, chkc) && Some(chkc) != at;
    }
    // Lua's `at` may be nil: the exclusion is then "nothing"
    let ex = at.map_or(NO_EX, Into::into);
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,0,1,at) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, ex);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,0,1,1,at)
    let _ = duel::select_target(d, a.tp, Some(&atkfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, ex);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // if at:IsFaceup() and at:IsRelateToBattle() and at:CanAttack() and not at:IsStatus(STATUS_ATTACK_CANCELED)
    //     and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, at)
        && card::is_relate_to_battle(d, at)
        && card::can_attack(d, at)
        && !card::is_status(d, at, STATUS_ATTACK_CANCELED)
        && card::is_faceup(d, tc.nil())
        && card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        // local atk=tc:GetBaseAttack()
        let atk = card::get_base_attack(d, tc.nil());
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
        // at:RegisterEffect(e1)
        card::register_effect(d, at, e1, false);
    }
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PERFORMAPAL) and c:GetAttack()>0
    card::is_set_card(d, c, &[SET_PERFORMAPAL as u16]) && card::get_attack(d, c) > 0
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local atk=g:GetFirst():GetAttack()
    let atk = card::get_attack(d, group::get_first(d, &g).nil());
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,atk)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, atk);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Recover(tp,tc:GetAttack(),REASON_EFFECT)
        let atk = card::get_attack(d, tc);
        duel::recover(d, a.tp, atk as i64, REASON_EFFECT, false, None);
    }
}
