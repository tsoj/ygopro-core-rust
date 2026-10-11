//! Votis
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 75311421;
pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.exctg)
    effect::set_target(d, e1, Some(cost_target(exctg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Excavate and sort cards
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.sdcon)
    effect::set_condition(d, e2, Some(condition(sdcon)));
    // e2:SetTarget(s.exctg)
    effect::set_target(d, e2, Some(cost_target(exctg)));
    // e2:SetOperation(s.sdop)
    effect::set_operation(d, e2, Some(operation(sdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.exctg(e,tp,eg,ep,ev,re,r,rp,chk)
fn exctg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=3 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) >= 3;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thfilter(c,code)
fn thfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsMonster() and c:IsCode(code) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
// function s.rmfilter(c,tp)
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove() and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,c,c:GetCode())
    if !(card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)) {
        return false;
    }
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, code)), tp, LOCATION_DECK, 0, 1, c.into())
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<3 then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) < 3 {
        return;
    }
    // Duel.ConfirmDecktop(tp,3)
    duel::confirm_decktop(d, a.tp, 3);
    // local dg=Duel.GetDecktopGroup(tp,3)
    let dg = duel::get_decktop_group(d, a.tp, 3);
    // if dg:IsExists(s.rmfilter,1,nil,tp) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if group::is_exists(d, &dg, &|d, c| rmfilter(d, c, a.tp), 1, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local tc=dg:FilterSelect(tp,s.rmfilter,1,1,nil,tp):GetFirst()
        let sel = group::filter_select(d, &dg, a.tp, &|d, c| rmfilter(d, c, a.tp), 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel);
        // if tc and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_REMOVED) then
        if let Some(tc) = tc {
            if duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 && card::is_location(d, tc, LOCATION_REMOVED) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
                // local sg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tc:GetCode())
                let code = card::get_code(d, tc)[0];
                let sg = duel::select_matching_card(d, a.tp, Some(&|d, x| thfilter(d, x, code)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
                // if #sg>0 then
                if !sg.is_empty() {
                    // Duel.SendtoHand(sg,nil,REASON_EFFECT)
                    duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
                    // Duel.ConfirmCards(1-tp,sg)
                    duel::confirm_cards(d, 1 - a.tp, sg);
                }
            }
        }
    }
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, a.tp);
}
// function s.sdcon(e,tp,eg,ep,ev,re,r,rp)
fn sdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetTurnID()==Duel.GetTurnCount() and c:IsPreviousLocation(LOCATION_MZONE) and c:IsNormalSummoned()
    (card::get_turn_id(d, c) as i32) == duel::get_turn_count(d, None) && card::is_previous_location(d, c, LOCATION_MZONE) && card::is_normal_summoned(d, c)
}
// function s.sdop(e,tp,eg,ep,ev,re,r,rp)
fn sdop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<3 then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) < 3 {
        return;
    }
    // Duel.ConfirmDecktop(tp,3)
    duel::confirm_decktop(d, a.tp, 3);
    // Duel.SortDecktop(tp,tp,3)
    duel::sort_decktop(d, a.tp, a.tp, 3);
}
