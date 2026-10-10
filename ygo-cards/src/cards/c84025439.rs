//! Super Quantal Mech King Great Magnus
use super::prelude::*;

pub const ID: u32 = 84025439;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_SUPER_QUANT,SET_SUPER_QUANTAL_MECH_BEAST}
    listed_series: &[SET_SUPER_QUANT, SET_SUPER_QUANTAL_MECH_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 12 monsters
    // Xyz.AddProcedure(c,nil,12,3)
    xyz::add_procedure(d, c, None, Some(12), 3, None, None, None, None, false, None);
    // If this card is sent to the GY: You can Special Summon 3 "Super Quantal Mech Beast" Xyz Monsters with different names from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This card gains these effects, based on the number of materials with different names attached to it
    // ● 2+: Once per turn, during the Main Phase (Quick Effect): You can detach 1 material from this card; shuffle 1 card on the field into the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(aux.AND(function() return Duel.IsMainPhase() end, s.effcon(2)))
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_main_phase(d, None) && effcon(d, a.e, 2))));
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // ● 4+: It is unaffected by card effects, except "Super Quant" cards
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.effcon(4))
    effect::set_condition(d, e3, Some(condition(|d, a| effcon(d, a.e, 4))));
    // e3:SetValue(function(e,te) return not te:IsCardSetcode(SET_SUPER_QUANT) end)
    effect::set_value_fn(d, e3, lua_fn(|d, a: Args| !effect::is_card_setcode(d, a.e(1), &[SET_SUPER_QUANT as u16])));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // ● 6+: Your opponent cannot add cards from the Deck to the hand by card effects
    // local e4a=Effect.CreateEffect(c)
    let e4a = effect::create_effect(d, c);
    // e4a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4a, EFFECT_TYPE_FIELD);
    // e4a:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4a, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4a:SetCode(EFFECT_CANNOT_TO_HAND)
    effect::set_code(d, e4a, EFFECT_CANNOT_TO_HAND);
    // e4a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4a, LOCATION_MZONE);
    // e4a:SetTargetRange(0,1)
    effect::set_target_range(d, e4a, 0, 1);
    // e4a:SetCondition(s.effcon(6))
    effect::set_condition(d, e4a, Some(condition(|d, a| effcon(d, a.e, 6))));
    // e4a:SetTarget(function(e,c,rp,re) return c:IsLocation(LOCATION_DECK) end)
    effect::set_target(d, e4a, Some(lua_fn(|d, a: Args| card::is_location(d, a.c(1), LOCATION_DECK))));
    // c:RegisterEffect(e4a)
    card::register_effect(d, c, e4a, false);
    // local e4b=e4a:Clone()
    let e4b = effect::clone(d, e4a);
    // e4b:SetCode(EFFECT_CANNOT_DRAW)
    effect::set_code(d, e4b, EFFECT_CANNOT_DRAW);
    // c:RegisterEffect(e4b)
    card::register_effect(d, c, e4b, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SUPER_QUANTAL_MECH_BEAST) and c:IsXyzMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_SUPER_QUANTAL_MECH_BEAST as u16])
        && card::is_xyz_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<3 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return false end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 3
            || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
        {
            return false;
        }
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
        // return #g>=3 and g:GetClassCount(Card.GetCode)>=3
        return g.len() >= 3 && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,3,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<3 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 3
        || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g>=3 and g:GetClassCount(Card.GetCode)>=3 then
    if g.len() >= 3 && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3 {
        // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
        let sg = aux::select_unselect_group(
            d,
            &g,
            Some(e),
            tp,
            Some(3),
            Some(3),
            Some(&aux::dncheck()),
            1,
            tp,
            Some(HINTMSG_SPSUMMON as u64),
            None,
            None,
            false,
        )
        .unwrap_or_default();
        // if #sg==3 then
        if sg.len() == 3 {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.effcon(material_count)`: the returned function checks the number of different-named materials
fn effcon(d: &mut Duel, e: EffectId, material_count: usize) -> bool {
    // return e:GetHandler():GetOverlayGroup():GetClassCount(Card.GetCode)>=material_count
    let h = effect::get_handler(d, e).nil();
    let og = card::get_overlay_group(d, h);
    group::get_class_count(d, &og, &|d, c| card::get_code(d, c)[0] as i64) >= material_count
}
/// `s.tdtg(e,tp,...,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_EITHER,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}
/// `s.tdop(e,tp,...)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&card::is_able_to_deck), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
