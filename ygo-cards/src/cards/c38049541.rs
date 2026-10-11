//! Genex Ally Chemistrer
use super::prelude::*;

pub const ID: u32 = 38049541;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENEX], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change the Attribute of 1 "Genex" monster you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(s.attrtg)
    effect::set_target(d, e1, Some(cost_target(attrtg)));
    // e1:SetOperation(s.attrop)
    effect::set_operation(d, e1, Some(operation(attrop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_GENEX)`
fn genex_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GENEX as u16])
}
/// `s.attrtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn attrtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup()
    //     and chkc:IsSetCard(SET_GENEX) and chkc:IsAttributeExcept(e:GetLabel()) end
    if let Some(chkc) = a.chkc() {
        let label = effect::get_label(d, a.e) as u32;
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, tp)
            && card::is_faceup(d, chkc)
            && card::is_set_card(d, chkc, &[SET_GENEX as u16])
            && card::is_attribute_except(d, chkc, label, None, 0, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSetCard,SET_GENEX),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&genex_faceup), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsCanBeEffectTarget,aux.FaceupFilter(Card.IsSetCard,SET_GENEX)),tp,LOCATION_MZONE,0,nil,e)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_can_be_effect_target(d, c, Some(a.e)) && genex_faceup(d, c)),
        tp,
        LOCATION_MZONE,
        0,
        NO_EX,
    );
    // local att=Duel.AnnounceAnotherAttribute(g,tp)
    let att = duel::announce_another_attribute(d, &g, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local sel=g:FilterSelect(tp,Card.IsAttributeExcept,1,1,nil,att)
    let sel = group::filter_select(
        d,
        &g,
        tp,
        &|d, c| card::is_attribute_except(d, c, att, None, 0, None),
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // Duel.SetTargetCard(sel)
    duel::set_target_card(d, sel);
    // e:SetLabel(att)
    effect::set_label(d, a.e, &[att as i64]);
    true
}
/// `s.attrop(e,tp,eg,ep,ev,re,r,rp)`
fn attrop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
        // e1:SetValue(e:GetLabel())
        let label = effect::get_label(d, a.e);
        effect::set_value(d, e1, label);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
