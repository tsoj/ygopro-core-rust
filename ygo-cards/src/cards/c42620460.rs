//! Time Thief Power Reserve
//! クロノダイバー・パワーリザーブ
use super::prelude::*;

pub const ID: u32 = 42620460;

// local LOCATION_HAND_DECK_GRAVE=LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE
const LOCATION_HAND_DECK_GRAVE: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TIME_THIEF], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself as a monster then Special Summon 1 Machine "Time Thief" monster
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
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 card on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.rmvcond)
    effect::set_condition(d, e2, Some(condition(rmvcond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.rmvtg)
    effect::set_target(d, e2, Some(cost_target(rmvtg)));
    // e2:SetOperation(s.rmvop)
    effect::set_operation(d, e2, Some(operation(rmvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,id,SET_TIME_THIEF,TYPE_MONSTER|TYPE_NORMAL,1900,2500,4,RACE_PSYCHIC,ATTRIBUTE_DARK)`
fn can_summon_self(d: &mut Duel, tp: u8) -> bool {
    let md = duel::MonsterData {
        setcodes: Some(vec![SET_TIME_THIEF as u16]),
        type_: Some(TYPE_MONSTER | TYPE_NORMAL),
        attack: Some(1900),
        defense: Some(2500),
        level: Some(4),
        race: Some(RACE_PSYCHIC),
        attribute: Some(ATTRIBUTE_DARK),
    };
    duel::is_player_can_special_summon_monster(d, tp, ID, md, Some(POS_FACEUP as u8), Some(tp), 0)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && can_summon_self(d, tp);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND_DECK_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND_DECK_GRAVE as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsSetCard(SET_TIME_THIEF) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_MACHINE) && card::is_set_card(d, c, &[SET_TIME_THIEF as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if !card::is_relate_to_effect(d, c, e)
        || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !can_summon_self(d, tp)
    {
        return;
    }
    // c:AddMonsterAttribute(TYPE_NORMAL|TYPE_TRAP)
    card::add_monster_attribute(d, c, TYPE_NORMAL | TYPE_TRAP, 0, 0, 0, 0, 0);
    // Duel.SpecialSummonStep(c,1,tp,tp,true,false,POS_FACEUP)
    duel::special_summon_step(d, c, 1, tp, tp, true, false, POS_FACEUP as u8, None);
    // c:AddMonsterAttributeComplete()
    card::add_monster_attribute_complete(d, c);
    // if Duel.SpecialSummonComplete()==0 then return end
    if duel::special_summon_complete(d) == 0 {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND_DECK_GRAVE,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp))), tp, LOCATION_HAND_DECK_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND_DECK_GRAVE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp))), tp, LOCATION_HAND_DECK_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.xyzmatfilter(c)`
fn xyzmatfilter(d: &mut Duel, c: CardId) -> bool {
    // if not (c:IsFaceup() and c:IsType(TYPE_XYZ)) then return false end
    if !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)) {
        return false;
    }
    // local og=c:GetOverlayGroup()
    let og = card::get_overlay_group(d, c);
    // return og:IsExists(Card.IsSpell,1,nil) and og:IsExists(Card.IsTrap,1,nil)
    group::is_exists(d, &og, &|d, x| card::is_spell(d, x), 1, NO_EX) && group::is_exists(d, &og, &|d, x| card::is_trap(d, x), 1, NO_EX)
}
/// `s.rmvcond(e,tp,eg,ep,ev,re,r,rp)`
fn rmvcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.xyzmatfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&xyzmatfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, 0, LOCATION_ONFIELD as i32);
    true
}
/// `s.rmvop(e,tp,eg,ep,ev,re,r,rp)`
fn rmvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
