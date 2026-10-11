//! Cluster Congester
use super::prelude::*;

pub const ID: u32 = 94703021;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Congester Token"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.tkcon1)
    effect::set_condition(d, e1, Some(condition(tkcon1)));
    // e1:SetTarget(s.tktg1)
    effect::set_target(d, e1, Some(cost_target(tktg1)));
    // e1:SetOperation(s.tkop1)
    effect::set_operation(d, e1, Some(operation(tkop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon "Congester Tokens" up to the number of Link monsters the opponent controls
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetHintTiming(0,TIMING_BATTLE_PHASE)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_BATTLE_PHASE));
    // e3:SetCondition(s.tkcon2)
    effect::set_condition(d, e3, Some(condition(tkcon2)));
    // e3:SetCost(s.tkcost2)
    effect::set_cost(d, e3, Some(cost_target(tkcost2)));
    // e3:SetTarget(s.tktg2)
    effect::set_target(d, e3, Some(cost_target(tktg2)));
    // e3:SetOperation(s.tkop2)
    effect::set_operation(d, e3, Some(operation(tkop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.listed_names={94703022}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[94703022], ..ScriptInfo::DEFAULT };

/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_DARK)`
fn can_summon_token(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID + 1,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(0),
            defense: Some(0),
            level: Some(1),
            race: Some(RACE_CYBERSE),
            attribute: Some(ATTRIBUTE_DARK),
        },
        None,
        None,
        0,
    )
}
/// `s.tkcon1(e,tp,eg,ep,ev,re,r,rp)`
fn tkcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroupCount(Card.IsLinkMonster,tp,LOCATION_MZONE,0,nil)==0
    duel::get_matching_group_count(d, Some(&|d, c| card::is_link_monster(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX) == 0
}
/// `s.tktg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tktg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon_token(d, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    true
}
/// `s.tkop1(e,tp,eg,ep,ev,re,r,rp)`
fn tkop1(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_DARK) then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon_token(d, a.tp) {
        // local tk=Duel.CreateToken(tp,id+1)
        let tk = duel::create_token(d, a.tp, ID + 1).nil();
        // Duel.SpecialSummon(tk,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tk, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tkcon2(e,tp,eg,ep,ev,re,r,rp)`
fn tkcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // local g=Duel.GetMatchingGroup(Card.IsLinkMonster,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_link_monster(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // return at and g:IsContains(at)
    at.is_some_and(|at| g.contains(at))
}
/// `s.tkcost2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tkcost2(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if chk==0 then return e:GetHandler():IsAbleToRemoveAsCost() and at and at:IsAbleToRemoveAsCost() and Duel.GetMZoneCount(tp,at)>0 end
    if a.chk == Some(0) {
        let handler = effect::get_handler(d, a.e).nil();
        if !card::is_able_to_remove_as_cost(d, handler, None) {
            return false;
        }
        return match at {
            Some(at) => {
                card::is_able_to_remove_as_cost(d, at, None) && duel::get_m_zone_count(d, a.tp, at.into(), None, None, None).0 > 0
            }
            None => false,
        };
    }
    // local g=Group.FromCards(e:GetHandler(),at)
    let handler = effect::get_handler(d, a.e).nil();
    let mut g = CardSet::from_card(handler);
    if let Some(at) = at {
        g.insert(at);
    }
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.tktg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tktg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsLinkMonster,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_link_monster(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX) && can_summon_token(d, a.tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    true
}
/// `s.tkop2(e,tp,eg,ep,ev,re,r,rp)`
fn tkop2(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_DARK) then return end
    if !can_summon_token(d, a.tp) {
        return;
    }
    // local ct=math.min(Duel.GetMatchingGroupCount(Card.IsLinkMonster,tp,0,LOCATION_MZONE,nil),Duel.GetLocationCount(tp,LOCATION_MZONE))
    let links = duel::get_matching_group_count(d, Some(&|d, c| card::is_link_monster(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX) as i64;
    let mzone = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    let mut ct = links.min(mzone);
    // if ct<1 then return end
    if ct < 1 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // if ct>1 then
    if ct > 1 {
        // ct=Duel.AnnounceNumberRange(tp,1,ct)
        ct = duel::announce_number_range(d, a.tp, Some(1), Some(ct as u32), &[]).nil().0 as i64;
    }
    // for i=1,ct do
    for _ in 0..ct {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, a.tp, ID + 1).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
