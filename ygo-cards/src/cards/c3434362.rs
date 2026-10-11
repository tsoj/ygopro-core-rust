//! Rustin Mammoth
//! ラスティン・マンモス
use super::prelude::*;

pub const ID: u32 = 3434362;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is in your hand: You can banish Machine Link Monsters from your Extra Deck whose combined Link Ratings equal exactly 5; Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can target 1 card you control and 1 card your opponent controls; return them to the hand
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2a, CATEGORY_TOHAND);
    // e2a:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2a, EFFECT_TYPE_IGNITION);
    // e2a:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2a, EFFECT_FLAG_CARD_TARGET, 0);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetCondition(aux.NOT(s.rthquickcon))
    effect::set_condition(d, e2a, Some(condition(|d, a| !rthquickcon(d, a))));
    // e2a:SetTarget(s.rthtg)
    effect::set_target(d, e2a, Some(cost_target(rthtg)));
    // e2a:SetOperation(s.rthop)
    effect::set_operation(d, e2a, Some(operation(rthop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // This is a Quick Effect if this card is linked to a Link-3 or higher Machine Link Monster
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2b, EFFECT_TYPE_QUICK_O);
    // e2b:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2b, EVENT_FREE_CHAIN);
    // e2b:SetCondition(s.rthquickcon)
    effect::set_condition(d, e2b, Some(condition(rthquickcon)));
    // e2b:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2b, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `s.spcostfilter(c)`
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsLinkMonster() and c:IsAbleToRemoveAsCost()
    card::is_race(d, c, RACE_MACHINE as u64) && card::is_link_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetSum(Card.GetLink)==5
    (group::get_sum(d, sg, &|d, x| card::get_link(d, x) as i64) == 5, false)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&spcostfilter), tp, LOCATION_EXTRA, 0, NO_EX);
    // if chk==0 then return #g>0 and aux.SelectUnselectGroup(g,e,tp,1,5,s.rescon,0) end
    if a.chk == Some(0) {
        return !g.is_empty()
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(5), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,1,5,s.rescon,1,tp,HINTMSG_REMOVE,s.rescon)
    let rg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(5), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), Some(&rescon), None, false)
        .unwrap_or_default();
    // Duel.Remove(rg,POS_FACEUP,REASON_COST)
    duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE,0)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(0), None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `aux.dpcheck(Card.GetControler)`
fn controler_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_controler(d, c) as i64
}
/// `s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetTargetGroup(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_target_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    let dp = aux::dpcheck(controler_value);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),0) end
    if a.chk == Some(0) {
        return g.len() >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&dp), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_RTOHAND)
    let tg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&dp), 1, tp, Some(HINTMSG_RTOHAND as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), 2, tp, 0);
    true
}
/// `s.rthop(e,tp,eg,ep,ev,re,r,rp)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then Duel.SendtoHand(tg,nil,REASON_EFFECT) end
    if !tg.is_empty() {
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
/// `s.rthquickconfilter(c,ec,lg)`
fn rthquickconfilter(d: &mut Duel, c: CardId, ec: CardId, lg: &CardSet) -> bool {
    // return c:IsLinkAbove(3) and c:IsRace(RACE_MACHINE) and c:IsFaceup() and (c:GetLinkedGroup():IsContains(ec) or lg:IsContains(c))
    if !(card::is_link_above(d, c, 3) && card::is_race(d, c, RACE_MACHINE as u64) && card::is_faceup(d, c)) {
        return false;
    }
    let linked = card::get_linked_group(d, c);
    group::is_contains(&linked, ec) || group::is_contains(lg, c)
}
/// `s.rthquickcon(e,tp,eg,ep,ev,re,r,rp)`
fn rthquickcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.IsExistingMatchingCard(s.rthquickconfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,c,c:GetLinkedGroup())
    let lg = card::get_linked_group(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| rthquickconfilter(d, x, c, &lg)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
