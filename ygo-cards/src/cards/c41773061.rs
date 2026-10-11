//! Voici la Carte (Today's Menu)
use super::prelude::*;

pub const ID: u32 = 41773061;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[87778106, 14166715], listed_series: &[SET_NOUVELLES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Take 2 "Nouvelles" monsters and make the opponent select 1 of them to add to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.revfilter(c)`
fn revfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NOUVELLES) and c:IsMonster() and c:IsAbleToHand() and not c:IsPublic()
    card::is_set_card(d, c, &[SET_NOUVELLES as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && !card::is_public(d, c)
}
/// `s.thfilter(c,rac)`
fn thfilter(d: &mut Duel, c: CardId, rac: u64) -> bool {
    // if not c:IsAbleToHand() then return false end
    if !card::is_able_to_hand(d, c, None) {
        return false;
    }
    // if rac==RACE_BEASTWARRIOR then return c:IsCode(87778106)
    if rac == RACE_BEASTWARRIOR {
        return card::is_code(d, c, &[87778106]);
    } else if rac == RACE_WARRIOR {
        // elseif rac==RACE_WARRIOR then return c:IsCode(14166715)
        return card::is_code(d, c, &[14166715]);
    }
    // end (returns nil, which is false)
    false
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.revfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&revfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,0) end
    if a.chk == Some(0) {
        return g.len() >= 2 && matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::dncheck()), 0, a.tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.revfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&revfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_CONFIRM)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::dncheck()), 1, a.tp, Some(HINTMSG_CONFIRM as u64), None, None, false).unwrap_or_default();
    // Duel.ConfirmCards(1-tp,sg)
    duel::confirm_cards(d, 1 - a.tp, sg.clone());
    // Duel.Hint(HINT_SELECTMSG,1-tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, aux::stringid(ID, 1));
    // local g=sg:Select(1-tp,1,1,nil)
    let g = group::select(d, &sg, 1 - a.tp, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
        // local rac=g:GetFirst():GetRace()
        let first = group::get_first(d, &g).nil();
        let rac = card::get_race(d, first);
        // if Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,rac)
        //    and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, c| thfilter(d, c, rac))), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
            // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,rac)
            let tc = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| thfilter(d, c, rac))), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            // if #tc>0 then
            if !tc.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoHand(tc,nil,REASON_EFFECT)
                duel::send_to_hand(d, tc.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,tc)
                duel::confirm_cards(d, 1 - a.tp, tc);
            }
        }
    }
}
