//! Shinobaroness Peacock
use super::prelude::*;

pub const ID: u32 = 25415052;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[73055622, TOKEN_SHINOBIRD],
    listed_card_types: &[TYPE_SPIRIT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // local sme,soe=Spirit.AddProcedure(c,EVENT_SPSUMMON_SUCCESS)
    let procs = spirit::add_procedure(d, c, &[EVENT_SPSUMMON_SUCCESS]);
    let (sme, soe) = (procs[0], procs[1]);
    // Mandatory return
    // sme:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, sme, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // sme:SetTarget(s.mrettg)
    effect::set_target(d, sme, Some(cost_target(mrettg)));
    // sme:SetOperation(s.retop)
    effect::set_operation(d, sme, Some(operation(retop)));
    // Optional return
    // soe:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, soe, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // soe:SetTarget(s.orettg)
    effect::set_target(d, soe, Some(cost_target(orettg)));
    // soe:SetOperation(s.retop)
    effect::set_operation(d, soe, Some(operation(retop)));
    // Must be Ritual Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.ritlimit)
    effect::set_value_fn(d, e1, aux::ritlimit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle up to 3 opponent Spell/Trap cards to the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.tdcon)
    effect::set_condition(d, e2, Some(condition(tdcon)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.tdcon(e,tp,eg,ep,ev,re,r,rp)
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsRitualSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_ritual_summoned(d, c)
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToDeck()
    card::is_spell_trap(d, c) && card::is_able_to_deck(d, c)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_SPIRIT) and c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_type(d, c, TYPE_SPIRIT) && card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tdfilter,tp,0,LOCATION_ONFIELD,1,3,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&tdfilter), a.tp, 0, LOCATION_ONFIELD, 1, 3, false, NO_EX).nil();
    // if #g==0 or Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0
    if g.len() == 0 {
        return;
    }
    if duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // or not Duel.GetOperatedGroup():IsExists(Card.IsLocation,1,nil,LOCATION_DECK)
    let operated = duel::get_operated_group(d);
    if !group::is_exists(d, &operated, &|d, x| card::is_location(d, x, LOCATION_DECK), 1, NO_EX) {
        return;
    }
    // or Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let (e, tp) = (a.e, a.tp);
    let sg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if #sg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
    if sg.len() == 0 || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local ssg=sg:Select(tp,1,1,nil)
    let ssg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
    // if #ssg>0 then
    if ssg.len() > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(ssg,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, ssg, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
// the eight effect arguments followed by chk
fn lua_args_chk(a: &EffArgs, chk: i64) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        Value::Int(chk),
    ]
}
// function s.mrettg(e,tp,eg,ep,ev,re,r,rp,chk)
fn mrettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Spirit.MandatoryReturnTarget(e,tp,eg,ep,ev,re,r,rp,1)
    spirit::mandatory_return_target_fn().call(d, &lua_args_chk(a, 1));
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, 0);
    true
}
// function s.tkcheck(e,tp)
fn tkcheck(d: &mut Duel, e: EffectId, tp: u8) -> bool {
    // return Duel.GetMZoneCount(tp,e:GetHandler())>1 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_SHINOBIRD,0,TYPES_TOKEN,1500,1500,4,RACE_WINGEDBEAST,ATTRIBUTE_WIND)
    let h = effect::get_handler(d, e).nil();
    duel::get_m_zone_count(d, tp, h.into(), None, None, None).0 > 1
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::is_player_can_special_summon_monster(
            d,
            tp,
            TOKEN_SHINOBIRD,
            duel::MonsterData {
                setcodes: Some(vec![0]),
                type_: Some(TYPES_TOKEN),
                attack: Some(1500),
                defense: Some(1500),
                level: Some(4),
                race: Some(RACE_WINGEDBEAST),
                attribute: Some(ATTRIBUTE_WIND),
            },
            None,
            None,
            0,
        )
}
// function s.orettg(e,tp,eg,ep,ev,re,r,rp,chk)
fn orettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Spirit.OptionalReturnTarget(e,tp,eg,ep,ev,re,r,rp,0) and s.tkcheck(e,tp) end
    if a.chk == Some(0) {
        return spirit::optional_return_target_fn().call(d, &lua_args_chk(a, 0)).truthy() && tkcheck(d, a.e, a.tp);
    }
    // Spirit.OptionalReturnTarget(e,tp,eg,ep,ev,re,r,rp,1)
    spirit::optional_return_target_fn().call(d, &lua_args_chk(a, 1));
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, 0);
    true
}
// function s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoHand(c,nil,REASON_EFFECT)>0
    //     and c:IsLocation(LOCATION_HAND) and s.tkcheck(e,tp) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_HAND)
        && tkcheck(d, a.e, a.tp)
    {
        // for _=1,2 do
        for _ in 0..2 {
            // local token=Duel.CreateToken(tp,TOKEN_SHINOBIRD)
            let token = duel::create_token(d, a.tp, TOKEN_SHINOBIRD).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
