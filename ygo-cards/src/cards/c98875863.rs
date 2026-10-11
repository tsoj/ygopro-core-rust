//! G Golem Rock Hammer
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 98875863;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID + 1], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reduce this card's Level by 2
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.lvcost)
    effect::set_cost(d, e1, Some(cost_target(lvcost)));
    // e1:SetTarget(s.lvtg)
    effect::set_target(d, e1, Some(cost_target(lvtg)));
    // e1:SetOperation(s.lvop)
    effect::set_operation(d, e1, Some(operation(lvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Tribute this card to Special Summon 3 "G Golem Tokens"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.tktg)
    effect::set_target(d, e2, Some(cost_target(tktg)));
    // e2:SetOperation(s.tkop)
    effect::set_operation(d, e2, Some(operation(tkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:IsDiscardable()
    card::is_race(d, c, RACE_CYBERSE) && card::is_discardable(d, c, None, None)
}
// function s.lvcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn lvcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD,c)
    duel::discard_hand(d, a.tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, c.into());
    true
}
// function s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:HasLevel() and c:IsLevelAbove(3) end
    if a.chk == Some(0) {
        return card::has_level(d, c) && card::is_level_above(d, c, 3);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,c,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(CardSet::from_card(c)), 2, 0, 0);
    true
}
// function s.lvop(e,tp,eg,ep,ev,re,r,rp)
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Reduce Level by 2
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_LEVEL)
    effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
    // e1:SetValue(-2)
    effect::set_value(d, e1, -2);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TOFIELD))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetMZoneCount(tp,e:GetHandler())>=3
        //   and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_G_GOLEM,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_EARTH,POS_FACEUP_DEFENSE) end
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 >= 3
            && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP_DEFENSE as u8), None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,3,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 3, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,3,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, tp, 0);
    true
}
// the token "G Golem Token": Duel.IsPlayerCanSpecialSummonMonster(...,id+1,SET_G_GOLEM,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_EARTH,...)
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_G_GOLEM as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_CYBERSE),
        attribute: Some(ATTRIBUTE_EARTH),
        ..Default::default()
    }
}
// function s.tkop(e,tp,eg,ep,ev,re,r,rp)
fn tkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(tp,LOCATION_MZONE)>2
    //   and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 2
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP_DEFENSE as u8), None, 0)
    {
        // for i=1,3 do
        for _ in 0..3 {
            // local token=Duel.CreateToken(tp,id+1)
            let token = duel::create_token(d, tp, ID + 1).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
    // Cannot Special Summon non-Cyberse monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function s.splimit(e,c)
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsRace(RACE_CYBERSE)
    !card::is_race(d, c, RACE_CYBERSE)
}
