//! Bloom Harmonist the Melodious Composer
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 34974462;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MELODIOUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_FAIRY),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_FAIRY, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // Special Summon 2 "Melodious" monsters with different Levels from your Deck to your zones this card points to
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
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_link_summoned(d, h)
    })));
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent cannot activate cards or effects until the end of the Damage Step if a "Melodious" this card points to attacks
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCondition(s.actcon)
    effect::set_condition(d, e2, Some(condition(actcon)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,aux.FilterBoolFunction(Card.IsSetCard,SET_MELODIOUS))
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(|d, a: Args| card::is_set_card(d, a.c(0), &[SET_MELODIOUS as u16])));
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil)
    //     and Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0;
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    // Cannot Special Summon monsters, except "Melodious" monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsSetCard(SET_MELODIOUS) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_MELODIOUS as u16]))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
/// `s.spfilter(c,e,tp,zone)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsSetCard(SET_MELODIOUS) and c:HasLevel() and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE,tp,zone)
    card::is_set_card(d, c, &[SET_MELODIOUS as u16])
        && card::has_level(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(tp), Some(zone))
}
/// `aux.dpcheck(Card.GetLevel)`: the Level of a card as a number
fn level_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_level(d, c) as i64
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local zone=e:GetHandler():GetLinkedZone(tp)&ZONES_MMZ
        let h = effect::get_handler(d, e).nil();
        let zone = card::get_linked_zone(d, h, Some(tp)) & ZONES_MMZ;
        // local ct=Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)
        let ct = duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0;
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp,zone)
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp, zone)), tp, LOCATION_DECK, 0, NO_EX);
        // return ct>1 and #g>=2 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetLevel),0)
        if !(ct > 1 && g.len() >= 2 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()) {
            return false;
        }
        let dp = aux::dpcheck(level_value);
        return matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&dp), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local zone=c:GetLinkedZone(tp)&ZONES_MMZ
    let zone = card::get_linked_zone(d, c, Some(tp)) & ZONES_MMZ;
    // local ct=Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)
    let ct = duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0;
    // if zone==0 or ct<2 then return end
    if zone == 0 || ct < 2 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp,zone)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp, zone)), tp, LOCATION_DECK, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetLevel),1,tp,HINTMSG_SPSUMMON)
    let dp = aux::dpcheck(level_value);
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&dp), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP_DEFENSE,zone)
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, Some(zone));
    }
}
/// `s.actcon(e)`: `local ac=Duel.GetAttacker() return ac and ac:IsSetCard(SET_MELODIOUS) and e:GetHandler():GetLinkedGroup():IsContains(ac)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    match duel::get_attacker(d) {
        None => false,
        Some(ac) => {
            if !card::is_set_card(d, ac, &[SET_MELODIOUS as u16]) {
                return false;
            }
            let h = effect::get_handler(d, a.e).nil();
            let linked = card::get_linked_group(d, h);
            group::is_contains(&linked, ac)
        }
    }
}
