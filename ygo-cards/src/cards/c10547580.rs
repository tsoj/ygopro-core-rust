//! Ancient Gear Ballista
use super::prelude::*;

pub const ID: u32 = 10547580;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[37694547],
    listed_series: &[SET_ANCIENT_GEAR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Materials: 2 EARTH Machine monsters
    // Link.AddProcedure(c,s.matfilter,2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 2, Some(2), None, None, None);
    // Add 1 "Ancient Gear" monster or "Geartown" from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(linkcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy your S/T, and if you do, monster's ATK/DEF become 0
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.matfilter(c,scard,sumtype,tp)
fn matfilter(d: &mut Duel, c: CardId, scard: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_MACHINE,scard,sumtype,tp) and c:IsAttribute(ATTRIBUTE_EARTH,scard,sumtype,tp)
    card::is_race_ctx(d, c, RACE_MACHINE, scard, sumtype, Some(tp)) && card::is_attribute_ctx(d, c, ATTRIBUTE_EARTH, scard, sumtype, Some(tp))
}
// function(e) return e:GetHandler():IsLinkSummoned() end
fn linkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsSetCard(SET_ANCIENT_GEAR) and c:IsMonster()) or c:IsCode(37694547)) and c:IsAbleToHand()
    ((card::is_set_card(d, c, &[SET_ANCIENT_GEAR as u16]) && card::is_monster(d, c)) || card::is_code(d, c, &[37694547]))
        && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
// function s.atkdeffilter(c)
fn atkdeffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:GetAttack()>0 or c:GetDefense()>0)
    card::is_faceup(d, c) && (card::get_attack(d, c) > 0 || card::get_defense(d, c) > 0)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsSpellTrap,tp,LOCATION_ONFIELD,0,1,nil)
    //   and Duel.IsExistingTarget(s.atkdeffilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&atkdeffilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g1=Duel.SelectTarget(tp,Card.IsSpellTrap,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g1 = duel::select_target(d, a.tp, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // local g2=Duel.SelectTarget(tp,s.atkdeffilter,tp,0,LOCATION_MZONE,1,1,nil)
    let g2 = duel::select_target(d, a.tp, Some(&atkdeffilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g1, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g2,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, g2, 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g1,g2=Duel.GetFirstTarget()
    let _ = duel::get_first_target(d);
    // local ex,g1=Duel.GetOperationInfo(0,CATEGORY_DESTROY)
    let g1 = duel::get_operation_info(d, 0, CATEGORY_DESTROY).and_then(|o| o.op_cards).nil();
    // local ex,g2=Duel.GetOperationInfo(0,CATEGORY_ATKCHANGE)
    let g2 = duel::get_operation_info(d, 0, CATEGORY_ATKCHANGE).and_then(|o| o.op_cards).nil();
    // if g1:GetFirst():IsRelateToEffect(e) and Duel.Destroy(g1,REASON_EFFECT)>0 then
    let first = group::get_first(d, &g1).nil();
    if card::is_relate_to_effect(d, first, a.e) && duel::destroy(d, g1.clone(), REASON_EFFECT, None, None) > 0 {
        // local tc=g2:GetFirst()
        let tc = group::get_first(d, &g2).nil();
        // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(0)
            effect::set_value(d, e1, 0);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
