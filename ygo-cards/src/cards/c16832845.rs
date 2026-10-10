//! Final Geas
use super::prelude::*;

pub const ID: u32 = 16832845;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE|TIMING_TOGRAVE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE | TIMING_TOGRAVE));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // s[0]=false
        set_s_var(d, ID, 0, Value::Bool(false));
        // s[1]=false
        set_s_var(d, ID, 1, Value::Bool(false));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_TO_GRAVE)
        effect::set_code(d, ge1, EVENT_TO_GRAVE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // aux.AddValuesReset(function()
        //     s[0]=false
        //     s[1]=false
        // end)
        aux::add_values_reset(d, reset_values);
    });
}
/// `aux.AddValuesReset` function
fn reset_values(d: &mut Duel) {
    // s[0]=false
    set_s_var(d, ID, 0, Value::Bool(false));
    // s[1]=false
    set_s_var(d, ID, 1, Value::Bool(false));
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let _ = group::get_first(d, &eg);
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // if tc:GetOriginalLevel()>=7 and tc:IsPreviousLocation(LOCATION_MZONE) then
        if card::get_original_level(d, tc) >= 7 && card::is_previous_location(d, tc, LOCATION_MZONE) {
            // s[tc:GetPreviousControler()]=true
            let p = card::get_previous_controler(d, tc) as u64;
            set_s_var(d, ID, p, Value::Bool(true));
        }
    }
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return s[0] and s[1]
    s_var(d, ID, 0).truthy() && s_var(d, ID, 1).truthy()
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove()
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if g:FilterCount(Card.IsControler,nil,1-tp)==0 then
    let n = g.len() as u8;
    if group::filter_count(d, &g, &|d, c| card::is_controler(d, c, 1 - tp), NO_EX) == 0 {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, tp, LOCATION_GRAVE as i32);
    // elseif g:FilterCount(Card.IsControler,nil,tp)==0 then
    } else if group::filter_count(d, &g, &|d, c| card::is_controler(d, c, tp), NO_EX) == 0 {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,1-tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 1 - tp, LOCATION_GRAVE as i32);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,PLAYER_ALL,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and c:IsLocation(LOCATION_REMOVED) and c:HasLevel()
    card::is_race(d, c, RACE_SPELLCASTER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_location(d, c, LOCATION_REMOVED)
        && card::has_level(d, c)
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if Duel.Remove(g,POS_FACEUP,REASON_EFFECT)>0 then
    if duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // local og=Duel.GetOperatedGroup():Filter(s.spfilter,nil,e,tp)
        let opg = duel::get_operated_group(d);
        let og = group::filter(d, &opg, &|d, c| spfilter(d, c, a.e, tp), NO_EX);
        // if #og>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if !og.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local sg=og:GetMaxGroup(Card.GetLevel)
            let mut sg = group::get_max_group(d, &og, &|d, c| card::get_level(d, c) as i64).map_or_else(CardSet::new, |(g, _)| g);
            // if #sg>1 then
            if sg.len() > 1 {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                // sg=sg:Select(tp,1,1,nil)
                sg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
            }
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
