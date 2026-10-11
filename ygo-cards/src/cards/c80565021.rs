//! Angelechy Opening to e4
//! JP name
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 80565021;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANGELECHY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If your opponent goes first, you can also activate this card from your hand during the Standby Phase of their first turn
    // local e4a=Effect.CreateEffect(c)
    let e4a = effect::create_effect(d, c);
    // e4a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4a, EFFECT_TYPE_SINGLE);
    // e4a:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e4a, EFFECT_TRAP_ACT_IN_HAND);
    // e4a:SetCondition(function(e) return Duel.GetTurnCount()==1 and Duel.IsStandbyPhase(1-e:GetHandlerPlayer()) end)
    effect::set_condition(d, e4a, Some(condition(e4a_condition)));
    // c:RegisterEffect(e4a)
    card::register_effect(d, c, e4a, false);
    // Place 1 "Angelechy" Field Spell from your hand, Deck, or GY, face-up on your field, and if you do, Special Summon 1 Level 2 or 7 "Angelechy" monster from your Extra Deck to the Extra Monster Zone, and if you do that, place 1 "Angelechy" monster from your Extra Deck in your Spell & Trap Zone as a face-up Continuous Spell. Until the end of your next turn after this card resolves, you cannot Special Summon from the Extra Deck, except Synchro Monsters
    // local e4b=Effect.CreateEffect(c)
    let e4b = effect::create_effect(d, c);
    // e4b:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4b, aux::stringid(ID, 0));
    // e4b:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4b, CATEGORY_SPECIAL_SUMMON);
    // e4b:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e4b, EFFECT_TYPE_ACTIVATE);
    // e4b:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4b, EVENT_FREE_CHAIN);
    // e4b:SetTarget(s.pltg)
    effect::set_target(d, e4b, Some(cost_target(pltg)));
    // e4b:SetOperation(s.plop)
    effect::set_operation(d, e4b, Some(operation(plop)));
    // e4b:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e4b, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e4b)
    card::register_effect(d, c, e4b, false);
}
// function(e) return Duel.GetTurnCount()==1 and Duel.IsStandbyPhase(1-e:GetHandlerPlayer()) end
fn e4a_condition(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_turn_count(d, None) == 1 && duel::is_standby_phase(d, Some(1 - effect::get_handler_player(d, a.e)))
}
// function s.plfilter(c,field)
fn plfilter(d: &mut Duel, c: CardId, field: bool) -> bool {
    // return c:IsSetCard(SET_ANGELECHY) and not c:IsForbidden() and (not field or c:IsFieldSpell())
    card::is_set_card(d, c, &[SET_ANGELECHY as u16]) && !card::is_forbidden(d, c) && (!field || card::is_field_spell(d, c))
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevel(2,7) and c:IsSetCard(SET_ANGELECHY) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c,ZONES_EMZ)>0 and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_EXTRA,0,1,c)
    card::is_level(d, c, &[2, 7])
        && card::is_set_card(d, c, &[SET_ANGELECHY as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), Some(ZONES_EMZ)).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| plfilter(d, x, false)), tp, LOCATION_EXTRA, 0, 1, c.into())
}
// function s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local stzone_count=Duel.GetLocationCount(tp,LOCATION_SZONE)
        let mut stzone_count = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
        // if e:IsHasType(EFFECT_TYPE_ACTIVATE) and not e:GetHandler():IsLocation(LOCATION_SZONE) then stzone_count=stzone_count-1 end
        if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && !card::is_location(d, effect::get_handler(d, a.e).nil(), LOCATION_SZONE) {
            stzone_count -= 1;
        }
        // return stzone_count>0 and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,true)
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
        return stzone_count > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| plfilter(d, x, true)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.plop(e,tp,eg,ep,ev,re,r,rp)
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.plfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,true):GetFirst()
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, x| plfilter(d, x, true))),
        tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        true,
        NO_EX,
    )
    .nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
        let fc = duel::get_field_card(d, tp, LOCATION_FZONE, 0);
        // if fc then
        if let Some(fc) = fc {
            // Duel.SendtoGrave(fc,REASON_RULE)
            duel::send_to_grave(d, fc, REASON_RULE, None, None);
            // Duel.BreakEffect()
            duel::break_effect(d);
        }
        // if Duel.MoveToField(tc,tp,tp,LOCATION_FZONE,POS_FACEUP,true) then
        if duel::move_to_field(d, tc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
            let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP,ZONES_EMZ)>0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 then
            if !g.is_empty()
                && duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, Some(ZONES_EMZ)) > 0
                && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
                // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_EXTRA,0,1,1,nil):GetFirst()
                let g2 = duel::select_matching_card(d, tp, Some(&|d, x| plfilter(d, x, false)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
                let sc = group::get_first(d, &g2);
                // if sc and Duel.MoveToField(sc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
                if let Some(sc) = sc {
                    if duel::move_to_field(d, sc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
                        // Treated as a Continuous Spell
                        // local e4=Effect.CreateEffect(c)
                        let e4 = effect::create_effect(d, c);
                        // e4:SetType(EFFECT_TYPE_SINGLE)
                        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
                        // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                        effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
                        // e4:SetCode(EFFECT_CHANGE_TYPE)
                        effect::set_code(d, e4, EFFECT_CHANGE_TYPE);
                        // e4:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
                        effect::set_value(d, e4, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
                        // e4:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
                        effect::set_reset(d, e4, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
                        // sc:RegisterEffect(e4)
                        card::register_effect(d, sc, e4, false);
                    }
                }
            }
        }
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local reset_count=Duel.IsTurnPlayer(tp) and 2 or 1
    let reset_count = if duel::is_turn_player(d, tp) { 2 } else { 1 };
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e4:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // e4:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsSynchroMonster() end)
    effect::set_target(d, e4, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_synchro_monster(d, c))));
    // e4:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,reset_count)
    effect::set_reset(d, e4, RESET_PHASE | PHASE_END | RESET_SELF_TURN, reset_count);
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, tp);
}
