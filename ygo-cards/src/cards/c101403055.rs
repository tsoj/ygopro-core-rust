//! Reindsurm Conquest
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 101403055;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card can be used to Ritual Summon any number of Insect Ritual Monsters. You must also shuffle Insect and/or Dragon monsters from the GY(s) into the Deck whose total Levels equal or exceed the total Levels of the Ritual Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK);
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
    // If this card is in your GY: You can send 1 Insect monster from your hand or face-up field to the GY; add this card to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.thcost)
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.matfilter(c)
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_INSECT|RACE_DRAGON) and c:HasLevel() and c:IsAbleToDeck()
    card::is_race(d, c, (RACE_INSECT | RACE_DRAGON) as u64) && card::has_level(d, c) && card::is_able_to_deck(d, c)
}
// function s.spfilter(c,e,tp,max_lv)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, max_lv: u32) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsRitualMonster() and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_RITUAL,tp,false,true)
    //     and c:IsLevelBelow(max_lv)
    card::is_race(d, c, RACE_INSECT as u64)
        && card::is_ritual_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_RITUAL, tp, false, true, None, None, None)
        && card::is_level_below(d, c, max_lv)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        // local mg=Duel.GetMatchingGroup(s.matfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil)
        let mg = duel::get_matching_group(d, Some(&matfilter), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
        // return #mg>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp,mg:GetSum(Card.GetLevel))
        let max_lv = group::get_sum(d, &mg, &|d, c| card::get_level(d, c) as i64) as u32;
        return !mg.is_empty()
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp, max_lv)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_EITHER,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_EITHER as u8, LOCATION_GRAVE as i32);
    true
}
// function s.rescon(max_lv)
//     return function(sg,e,tp,mg)
//         local sg_lvsum=sg:GetSum(Card.GetLevel)
//         return sg_lvsum<=max_lv,sg_lvsum>max_lv
//     end
// end
fn rescon(d: &mut Duel, sg: &CardSet, max_lv: i64) -> (bool, bool) {
    let sg_lvsum = group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64);
    (sg_lvsum <= max_lv, sg_lvsum > max_lv)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local max_count=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut max_count = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    // if max_count<=0 then return end
    if max_count <= 0 {
        return;
    }
    // local mg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.matfilter),tp,LOCATION_GRAVE,LOCATION_GRAVE,nil)
    let mg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(matfilter)), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if #mg==0 then return end
    if mg.is_empty() {
        return;
    }
    // local max_lv=mg:GetSum(Card.GetLevel)
    let max_lv = group::get_sum(d, &mg, &|d, c| card::get_level(d, c) as i64);
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp,max_lv)
    let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp, max_lv as u32)), tp, LOCATION_HAND, 0, NO_EX);
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // max_count=math.min(max_count,#sg)
    max_count = max_count.min(sg.len() as i64);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then max_count=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        max_count = 1;
    }
    // local rescon=s.rescon(max_lv)
    // local summon_group=aux.SelectUnselectGroup(sg,e,tp,1,max_count,rescon,1,tp,HINTMSG_SPSUMMON,rescon)
    let rescon_fn = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| rescon(d, sg, max_lv);
    let summon_group = aux::select_unselect_group(
        d,
        &sg,
        Some(e),
        tp,
        Some(1),
        Some(max_count as usize),
        Some(&rescon_fn),
        1,
        tp,
        Some(HINTMSG_SPSUMMON as u64),
        Some(&rescon_fn),
        None,
        false,
    )
    .unwrap_or_default();
    // if #summon_group==0 then return end
    if summon_group.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local final_mats=mg:SelectWithSumGreater(tp,Card.GetLevel,summon_group:GetSum(Card.GetLevel))
    let summon_sum = group::get_sum(d, &summon_group, &|d, c| card::get_level(d, c) as i64) as u32;
    let final_mats = group::select_with_sum_greater(d, &mg, tp, &|d, c| card::get_level(d, c) as i64, summon_sum).nil();
    // if #final_mats==0 then return end
    if final_mats.is_empty() {
        return;
    }
    // for sc in summon_group:Iter() do
    for sc in group::iter(d, &summon_group) {
        // sc:SetMaterial(final_mats)
        card::set_material(d, sc, final_mats.clone());
    }
    // Duel.HintSelection(final_mats)
    duel::hint_selection(d, final_mats.clone(), true);
    // Duel.SendtoDeck(final_mats,nil,SEQ_DECKSHUFFLE,REASON_EFFECT|REASON_RITUAL|REASON_MATERIAL)
    duel::send_to_deck(d, final_mats, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT | REASON_RITUAL | REASON_MATERIAL, None);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // for sc in summon_group:Iter() do
    for sc in group::iter(d, &summon_group) {
        // Duel.SpecialSummonStep(sc,SUMMON_TYPE_RITUAL,tp,tp,false,true,POS_FACEUP)
        duel::special_summon_step(d, sc, SUMMON_TYPE_RITUAL, tp, tp, false, true, POS_FACEUP as u8, None);
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.thcostfilter(c)
fn thcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_INSECT) and (c:IsFaceup() or c:IsLocation(LOCATION_HAND)) and c:IsAbleToGraveAsCost()
    card::is_race(d, c, RACE_INSECT as u64) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND)) && card::is_able_to_grave_as_cost(d, c)
}
// function s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thcostfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thcostfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thcostfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thcostfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
    }
}
