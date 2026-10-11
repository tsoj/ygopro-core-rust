//! Arionpos, Serpent of the Ghoti
use super::prelude::*;

pub const ID: u32 = 65910922;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.rmvtg)
    effect::set_target(d, e1, Some(cost_target(rmvtg)));
    // e1:SetOperation(s.rmvop)
    effect::set_operation(d, e1, Some(operation(rmvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE+CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thcond)
    effect::set_condition(d, e2, Some(condition(thcond)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.rmvfilter(c)
fn rmvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FISH) and c:IsLevelBelow(6) and c:IsAbleToRemove()
    card::is_race(d, c, RACE_FISH) && card::is_level_below(d, c, 6) && card::is_able_to_remove(d, c, None, None, None)
}
// function(e) return e:GetHandler():IsSynchroSummoned() end
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, c)
}
// function s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmvfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmvfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.rmvop(e,tp,eg,ep,ev,re,r,rp)
fn rmvop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rmvfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&rmvfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
// function s.thcond(e,tp,eg,ep,ev,re,r,rp)
fn thcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and r&REASON_SYNCHRO>0
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE) && (a.r & REASON_SYNCHRO) > 0
}
// function s.tgtfilter(c)
fn tgtfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FISH) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_FISH) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.tgtfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && tgtfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgtfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgtfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.tgtfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&tgtfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thfilter(c,lv)
fn thfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsRace(RACE_FISH) and c:IsLevelBelow(lv) and c:IsAbleToHand()
    card::is_race(d, c, RACE_FISH) && card::is_level_below(d, c, lv) && card::is_able_to_hand(d, c, None)
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsRace(RACE_FISH) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0
    if card::is_relate_to_effect(d, tc, a.e) && card::is_race(d, tc, RACE_FISH) && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,tc:GetLevel())
        let lv = card::get_level(d, tc);
        // and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, lv)), a.tp, LOCATION_DECK, 0, 1, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tc:GetLevel())
            let lv2 = card::get_level(d, tc);
            let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, lv2)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(g,nil,REASON_EFFECT)
                duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,g)
                duel::confirm_cards(d, 1 - a.tp, g);
            }
        }
    }
}
