//! Morpho Butterspy
//! 幻蝶の刺客モルフォ
use super::prelude::*;

pub const ID: u32 = 43573231;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk,def
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e1:SetCode(EVENT_CHANGE_POS)
    effect::set_code(d, e1, EVENT_CHANGE_POS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.adtg)
    effect::set_target(d, e1, Some(cost_target(adtg)));
    // e1:SetOperation(s.adop)
    effect::set_operation(d, e1, Some(operation(adop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local np=c:GetPosition()
    let np = card::get_position(d, c);
    // local pp=c:GetPreviousPosition()
    let pp = card::get_previous_position(d, c);
    // return c:IsControler(tp) and c:IsCanBeEffectTarget(e)
    //     and ((pp==POS_FACEUP_ATTACK and np==POS_FACEUP_DEFENSE) or (pp==POS_FACEUP_DEFENSE and np==POS_FACEUP_ATTACK) or (pp==POS_FACEDOWN_DEFENSE and np==POS_FACEUP_ATTACK))
    card::is_controler(d, c, tp)
        && card::is_can_be_effect_target(d, c, Some(e))
        && ((pp == POS_FACEUP_ATTACK && np == POS_FACEUP_DEFENSE)
            || (pp == POS_FACEUP_DEFENSE && np == POS_FACEUP_ATTACK)
            || (pp == POS_FACEDOWN_DEFENSE && np == POS_FACEUP_ATTACK))
}
/// `s.adtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn adtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return eg:IsContains(chkc) and s.cfilter(chkc,e,1-tp) end
    if let Some(chkc) = a.chkc() {
        let eg = a.eg.clone().nil();
        return group::is_contains(&eg, chkc) && cfilter(d, chkc, a.e, 1 - a.tp);
    }
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(s.cfilter,1,nil,e,1-tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.e, 1 - a.tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=eg:FilterSelect(tp,s.cfilter,1,1,nil,e,1-tp)
    let g = group::filter_select(d, &eg, a.tp, &|d, c| cfilter(d, c, a.e, 1 - a.tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    true
}
/// `s.adop(e,tp,eg,ep,ev,re,r,rp)`
fn adop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
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
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
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
