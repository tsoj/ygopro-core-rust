//! Ryu-Ge War Zone
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 55276522;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[92487128], listed_series: &[SET_RYU_GE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Take 3 monsters from your Deck (1 Dinosaur, 1 Sea Serpent, and 1 Wyrm)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_REMOVE+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_REMOVE + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 3 "Ryu-Ge" monsters with different Types, 1 each from your Deck, GY, and banishment
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,92487128),tp,LOCATION_EXTRA,0,1,nil) end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, a| {
            duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[92487128]))), a.tp, LOCATION_EXTRA, 0, 1, NO_EX)
        })),
    );
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DINOSAUR|RACE_SEASERPENT|RACE_WYRM) and (c:IsAbleToHand() or c:IsAbleToRemove() or c:IsAbleToGrave())
    card::is_race(d, c, RACE_DINOSAUR | RACE_SEASERPENT | RACE_WYRM)
        && (card::is_able_to_hand(d, c, None) || card::is_able_to_remove(d, c, None, None, None) || card::is_able_to_grave(d, c, None))
}
/// `s.thcheck(sg,e,tp,mg)`
fn thcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:CheckDifferentProperty(Card.GetRace) and sg:FilterCount(Card.IsAbleToHand,nil)>=1
    //     and sg:FilterCount(Card.IsAbleToRemove,nil)>=1 and sg:FilterCount(Card.IsAbleToGrave,nil)>=1
    let res = group::check_different_property(d, sg, &|d, c| vec![card::get_race(d, c) as i64])
        && group::filter_count(d, sg, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) >= 1
        && group::filter_count(d, sg, &|d, c| card::is_able_to_remove(d, c, None, None, None), NO_EX) >= 1
        && group::filter_count(d, sg, &|d, c| card::is_able_to_grave(d, c, None), NO_EX) >= 1;
    (res, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,3,3,s.thcheck,0)
        let rs: aux::Rescon = &thcheck;
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), tp, Some(3), Some(3), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let sg = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #sg>=3 then
    if sg.len() >= 3 {
        // local g=aux.SelectUnselectGroup(sg,e,tp,3,3,s.thcheck,1,tp,aux.Stringid(id,2))
        let rs: aux::Rescon = &thcheck;
        let g = aux::select_unselect_group(d, &sg, Some(a.e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(aux::stringid(ID, 2)), None, None, false)
            .unwrap_or_default();
        // if #g==3 then
        if g.len() == 3 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local hg=g:FilterSelect(tp,Card.IsAbleToHand,1,1,nil)
            let hg = group::filter_select(d, &g, tp, &|d, c| card::is_able_to_hand(d, c, None), 1, 1, false, NO_EX).nil();
            // if #hg>0 and Duel.SendtoHand(hg,nil,REASON_EFFECT)>0 then
            if !hg.is_empty() && duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None) > 0 {
                // Duel.ConfirmCards(1-tp,hg)
                duel::confirm_cards(d, 1 - tp, hg.clone());
                // g=g-hg
                let mut g = group::difference(&g, &hg);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
                // local rg=g:FilterSelect(tp,Card.IsAbleToRemove,1,1,nil)
                let rg = group::filter_select(d, &g, tp, &|d, c| card::is_able_to_remove(d, c, None, None, None), 1, 1, false, NO_EX).nil();
                // if #rg>0 and Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)>0 then
                if !rg.is_empty() && duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
                    // g=g-rg
                    g = group::difference(&g, &rg);
                    // Duel.SendtoGrave(g,REASON_EFFECT)
                    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
                }
            }
        }
    }
    // You cannot Special Summon for the rest of this turn, except Dragon, Dinosaur, Sea Serpent, and Wyrm monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsRace(RACE_DRAGON|RACE_DINOSAUR|RACE_SEASERPENT|RACE_WYRM) end)
    effect::set_target(
        d,
        e1,
        Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_DRAGON | RACE_DINOSAUR | RACE_SEASERPENT | RACE_WYRM))),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_RYU_GE) and (c:IsFaceup() or not c:IsLocation(LOCATION_REMOVED)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_RYU_GE as u16])
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spcheck(sg,e,tp)`
fn spcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:CheckDifferentProperty(Card.GetLocation) and sg:CheckDifferentProperty(Card.GetRace)
    let res = group::check_different_property(d, sg, &|d, c| vec![card::get_location(d, c) as i64])
        && group::check_different_property(d, sg, &|d, c| vec![card::get_race(d, c) as i64]);
    (res, false)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED,0,nil,e,tp)
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| spfilter(d, c, a.e, tp)),
            tp,
            LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED,
            0,
            NO_EX,
        );
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>2 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and aux.SelectUnselectGroup(g,e,tp,3,3,s.spcheck,0)
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 2 {
            return false;
        }
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT as u32).is_some() {
            return false;
        }
        let rs: aux::Rescon = &spcheck;
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), tp, Some(3), Some(3), Some(rs), 0, tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,3,tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, tp, (LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<3 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 3 {
        return;
    }
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT as u32).is_some() {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_DECK|LOCATION_GRAVE|LOCATION_REMOVED,0,nil,e,tp)
    let g = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp))),
        tp,
        LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        NO_EX,
    );
    // if #g<3 then return end
    if g.len() < 3 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.spcheck,1,tp,HINTMSG_SPSUMMON)
    let rs: aux::Rescon = &spcheck;
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #sg==3 then
    if sg.len() == 3 {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
