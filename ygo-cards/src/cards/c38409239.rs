//! Ghoti Cosmos
//! ゴーティス・コスモス
use super::prelude::*;

pub const ID: u32 = 38409239;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effects in sequence
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// The number of face-up Fish monsters in the banished zone (`Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_FISH),tp,LOCATION_REMOVED,LOCATION_REMOVED,nil)`).
fn fish_removed_count(d: &mut Duel, tp: u8) -> usize {
    duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_FISH))), tp, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(...)
    let ct = fish_removed_count(d, a.tp);
    // if chk==0 then return ct>0 end
    if a.chk == Some(0) {
        return ct > 0;
    }
    // if ct>=8 then
    if ct >= 8 {
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,c,nil,REASON_SYNCHRO)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, Some(c), None, REASON_SYNCHRO);
    // return #pg<=0 and c:IsType(TYPE_SYNCHRO) and c:IsRace(RACE_FISH)
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    pg.is_empty()
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_race(d, c, RACE_FISH)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO as u32, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetMatchingGroupCount(...)
    let ct = fish_removed_count(d, a.tp);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local breakc=false
    let mut breakc = false;
    // if ct>=1 then
    if ct >= 1 {
        // Your Fish monsters cannot be destroyed by battle
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        // e1:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_FISH))
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_FISH))));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,1))
        aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
        breakc = true;
    }
    // if ct>=4 then
    if ct >= 4 {
        // if breakc then Duel.BreakEffect() end
        if breakc {
            duel::break_effect(d);
        }
        // The activation and effects of your Fish monsters on your field cannot be negated
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_INACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_INACTIVATE);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetValue(s.effectfilter)
        effect::set_value_fn(d, e1, lua_fn(effectfilter));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CANNOT_DISEFFECT)
        effect::set_code(d, e2, EFFECT_CANNOT_DISEFFECT);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
        breakc = true;
    }
    // if ct>=8 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) then
    if ct >= 8 && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX) {
        // Special Summon 1 Fish Synchro monster from the Extra Deck
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if sc then
        if let Some(sc) = group::get_first(d, &g) {
            // if breakc then Duel.BreakEffect() end
            if breakc {
                duel::break_effect(d);
            }
            // if Duel.SpecialSummon(sc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)>0 then
            if duel::special_summon(d, sc, SUMMON_TYPE_SYNCHRO, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
                // sc:CompleteProcedure()
                card::complete_procedure(d, sc);
            }
        }
    }
}
/// `s.effectfilter(e,ct)`
fn effectfilter(d: &mut Duel, a: Args) -> bool {
    // local p=e:GetHandlerPlayer()
    let p = effect::get_handler_player(d, a.e(0));
    let ct = a.int(1) as u8;
    // local tp,loc,race=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_PLAYER,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_RACE)
    let info = duel::get_chain_info(d, ct, &[CHAININFO_TRIGGERING_PLAYER, CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_RACE]);
    let tp = info[0].int();
    let loc = info[1].int();
    let race = info[2].int();
    // return p==tp and (loc&LOCATION_ONFIELD)~=0 and race&RACE_FISH>0
    p as i64 == tp && (loc & LOCATION_ONFIELD as i64) != 0 && (race & RACE_FISH as i64) > 0
}
