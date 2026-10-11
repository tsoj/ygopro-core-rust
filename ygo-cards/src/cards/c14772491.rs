//! Common Soul
use super::prelude::*;

pub const ID: u32 = 14772491;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEO_SPACIAN as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // tohand
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // e1:SetLabelObject(e2)
    effect::set_label_object(d, e1, Value::Effect(e2));
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NEO_SPACIAN) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.filter(c)`: `c:IsFaceup() and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && {
        let cp = card::get_controler(d, c);
        duel::get_location_count(d, cp, LOCATION_MZONE as u8, None, None, None).0 > 0
    }
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    // local cp=tc:GetControler()
    let tc = duel::first_target(d).nil();
    let cp = card::get_controler(d, tc);
    // if c:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // if Duel.GetLocationCount(cp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, cp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // local sc=g:GetFirst()
        let sc = group::get_first(d, &g).nil();
        // Duel.SpecialSummon(sc,0,tp,cp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, tp, cp, false, false, POS_FACEUP as u8, None);
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc);
        // c:SetCardTarget(sc)
        card::set_card_target(d, c, sc);
        // e:GetLabelObject():SetLabelObject(sc)
        let e2 = effect::get_label_object(d, a.e).as_effect().nil();
        effect::set_label_object(d, e2, Value::Card(sc));
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_OWNER_RELATE)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_OWNER_RELATE, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetCondition(s.rcon)
        effect::set_condition(d, e1, Some(condition(rcon)));
        // e1:SetValue(sc:GetAttack())
        let atk = card::get_attack(d, sc);
        effect::set_value(d, e1, atk as i64);
        // e1:SetLabelObject(sc)
        effect::set_label_object(d, e1, Value::Card(sc));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
    }
}
/// `s.rcon(e)`: `e:GetOwner():IsHasCardTarget(e:GetHandler()) and e:GetOwner():IsHasCardTarget(e:GetLabelObject())`
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    let owner = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    card::is_has_card_target(d, owner, h) && card::is_has_card_target(d, owner, lo)
}
/// `s.thcon(e,tp,...)`: `local tc=e:GetLabelObject(); return tc and tc:IsOnField() and e:GetHandler():IsHasCardTarget(tc)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tc = match effect::get_label_object(d, a.e).as_card() {
        Some(tc) => tc,
        None => return false,
    };
    let h = effect::get_handler(d, a.e).nil();
    card::is_on_field(d, tc) && card::is_has_card_target(d, h, tc)
}
/// `s.thop(e,tp,...)`: `Duel.SendtoHand(e:GetLabelObject(),nil,REASON_EFFECT)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
}
