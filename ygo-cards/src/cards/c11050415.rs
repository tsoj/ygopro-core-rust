//! Super Hippo Carnival
use super::prelude::*;

pub const ID: u32 = 11050415;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[41440148, TOKEN_HIPPO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 "Performapal Hip Hippo" from hand, deck, or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(41440148) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[41440148]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let (e, tp) = (a.e, a.tp);
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let (e, tp) = (a.e, a.tp);
    let f = aux::necro_valley_filter(move |d, c| filter(d, c, e, tp));
    let g = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // if Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)==0 then return end
        if duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
            return;
        }
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        // if ft<=0 or (Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and ft>1) then return end
        if ft <= 0 || (duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() && ft > 1) {
            return;
        }
        // if Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_HIPPO,0,TYPES_TOKEN,0,0,1,RACE_BEAST,ATTRIBUTE_EARTH)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_HIPPO, token_data(), None, None, 0)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // for i=1,ft do
            for i in 1..=ft as u32 {
                // local token=Duel.CreateToken(tp,id+i)
                let token = duel::create_token(d, a.tp, ID + i).nil();
                // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
                // Cannot be tributed
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetDescription(3303)
                effect::set_description(d, e1, 3303);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_UNRELEASABLE_SUM)
                effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetValue(1)
                effect::set_value(d, e1, 1);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, (RESET_EVENT | RESETS_STANDARD) as u32, 1);
                // token:RegisterEffect(e1)
                card::register_effect(d, token, e1, false);
                // local e2=e1:Clone()
                let e2 = effect::clone(d, e1);
                // e2:SetCode(EFFECT_UNRELEASABLE_NONSUM)
                effect::set_code(d, e2, EFFECT_UNRELEASABLE_NONSUM);
                // token:RegisterEffect(e2)
                card::register_effect(d, token, e2, false);
                // Cannot special summon from extra deck while you control this token
                // local e3=Effect.CreateEffect(c)
                let e3 = effect::create_effect(d, c);
                // e3:SetType(EFFECT_TYPE_FIELD)
                effect::set_type(d, e3, EFFECT_TYPE_FIELD);
                // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
                effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
                // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
                effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
                // e3:SetRange(LOCATION_MZONE)
                effect::set_range(d, e3, LOCATION_MZONE);
                // e3:SetAbsoluteRange(tp,1,0)
                effect::set_absolute_range(d, e3, a.tp, 1, 0);
                // e3:SetTarget(s.splimit)
                effect::set_target(d, e3, Some(effect_card_fn(splimit)));
                // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e3, (RESET_EVENT | RESETS_STANDARD) as u32, 1);
                // token:RegisterEffect(e3)
                card::register_effect(d, token, e3, false);
                // Clock Lizard check
                // local e4=aux.createContinuousLizardCheck(e:GetHandler(),LOCATION_MZONE)
                let e4 = aux::create_continuous_lizard_check(d, c, LOCATION_MZONE, None, None, None);
                // e4:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e4, (RESET_EVENT | RESETS_STANDARD) as u32, 1);
                // token:RegisterEffect(e4,true)
                card::register_effect(d, token, e4, true);
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
            effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
            // e1:SetTargetRange(0,LOCATION_MZONE)
            effect::set_target_range(d, e1, 0, LOCATION_MZONE);
            // e1:SetValue(s.atlimit)
            effect::set_value_fn(d, e1, effect_card_fn(atlimit));
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, (RESET_PHASE | PHASE_END) as u32, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
        }
    }
}
/// The token data of `TYPES_TOKEN, 0, 0, 1, RACE_BEAST, ATTRIBUTE_EARTH`.
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_BEAST),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
// function s.splimit(e,c)
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_EXTRA)
    card::is_location(d, c, LOCATION_EXTRA)
}
// function s.atlimit(e,c)
fn atlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsCode(TOKEN_HIPPO)
    !card::is_code(d, c, &[TOKEN_HIPPO])
}
