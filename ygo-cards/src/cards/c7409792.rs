//! Dynatherium
use super::prelude::*;

pub const ID: u32 = 7409792;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon when summoned
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+1)
    let h = effect::get_handler(d, a.e).nil();
    card::is_summon_type(d, h, &[SUMMON_TYPE_SPECIAL + 1])
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_EITHER,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_EITHER as u8, LOCATION_GRAVE as i32);
    true
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevel(4) and c:IsCanBeSpecialSummoned(e,0,1-tp,false,false)
    card::is_level(d, c, &[4]) && card::is_can_be_special_summoned(d, c, e, 0, 1 - tp, false, false, None, None, None)
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filter),0,LOCATION_GRAVE,LOCATION_GRAVE,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| filter(d, c, a.e, tp))), 0, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if #g>0 and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
    if !g.is_empty()
        && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(1-tp,1,1,nil)
        let sg = group::select(d, &g, 1 - tp, 1, 1, false, NO_EX).nil();
        // Duel.SpecialSummon(sg,0,1-tp,1-tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, 1 - tp, 1 - tp, false, false, POS_FACEUP as u8, None);
    }
}
