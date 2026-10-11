//! Spirit Converter
use super::prelude::*;

pub const ID: u32 = 20802187;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.mattg)
    effect::set_target(d, e2, Some(cost_target(mattg)));
    // e2:SetOperation(s.matop)
    effect::set_operation(d, e2, Some(operation(matop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // salvage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e3, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.xyzfilter(c)
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)
}
// function s.matfilter(c)
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_LIGHT) and c:GetLevel()==4 and not c:IsType(TYPE_TOKEN)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::get_level(d, c) == 4 && !card::is_type(d, c, TYPE_TOKEN)
}
// function s.mattg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn mattg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.xyzfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && xyzfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.matfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&xyzfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&matfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&xyzfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.matop(e,tp,eg,ep,ev,re,r,rp)
fn matop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        // local g=Duel.SelectMatchingCard(tp,s.matfilter,tp,LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&matfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Overlay(tc,g)
            duel::overlay(d, tc, g, false);
        }
    }
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
// function s.thfilter(c,e)
fn thfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsRace(RACE_THUNDER) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:GetLevel()==4 and c:IsCanBeEffectTarget(e)
    card::is_race(d, c, RACE_THUNDER) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::get_level(d, c) == 4 && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.thfilter2(c,g)
fn thfilter2(d: &mut Duel, c: CardId, g: &CardSet) -> bool {
    // return g:IsExists(Card.IsCode,1,c,c:GetCode())
    let codes = card::get_code(d, c);
    group::is_exists(d, g, &|d, x| card::is_code(d, x, &codes), 1, c.into())
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.thfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && thfilter(d, chkc, a.e);
    }
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c, a.e)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g1=g:FilterSelect(tp,s.thfilter2,1,1,nil,g)
    let mut g1 = group::filter_select(d, &g, tp, &|d, c| thfilter2(d, c, &g), 1, 1, false, NO_EX).nil();
    // if #g1>0 then
    if !g1.is_empty() {
        // local g2=g:FilterSelect(tp,Card.IsCode,1,1,g1:GetFirst(),g1:GetFirst():GetCode())
        let first = group::get_first(d, &g1).nil();
        let codes = card::get_code(d, first);
        let g2 = group::filter_select(d, &g, tp, &|d, c| card::is_code(d, c, &codes), 1, 1, false, first.into()).nil();
        // g1:Merge(g2)
        group::add_card(&mut g1, &g2);
        // Duel.SetTargetCard(g1)
        duel::set_target_card(d, g1.clone());
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g1,2,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g1), 2, 0, 0);
    }
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    // if not g then return end
    let Some(g) = g else {
        return;
    };
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
