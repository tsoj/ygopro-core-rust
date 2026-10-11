//! Azure Ocean Dragon Lord - Neo-Daedalus Rage
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 41570943;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON, CARD_UMI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 4 monsters
    // Link.AddProcedure(c,nil,4,4)
    link::add_procedure(d, c, None, 4, Some(4), None, None, None);
    // Additional prcedure for "Atlantis, the Dragon City"
    // Link.AddProcedure(c,nil,3,3,nil,aux.Stringid(CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON,0),s.splimit)
    link::add_procedure(d, c, None, 3, Some(3), None, Some(aux::stringid(CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON, 0)), Some(lua_fn(splimit)));
    // If this card is Link Summoned: You can Special Summon up to 3 monsters that mention "Atlantis, the Dragon City" from your GY to your zones this card points to, also you cannot Special Summon for the rest of this turn
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
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This card's name becomes "Umi" while in the Monster Zone
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e2, EFFECT_CHANGE_CODE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(CARD_UMI)
    effect::set_value(d, e2, CARD_UMI as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Once per turn (Quick Effect): You can send 1 face-up "Umi" you control to the GY; send all cards on the field to the GY, except "Umi"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(s.tgcost)
    effect::set_cost(d, e3, Some(cost_target(tgcost)));
    // e3:SetTarget(s.tgtg)
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    // e3:SetOperation(s.tgop)
    effect::set_operation(d, e3, Some(operation(tgop)));
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // if Duel.IsPlayerAffectedByEffect(sp,CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON) then
    if duel::is_player_affected_by_effect(d, a.p(2), CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON).is_some() {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e(0)).nil();
        // c:AssumeProperty(ASSUME_LINK,c:GetLink()-1)
        let link = (card::get_link(d, c) as i64 - 1) as u64;
        card::assume_property(d, c, ASSUME_LINK, link);
        // return true
        return true;
    }
    // return false
    false
}
/// `function(e) return e:GetHandler():IsLinkSummoned() end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
/// `s.spfilter(c,e,tp,zone)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    card::lists_code(d, c, &[CARD_ATLANTIS_CITY_OF_THE_SEA_DRAGON])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
}
/// `s.sptg`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local zone=e:GetHandler():GetFreeLinkedZone()
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_free_linked_zone(d, h, None);
    // if chk==0 then return zone>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,zone) end
    if a.chk == Some(0) {
        return zone > 0 && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, zone)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zone=c:GetFreeLinkedZone()
    let zone = card::get_free_linked_zone(d, c, None);
    // if c:IsRelateToEffect(e) and zone>0 then
    if card::is_relate_to_effect(d, c, a.e) && zone > 0 {
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp,zone)
        let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, zone)), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // local znct=0
        // for i=0,4 do if zone&(1<<i)>0 then znct=znct+1 end end
        let znct = (0..5u32).filter(|i| zone & (1 << i) > 0).count();
        // local ct=math.min(3,#g,znct)
        let mut ct = 3.min(g.len()).min(znct);
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and ct>1 then ct=1 end
        let spirit = duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some();
        if spirit && ct > 1 {
            ct = 1;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,ct,nil)
        let sg = group::select(d, &g, a.tp, 1, ct as u16, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP,zone)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
        }
    }
    // You cannot Special Summon for the rest of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.tgcostfilter(c)`: `c:IsCode(CARD_UMI) and c:IsFaceup() and c:IsAbleToGraveAsCost()`
fn tgcostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_UMI]) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.tgcost`
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgcostfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgcostfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&tgcostfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.tgfilter(c)`: `not (c:IsCode(CARD_UMI) and c:IsFaceup()) and c:IsAbleToGrave()`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    !(card::is_code(d, c, &[CARD_UMI]) && card::is_faceup(d, c)) && card::is_able_to_grave(d, c, None)
}
/// `s.tgtg`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g.clone()), g.len() as u8, a.tp, 0);
    true
}
/// `s.tgop`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if #g>0 then Duel.SendtoGrave(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
