//! Levia-Dragoon - Daedalus
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 78579058;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_UMI, CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 3 WATER monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WATER),3,3)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))), 3, Some(3), None, None, None);
    // Additional prcedure for "Atlantis, the Dragon City"
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WATER),2,2,nil,aux.Stringid(CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON,0),s.splimit)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        Some(aux::stringid(CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON, 0)),
        Some(lua_fn(|d, a| splimit(d, &a))),
    );
    // This card's name becomes "Umi" while in the Monster Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(CARD_UMI)
    effect::set_value(d, e1, CARD_UMI as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Link Summoned: You can Special Summon 1 monster that mentions "Atlantis, the Dragon City" from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_link_summoned(d, h)
    })));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // During the Main Phase (Quick Effect): You can send 1 face-up "Umi" you control to the GY; discard your opponent's entire hand, and if you do, they draw the same number of cards. Neither player can activate cards or effects in response to this effect's activation
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_HANDES+CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_HANDES + CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e3, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e3:SetCost(s.discardcost)
    effect::set_cost(d, e3, Some(cost_target(discardcost)));
    // e3:SetTarget(s.discardtg)
    effect::set_target(d, e3, Some(cost_target(discardtg)));
    // e3:SetOperation(s.discardop)
    effect::set_operation(d, e3, Some(operation(discardop)));
    // e3:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e3, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER) as u32));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: &Args) -> bool {
    // if Duel.IsPlayerAffectedByEffect(sp,CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON) then
    if duel::is_player_affected_by_effect(d, a.p(2), CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON).is_some() {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e(0)).nil();
        // c:AssumeProperty(ASSUME_LINK,c:GetLink()-1)
        let link_count = card::get_link(d, c) as i64 - 1;
        card::assume_property(d, c, ASSUME_LINK, link_count as u64);
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:ListsCode(CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::lists_code(d, c, &[CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON as u32]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        let tp = a.tp;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let e = a.e;
    let tp = a.tp;
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.discardcostfilter(c)`
fn discardcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_UMI) and c:IsFaceup() and c:IsAbleToGraveAsCost()
    card::is_code(d, c, &[CARD_UMI]) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.discardcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discardcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.discardcostfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&discardcostfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.discardcostfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&discardcostfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.discardtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discardtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    // local ct=#g
    let ct = g.len();
    // if chk==0 then return ct>0 and Duel.IsPlayerCanDraw(1-tp,ct) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_draw(d, 1 - a.tp, ct as u32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,g,ct,1-tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, Some(g), ct as u8, 1 - a.tp, ct as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - a.tp, ct as i32);
    // Duel.SetChainLimit(aux.FALSE)
    duel::set_chain_limit(d, aux_false());
    true
}
/// `s.discardop(e,tp,eg,ep,ev,re,r,rp)`
fn discardop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    // if #g>0 then
    if !g.is_empty() {
        // local ct=Duel.SendtoGrave(g,REASON_DISCARD|REASON_EFFECT)
        let ct = duel::send_to_grave(d, g, REASON_DISCARD | REASON_EFFECT, None, None);
        // if ct==0 then return end
        if ct == 0 {
            return;
        }
        // Duel.Draw(1-tp,ct,REASON_EFFECT)
        duel::draw(d, 1 - a.tp, ct as u32, REASON_EFFECT);
    }
}
