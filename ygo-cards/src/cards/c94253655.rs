//! Plunder Patrollship Brann
//! 海造賊－豪速のブレンネ号
use super::prelude::*;

pub const ID: u32 = 94253655;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PLUNDER_PATROLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Your other fiend monsters gain 500 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 of opponent's spells/traps
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.condition1)
    effect::set_condition(d, e2, Some(condition(condition1)));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetHintTiming(0,TIMING_SSET|TIMING_EQUIP|TIMING_END_PHASE)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_SSET | TIMING_EQUIP | TIMING_END_PHASE));
    // e3:SetCondition(s.condition2)
    effect::set_condition(d, e3, Some(condition(condition2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsRace(RACE_FIEND) and c~=e:GetHandler()
    card::is_race(d, c, RACE_FIEND as u64) && c != effect::get_handler(d, e).nil()
}
/// `s.condition1(e,tp,eg,ep,ev,re,r,rp)`
fn condition1(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():GetEquipGroup():IsExists(Card.IsSetCard,1,nil,SET_PLUNDER_PATROLL)
    let h = effect::get_handler(d, a.e).nil();
    let eg = card::get_equip_group(d, h);
    !group::is_exists(d, &eg, &|d, c| card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16]), 1, NO_EX)
}
/// `s.condition2(e,tp,eg,ep,ev,re,r,rp)`
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipGroup():IsExists(Card.IsSetCard,1,nil,SET_PLUNDER_PATROLL)
    let h = effect::get_handler(d, a.e).nil();
    let eg = card::get_equip_group(d, h);
    group::is_exists(d, &eg, &|d, c| card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16]), 1, NO_EX)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PLUNDER_PATROLL) and c:IsDiscardable()
    card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16]) && card::is_discardable(d, c, None, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToRemove()
    card::is_spell_trap(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) and chkc:IsControler(1-tp) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_controler(d, chkc, 1 - tp) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tgfilter), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,g:GetFirst():GetLocation())
    let gg = g.clone().nil();
    let first = group::get_first(d, &gg).nil();
    let loc = card::get_location(d, first) as i32;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, loc);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PLUNDER_PATROLL) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
            // local sg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
            let sg = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
            // if #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
                // sg=sg:Select(tp,1,1,nil)
                let sg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(sg,nil,REASON_EFFECT)
                duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,sg)
                duel::confirm_cards(d, 1 - tp, sg);
            }
        }
    }
}
