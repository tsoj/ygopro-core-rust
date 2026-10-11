//! The Flute of Guiding Dragon
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 93871599;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If a Spellcaster monster is on the field: Special Summon 1 Level 5 or higher Dragon monster from your Deck, ...
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
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),0,LOCATION_MZONE,LOCATION_MZONE,1,nil) end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your Main Phase 2, if this card is in your GY because it was sent there this turn: You can banish it; draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) local c=e:GetHandler() return Duel.IsMainPhase2() and c:GetTurnID()==Duel.GetTurnCount() and not c:IsReason(REASON_RETURN) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// Condition of e1: function() return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),0,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
fn cond1(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_SPELLCASTER))),
        0,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        NO_EX,
    )
}
// Condition of e2: function(e) local c=e:GetHandler() return Duel.IsMainPhase2() and c:GetTurnID()==Duel.GetTurnCount() and not c:IsReason(REASON_RETURN) end
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    duel::is_main_phase2(d, None)
        && card::get_turn_id(d, c) as i32 == duel::get_turn_count(d, None)
        && !card::is_reason(d, c, REASON_RETURN)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelAbove(5) and c:IsRace(RACE_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_above(d, c, 5) && card::is_race(d, c, RACE_DRAGON) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.samenamespfilter(c,e,tp,code)
fn samenamespfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>=2 then
        if let Some(sc) = sc {
            if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
                && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) >= 2
            {
                // local code=sc:GetCode()
                let code = card::get_code(d, sc)[0];
                // if Duel.IsExistingMatchingCard(s.samenamespfilter,tp,LOCATION_DECK,0,1,nil,e,tp,code)
                //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
                if duel::is_existing_matching_card(d, Some(&|d, c| samenamespfilter(d, c, e, tp, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
                    && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
                {
                    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                    // local g=Duel.SelectMatchingCard(tp,s.samenamespfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,code)
                    let g = duel::select_matching_card(d, tp, Some(&|d, c| samenamespfilter(d, c, e, tp, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
                    // if #g>0 then
                    if !g.is_empty() {
                        // Duel.BreakEffect()
                        duel::break_effect(d);
                        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                        duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                    }
                }
            }
        }
    }
    // You cannot Special Summon for the rest of this turn, except Dragon or Spellcaster monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsRaceExcept(RACE_DRAGON|RACE_SPELLCASTER) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race_except(d, c, RACE_DRAGON | RACE_SPELLCASTER, None, 0, None))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
