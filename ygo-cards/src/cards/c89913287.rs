//! 魔救の奇跡－ティアマイト
use super::prelude::*;

pub const ID: u32 = 89913287;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ADAMANCIPATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuners
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // If you have a DARK monster in your GY: You can add 1 "Adamancipator" Spell/Trap from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp)
    //     return Duel.IsExistingMatchingCard(Card.IsAttribute,tp,LOCATION_GRAVE,0,1,nil,ATTRIBUTE_DARK)
    // end)
    effect::set_condition(d, e1, Some(condition(dark_in_gy_cond)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When your opponent activates a monster effect on the field (Quick Effect): You can excavate the top 5 cards of your Deck, and if you do, you can return cards your opponent controls to the hand, up to the number of excavated Rock monsters, also place the excavated cards on the bottom of the Deck in any order
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp)
    //     return ep==1-tp and Chain.IsTriggeringLocation(ev,LOCATION_MZONE)
    // end)
    effect::set_condition(d, e2, Some(condition(opp_trigger_mzone_cond)));
    // e2:SetTarget(s.excavtg)
    effect::set_target(d, e2, Some(cost_target(excavtg)));
    // e2:SetOperation(s.excavop)
    effect::set_operation(d, e2, Some(operation(excavop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn dark_in_gy_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // Duel.IsExistingMatchingCard(Card.IsAttribute,tp,LOCATION_GRAVE,0,1,nil,ATTRIBUTE_DARK)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_attribute(d, c, ATTRIBUTE_DARK)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
fn opp_trigger_mzone_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and Chain.IsTriggeringLocation(ev,LOCATION_MZONE)
    a.ep == 1 - a.tp && chain::is_triggering(d, a.ev as u8, chain::Prop::Location, &[LOCATION_MZONE as i64]).truthy()
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsSetCard(SET_ADAMANCIPATOR) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ADAMANCIPATOR as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
fn excavtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=5 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_DECK, 0) >= 5;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
fn excavop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)==0 then return end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) == 0 {
        return;
    }
    // Duel.ConfirmDecktop(tp,5)
    duel::confirm_decktop(d, tp, 5);
    // local excavg=Duel.GetDecktopGroup(tp,5)
    let excavg = duel::get_decktop_group(d, tp, 5);
    // local excav_count=#excavg
    let excav_count = excavg.len() as u32;
    // local excav_rock_count=excavg:FilterCount(Card.IsRace,nil,RACE_ROCK)
    let excav_rock_count = group::filter_count(d, &excavg, &|d, c| card::is_race(d, c, RACE_ROCK), NO_EX);
    // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if excav_rock_count>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if excav_rock_count > 0 && !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // local return_count=math.min(#g,excav_rock_count)
        let return_count = g.len().min(excav_rock_count) as u16;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        // local sg=g:Select(tp,1,return_count,nil)
        let sg = group::select(d, &g, tp, 1, return_count, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.HintSelection(sg)
            duel::hint_selection(d, &sg, true);
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
            // local og=Duel.GetOperatedGroup():Match(Card.IsLocation,nil,LOCATION_HAND)
            let mut og = duel::get_operated_group(d);
            group::match_(d, &mut og, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
            // if #og>0 then
            if !og.is_empty() {
                // if og:IsExists(Card.IsControler,1,nil,tp) then Duel.ShuffleHand(tp) end
                if group::is_exists(d, &og, &|d, c| card::is_controler(d, c, tp), 1, NO_EX) {
                    duel::shuffle_hand(d, tp);
                }
                // if og:IsExists(Card.IsControler,1,nil,1-tp) then Duel.ShuffleHand(1-tp) end
                if group::is_exists(d, &og, &|d, c| card::is_controler(d, c, 1 - tp), 1, NO_EX) {
                    duel::shuffle_hand(d, 1 - tp);
                }
            }
        }
    }
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>excav_count then
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > excav_count as i32 {
        // Duel.MoveToDeckBottom(excav_count,tp)
        duel::move_decktop_to_bottom(d, excav_count, tp);
    }
    // Duel.SortDeckbottom(tp,tp,excav_count)
    duel::sort_deckbottom(d, tp, tp, excav_count as u16);
}
