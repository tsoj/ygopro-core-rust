//! The Monarchs Revolt
use super::prelude::*;

pub const ID: u32 = 9283801;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reveal 3 monsters with 800 or 2400 or more ATK, and 1000 DEF, from your Deck, your opponent chooses 1 for you to add to your hand, and you send the rest to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.Discard(nil,true))
    effect::set_cost(d, e1, Some(cost::discard(None, true, None, None, None)));
    // e1:SetTarget(s.thtgtg)
    effect::set_target(d, e1, Some(cost_target(thtgtg)));
    // e1:SetOperation(s.thtgop)
    effect::set_operation(d, e1, Some(operation(thtgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 monster with 800 ATK/1000 DEF from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.revfilter(c)
fn revfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsAttack(800) or c:IsAttackAbove(2400)) and c:IsDefense(1000) and (c:IsAbleToHand() or c:IsAbleToGrave())
    (card::is_attack(d, c, &[800]) || card::is_attack_above(d, c, 2400))
        && card::is_defense(d, c, &[1000])
        && (card::is_able_to_hand(d, c, None) || card::is_able_to_grave(d, c, None))
}
// function s.thtgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.revfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&revfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return #g>=3 and g:IsExists(Card.IsAbleToHand,1,nil) end
    if a.chk == Some(0) {
        return g.len() >= 3 && group::is_exists(d, &g, &|d, c| card::is_able_to_hand(d, c, None), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsAbleToHand,1,nil)
    (group::is_exists(d, sg, &|d, c| card::is_able_to_hand(d, c, None), 1, NO_EX), false)
}
// function s.thtgop(e,tp,eg,ep,ev,re,r,rp)
fn thtgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.revfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&revfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>=3 and g:IsExists(Card.IsAbleToHand,1,nil) then
    if g.len() >= 3 && group::is_exists(d, &g, &|d, c| card::is_able_to_hand(d, c, None), 1, NO_EX) {
        // local rg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_CONFIRM)
        let rg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(3), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_CONFIRM as u64), None, None, false).unwrap_or_default();
        // Duel.ConfirmCards(1-tp,rg)
        duel::confirm_cards(d, 1 - tp, rg.clone());
        // Duel.Hint(HINT_SELECTMSG,1-tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, aux::stringid(ID, 2));
        // local sc=rg:FilterSelect(1-tp,Card.IsAbleToHand,1,1,nil):GetFirst()
        let sel = group::filter_select(d, &rg, 1 - tp, &|d, c| card::is_able_to_hand(d, c, None), 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel).nil();
        // Duel.SendtoHand(sc,nil,REASON_EFFECT)
        duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
        // Duel.SendtoGrave(rg-sc,REASON_EFFECT)
        let mut rest = rg.clone();
        group::remove_card(&mut rest, &CardSet::from_card(sc));
        duel::send_to_grave(d, rest, REASON_EFFECT, None, None);
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // --You cannot Special Summon from the Extra Deck for the rest of this turn after this card resolves
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
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
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttack(800) and c:IsDefense(1000) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attack(d, c, &[800])
        && card::is_defense(d, c, &[1000])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
