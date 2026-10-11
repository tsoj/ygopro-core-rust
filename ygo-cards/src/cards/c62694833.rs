//! Brotherhood of the Fire Fist - Ram
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 62694833;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FIRE_FORMATION, SET_FIRE_FIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When normal summoned, set 1 "Fire Formation" spell/trap from deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.setcost)
    effect::set_cost(d, e1, Some(cost_target(setcost)));
    // e1:SetTarget(s.settg)
    effect::set_target(d, e1, Some(cost_target(settg)));
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If special summoned by effect of a "Fire Fist" monster, set 1 "Fire Formation" spell/trap from deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.setcon2)
    effect::set_condition(d, e2, Some(condition(setcon2)));
    // e2:SetTarget(s.settg2)
    effect::set_target(d, e2, Some(cost_target(settg2)));
    // e2:SetOperation(s.setop2)
    effect::set_operation(d, e2, Some(operation(setop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn setcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn setfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_FIRE_FORMATION) and c:IsSpellTrap()
    //   and Duel.IsExistingMatchingCard(s.setfilter2,tp,LOCATION_DECK,0,1,nil,c)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_FIRE_FORMATION as u16])
        && card::is_spell_trap(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| setfilter2(d, x, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
fn setfilter2(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // return c:IsSetCard(SET_FIRE_FORMATION) and c:IsSpellTrap() and not c:IsCode(tc:GetCode()) and c:IsSSetable()
    let codes = card::get_code(d, tc);
    card::is_set_card(d, c, &[SET_FIRE_FORMATION as u16]) && card::is_spell_trap(d, c) && !card::is_code(d, c, &codes) && card::is_s_setable(d, c, false, None)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsOnField() and s.setfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_on_field(d, chkc) && setfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.setfilter,tp,LOCATION_ONFIELD,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| setfilter(d, c, tp)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.setfilter,tp,LOCATION_ONFIELD,0,1,1,nil,tp)
    duel::select_target(d, tp, Some(&|d, c| setfilter(d, c, tp)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter2,tp,LOCATION_DECK,0,1,1,nil,tc)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| setfilter2(d, x, tc)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g:GetFirst())
        let first = group::get_first(d, &g).nil();
        duel::s_set(d, a.tp, CardSet::from_card(first), None, true);
    }
}
fn setcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:IsMonsterEffect() and re:GetHandler():IsSetCard(SET_FIRE_FIST)
    match a.re {
        None => false,
        Some(re) => {
            effect::is_monster_effect(d, re) && {
                let rh = effect::get_handler(d, re).nil();
                card::is_set_card(d, rh, &[SET_FIRE_FIST as u16])
            }
        }
    }
}
fn setfilter3(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_FIRE_FORMATION) and c:IsSpellTrap() and c:IsSSetable()
    //   and not Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,c:GetCode())
    let codes = card::get_code(d, c);
    card::is_set_card(d, c, &[SET_FIRE_FORMATION as u16])
        && card::is_spell_trap(d, c)
        && card::is_s_setable(d, c, false, None)
        && !duel::is_existing_matching_card(d, Some(&|d, x| card::is_code(d, x, &codes)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
fn settg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter3,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, c| setfilter3(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
fn setop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter3,tp,LOCATION_DECK,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| setfilter3(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g:GetFirst())
        let first = group::get_first(d, &g).nil();
        duel::s_set(d, tp, CardSet::from_card(first), None, true);
    }
}
