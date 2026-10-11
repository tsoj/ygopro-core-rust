//! Dark Tyranno
use super::prelude::*;

pub const ID: u32 = 46826483;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can attack directly if all monsters your opponent controls are in Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetCondition(s.directattackcond)
    effect::set_condition(d, e1, Some(condition(directattackcond)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase, if a Spell Card or effect was activated this turn: You can Special Summon this card from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function() return Duel.GetCustomActivityCount(id,0,ACTIVITY_CHAIN)>0 or Duel.GetCustomActivityCount(id,1,ACTIVITY_CHAIN)>0 end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,function(re,tp,cid) return not re:IsSpellEffect() end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(counter_filter));
    // If this card is sent to the GY: You can add 1 Level 8 or higher Dinosaur monster from your Deck to your hand, then you can Special Summon it if your opponent controls a monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.directattackcond(e)
fn directattackcond(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(e:GetHandlerPlayer(),0,LOCATION_MZONE)
    let p = effect::get_handler_player(d, a.e);
    let g = duel::get_field_group(d, p, 0, LOCATION_MZONE);
    // return #g==g:FilterCount(Card.IsDefensePos,nil)
    g.len() == group::filter_count(d, &g, &|d, c| card::is_defense_pos(d, c), NO_EX)
}
// function(e) return Duel.GetCustomActivityCount(id,0,ACTIVITY_CHAIN)>0 or Duel.GetCustomActivityCount(id,1,ACTIVITY_CHAIN)>0 end
fn e2_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_custom_activity_count(d, ID, 0, ACTIVITY_CHAIN) > 0 || duel::get_custom_activity_count(d, ID, 1, ACTIVITY_CHAIN) > 0
}
// function(re,tp,cid) return not re:IsSpellEffect() end
fn counter_filter(d: &mut Duel, a: Args) -> bool {
    let re = a.e(0);
    !effect::is_spell_effect(d, re)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(8) and c:IsRace(RACE_DINOSAUR) and c:IsAbleToHand()
    card::is_level_above(d, c, 8) && card::is_race(d, c, RACE_DINOSAUR) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if sc and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 then
    if let Some(sc) = group::get_first(d, &sel) {
        if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - tp, sc);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // if Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
            //     and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
                && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, None, None, None)
                && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
            {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
