//! Elfnote June Pride
use super::prelude::*;

pub const ID: u32 = 5559570;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELFNOTE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1 Spellcaster non-Tuner
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(Card.IsRace,RACE_SPELLCASTER),1,1)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner(Some(std::sync::Arc::new(|d: &mut Duel, cc: CardId| card::is_race(d, cc, RACE_SPELLCASTER as u64))))),
        1,
        1,
        None,
        None,
        None,
        None,
        None,
    );
    // While this card is in the center Main Monster Zone, neither player can declare attacks, except with monsters in the center Main Monster Zones
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetCondition(function(e) return e:GetHandler():IsSequence(2) end)
    effect::set_condition(d, e1, Some(condition(e1_condition)));
    // e1:SetTarget(function(e,c) return not c:IsSequence(2) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, cc| !card::is_sequence(d, cc, &[2]))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During the Main Phase, if this card is in the center Main Monster Zone (Quick Effect): You can return it to the Extra Deck, ...
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return Duel.IsMainPhase() and e:GetHandler():IsSequence(2) end)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsSequence(2) end` (e1 condition)
fn e1_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_sequence(d, h, &[2])
}
/// `function(e) return Duel.IsMainPhase() and e:GetHandler():IsSequence(2) end` (e2 condition)
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    duel::is_main_phase(d, None) && card::is_sequence(d, h, &[2])
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ELFNOTE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ELFNOTE as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return c:IsAbleToExtra() and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return card::is_able_to_extra(d, c)
            && duel::get_m_zone_count(d, tp, (&CardSet::from_card(c)).into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, cc| spfilter(d, cc, e, tp)),
                tp,
                LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and c:IsLocation(LOCATION_EXTRA) then
    if card::is_relate_to_effect(d, c, e)
        && duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_EXTRA)
    {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
        // ft=math.min(ft,#g,3)
        ft = ft.min(g.len() as i64).min(3);
        // if ft<=0 then return end
        if ft <= 0 {
            return;
        }
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,aux.dpcheck(Card.GetLocation),1,tp,HINTMSG_SPSUMMON)
        let sg = aux::select_unselect_group(
            d,
            &g,
            Some(e),
            tp,
            Some(1),
            Some(ft as usize),
            Some(&aux::dpcheck(|d, cc| card::get_location(d, cc) as i64)),
            1,
            tp,
            Some(HINTMSG_SPSUMMON as u64),
            None,
            None,
            false,
        )
        .unwrap_or_default();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
