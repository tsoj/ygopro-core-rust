//! Seventh Tachyon
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 7477101;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(-1)
    effect::set_label(d, a.e, &[-1]);
    // return true
    true
}
/// `s.revfilter(c,tp)`
fn revfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // if not (c:IsSetCard(SET_NUMBER) and not c:IsPublic() and c:IsType(TYPE_XYZ)) then return false end
    if !(card::is_set_card(d, c, &[SET_NUMBER as u16]) && !card::is_public(d, c) && card::is_type(d, c, TYPE_XYZ)) {
        return false;
    }
    // local number=c.xyz_number
    let number = crate::card_info(d, c).xyz_number;
    // return number and number>=101 and number<=107 and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetRace(),c:GetAttribute(),c:GetRank())
    match number {
        Some(n) if (101..=107).contains(&n) => {
            let race = card::get_race(d, c);
            let attr = card::get_attribute(d, c);
            let rank = card::get_rank(d, c);
            duel::is_existing_matching_card(d, Some(&|d, cc| thfilter(d, cc, race, attr, rank)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
        _ => false,
    }
}
/// `s.thfilter(c,race,attr,rank)`
fn thfilter(d: &mut Duel, c: CardId, race: u64, attr: u32, rank: u32) -> bool {
    // return (c:IsRace(race) or c:IsAttribute(attr)) and c:IsLevel(rank) and c:IsAbleToHand()
    (card::is_race(d, c, race) || card::is_attribute(d, c, attr)) && card::is_level(d, c, &[rank]) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=e:GetLabel()==-1 and Duel.IsExistingMatchingCard(s.revfilter,tp,LOCATION_EXTRA,0,1,nil,tp)
        let res = effect::get_label(d, a.e) == -1 && duel::is_existing_matching_card(d, Some(&|d, c| revfilter(d, c, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return res
        return res;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.revfilter,tp,LOCATION_EXTRA,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| revfilter(d, c, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &g).nil();
    // e:SetLabel(rc:GetRace(),rc:GetAttribute(),rc:GetRank())
    let race = card::get_race(d, rc) as i64;
    let attr = card::get_attribute(d, rc) as i64;
    let rank = card::get_rank(d, rc) as i64;
    effect::set_label(d, a.e, &[race, attr, rank]);
    // Duel.ConfirmCards(1-tp,rc)
    duel::confirm_cards(d, 1 - tp, rc);
    // Duel.ShuffleExtra(tp)
    duel::shuffle_extra(d, tp);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local race,attr,rank=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let race = labels.first().copied().unwrap_or(0) as u64;
    let attr = labels.get(1).copied().unwrap_or(0) as u32;
    let rank = labels.get(2).copied().unwrap_or(0) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,race,attr,rank):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, race, attr, rank)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) then
    if let Some(sc) = sc {
        if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - tp, sc);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // Duel.ShuffleDeck(tp)
            duel::shuffle_deck(d, tp);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,1,nil)
            let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if group::get_count(&g) > 0 {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT)
                duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
            }
        }
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot Special Summon from the Extra Deck, except Xyz Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_XYZ) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_XYZ))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Clock Lizard check
    // aux.addTempLizardCheck(c,tp,function(e,c) return not c:IsOriginalType(TYPE_XYZ) end)
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, c| !card::is_original_type(d, c, TYPE_XYZ))), None, None, None, None);
}
