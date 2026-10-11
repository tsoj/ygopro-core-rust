//! Satellarknight Rigel
use super::prelude::*;

pub const ID: u32 = 13851202;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TELLARKNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make 1 "tellarknight" monster on the field gain 500 ATK, but send it to the GY during the End Phase
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetTarget(s.atktg)
    effect::set_target(d, e1a, Some(cost_target(atktg)));
    // e1a:SetOperation(s.atkop)
    effect::set_operation(d, e1a, Some(operation(atkop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e1c=e1a:Clone()
    let e1c = effect::clone(d, e1a);
    // e1c:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1c, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1c)
    card::register_effect(d, c, e1c, false);
}
fn tfilter(d: &mut Duel, c: CardId) -> bool {
    // aux.FaceupFilter(Card.IsSetCard,SET_TELLARKNIGHT)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16])
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsSetCard(SET_TELLARKNIGHT) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_set_card(d, chkc, &[SET_TELLARKNIGHT as u16]) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSetCard,SET_TELLARKNIGHT),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsSetCard,SET_TELLARKNIGHT),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, tp, Some(&tfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // if tc:IsFaceup() then
        if card::is_faceup(d, tc) {
            // --It gains 500 ATK
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(500)
            effect::set_value(d, e1, 500);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
        // --Send it to the GY during the End Phase
        // aux.DelayedOperation(tc,PHASE_END,id,e,tp,function(dg) Duel.SendtoGrave(dg,REASON_EFFECT) end,nil,0,1,aux.Stringid(id,1),aux.Stringid(id,2))
        aux::delayed_operation(
            d,
            tc,
            PHASE_END,
            ID,
            a.e,
            tp,
            Some(std::sync::Arc::new(grave_oper)),
            None,
            Some(0),
            Some(1),
            Some(aux::stringid(ID, 1)),
            Some(aux::stringid(ID, 2)),
        );
    }
}
fn grave_oper(d: &mut Duel, dg: &CardSet, _a: &EffArgs) -> bool {
    // Duel.SendtoGrave(dg,REASON_EFFECT)
    duel::send_to_grave(d, dg.clone(), REASON_EFFECT, None, None);
    true
}
