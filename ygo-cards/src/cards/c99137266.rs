//! Swordsoul Strife
use super::prelude::*;

pub const ID: u32 = 99137266;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_SWORDSOUL], listed_series: &[SET_SWORDSOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Banish
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_REMOVE);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    effect::set_operation(d, e1, Some(operation(rmop)));
    card::register_effect(d, c, e1, false);
    // Special Summon token
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_REMOVE);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(tktg)));
    effect::set_operation(d, e2, Some(operation(tkop)));
    card::register_effect(d, c, e2, false);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_SWORDSOUL) or (c:IsRace(RACE_WYRM) and c:IsType(TYPE_SYNCHRO)))
    //     and c:IsAbleToRemove() and (c:IsFaceup() or not c:IsLocation(LOCATION_ONFIELD))
    (card::is_set_card(d, c, &[SET_SWORDSOUL as u16]) || (card::is_race(d, c, RACE_WYRM) && card::is_type(d, c, TYPE_SYNCHRO)))
        && card::is_able_to_remove(d, c, None, None, None)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_ONFIELD))
}
/// `s.rmtg(e,...,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local loc=LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE|LOCATION_ONFIELD
    let loc = LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE | LOCATION_ONFIELD;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,loc,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmfilter), a.tp, loc, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,0,loc)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 0, loc as i32);
    true
}
/// `s.rmop(e,...)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local loc=LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE|LOCATION_ONFIELD
    let loc = LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE | LOCATION_ONFIELD;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rmfilter,tp,loc,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&rmfilter), a.tp, loc, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // if g:GetFirst():IsLocation(LOCATION_ONFIELD) then
        let first = group::get_first(d, &g).nil();
        if card::is_location(d, first, LOCATION_ONFIELD) {
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
        }
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// The chk==0 part of `s.tktg`, shared with `s.tkop` (`s.tktg(e,...,0)`).
fn tktg_check(d: &mut Duel, tp: u8) -> bool {
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_SWORDSOUL,SET_SWORDSOUL,TYPES_TOKEN+TYPE_TUNER,0,0,4,RACE_WYRM,ATTRIBUTE_WATER)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, TOKEN_SWORDSOUL, token_data(), None, None, 0)
}
/// The token's stats of `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_SWORDSOUL,SET_SWORDSOUL,TYPES_TOKEN+TYPE_TUNER,0,0,4,RACE_WYRM,ATTRIBUTE_WATER)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_SWORDSOUL as u16]),
        type_: Some(TYPES_TOKEN | TYPE_TUNER),
        attack: Some(0),
        defense: Some(0),
        level: Some(4),
        race: Some(RACE_WYRM),
        attribute: Some(ATTRIBUTE_WATER),
    }
}
/// `s.tktg(e,...,chk)`
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return ... end
    if a.chk == Some(0) {
        return tktg_check(d, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.tkop(e,...)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    // if s.tktg(e,tp,eg,ep,ev,re,r,rp,0) then
    if tktg_check(d, a.tp) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local token=Duel.CreateToken(tp,TOKEN_SWORDSOUL)
        let token = duel::create_token(d, a.tp, TOKEN_SWORDSOUL).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // Cannot Special Summon non-Synchro monsters from Extra Deck
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetAbsoluteRange(tp,1,0)
        effect::set_absolute_range(d, e1, a.tp, 1, 0);
        // e1:SetTarget(function(_,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_SYNCHRO) end)
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_SYNCHRO))));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // token:RegisterEffect(e1,true)
        card::register_effect(d, token, e1, true);
        // Lizard check
        // local e2=aux.createContinuousLizardCheck(c,LOCATION_MZONE,function(_,c) return c:IsOriginalType(TYPE_SYNCHRO) end)
        let e2 = aux::create_continuous_lizard_check(d, c, LOCATION_MZONE, Some(effect_card_fn(|d, _e, c| card::is_original_type(d, c, TYPE_SYNCHRO))), None, None);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // token:RegisterEffect(e2,true)
        card::register_effect(d, token, e2, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
