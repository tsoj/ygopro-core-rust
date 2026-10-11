//! Abyssrhine, the Atlantean Spirit
//! 海皇精 アビスライン
use super::prelude::*;

pub const ID: u32 = 17080584;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ATLANTEAN, SET_MERMAIL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Take 1 Level 7 Fish/Sea Serpent/Aqua monster from your Deck, and either add it to your hand or Special Summon it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.thspcost)
    effect::set_cost(d, e1, Some(cost_target(thspcost)));
    // e1:SetTarget(s.thsptg)
    effect::set_target(d, e1, Some(cost_target(thsptg)));
    // e1:SetOperation(s.thspop)
    effect::set_operation(d, e1, Some(operation(thspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e2:SetCost(s.drawcost)
    effect::set_cost(d, e2, Some(cost_target(drawcost)));
    // e2:SetTarget(s.drawtg)
    effect::set_target(d, e2, Some(cost_target(drawtg)));
    // e2:SetOperation(s.drawop)
    effect::set_operation(d, e2, Some(operation(drawop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thspcostfilter(c,e,tp)`
fn thspcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(s.listed_series) and (c:IsFaceup() or c:IsControler(tp))
    //     and Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,Duel.GetMZoneCount(tp,c)>0)
    card::is_set_card(d, c, &[SET_ATLANTEAN as u16, SET_MERMAIL as u16])
        && (card::is_faceup(d, c) || card::is_controler(d, c, tp))
        && {
            let mmz = duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0;
            duel::is_existing_matching_card(d, Some(&|d, cc| thspfilter(d, cc, e, tp, mmz)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
/// `s.thspfilter(c,e,tp,mmz_check)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_check: bool) -> bool {
    // return c:IsLevel(7) and c:IsRace(RACE_FISH|RACE_SEASERPENT|RACE_AQUA)
    //     and (c:IsAbleToHand() or (mmz_check and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_level(d, c, &[7])
        && card::is_race(d, c, RACE_FISH | RACE_SEASERPENT | RACE_AQUA)
        && (card::is_able_to_hand(d, c, None) || (mmz_check && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.thspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsReleasable() and Duel.CheckReleaseGroupCost(tp,s.thspcostfilter,1,true,nil,c,e,tp) end
    if a.chk == Some(0) {
        return card::is_releasable(d, c, None, None)
            && duel::check_release_group_cost(d, a.tp, Some(&|d, cc| thspcostfilter(d, cc, a.e, a.tp)), 1, 1, true, None, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // local rg=Duel.SelectReleaseGroupCost(tp,s.thspcostfilter,1,1,true,nil,c,e,tp)
    let mut rg = duel::select_release_group_cost(d, a.tp, Some(&|d, cc| thspcostfilter(d, cc, a.e, a.tp)), 1, 1, true, None, c.into());
    // rg:AddCard(c)
    group::add_card(&mut rg, &CardSet::from_card(c));
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    true
}
/// `s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local cost_chk=e:GetLabel()==1 or Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let cost_chk = effect::get_label(d, a.e) == 1 || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,cost_chk)
        return duel::is_existing_matching_card(d, Some(&|d, cc| thspfilter(d, cc, a.e, a.tp, cost_chk)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thspop(e,tp,eg,ep,ev,re,r,rp)`
fn thspop(d: &mut Duel, a: &EffArgs) {
    // local mmz_check=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_check = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,mmz_check):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, cc| thspfilter(d, cc, a.e, a.tp, mmz_check)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if sc then
    if let Some(sc) = group::get_first(d, &g) {
        // aux.ToHandOrElse(sc,tp,
        //     function(c) return mmz_check and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
        //     function(c) Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
        //     aux.Stringid(id,3))
        aux::to_hand_or_else(
            d,
            sc,
            a.tp,
            Some(&|d, _c| mmz_check && card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, None, None, None)),
            Some(&|d, _g| duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None)),
            Some(aux::stringid(ID, 3)),
        );
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // --Cannot Special Summon from the Extra Deck for the rest of this turn, except WATER monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e1, aux::stringid(ID, 4));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsAttribute(ATTRIBUTE_WATER) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_attribute(d, c, ATTRIBUTE_WATER))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // --Clock Lizard check
    // aux.addTempLizardCheck(c,tp,function(e,c) return not c:IsOriginalAttribute(ATTRIBUTE_WATER) end)
    aux::add_temp_lizard_check(d, c, a.tp, Some(effect_card_fn(|d, _e, c| !card::is_original_attribute(d, c, ATTRIBUTE_WATER))), None, None, None, None);
}
/// `aux.AND(Card.IsDiscardable,Card.IsAbleToGraveAsCost)`
fn discardable_to_grave(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.drawcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drawcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost()
    //     and Duel.IsExistingMatchingCard(aux.AND(Card.IsDiscardable,Card.IsAbleToGraveAsCost),tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && duel::is_existing_matching_card(d, Some(&discardable_to_grave), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Remove(c,POS_FACEUP,REASON_COST)
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.DiscardHand(tp,aux.AND(Card.IsDiscardable,Card.IsAbleToGraveAsCost),1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&discardable_to_grave), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.drawtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drawtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drawop(e,tp,eg,ep,ev,re,r,rp)`
fn drawop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
