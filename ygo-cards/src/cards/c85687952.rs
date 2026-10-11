//! Theia, the Primal Being
use super::prelude::*;

pub const ID: u32 = 85687952;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Neither player can Normal and/or Special Summon more than 4 times per turn while this card is face-up in the Monster Zone
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetTarget(function(e,c,tp) return e:GetHandler():GetFlagEffect(id+tp)>=4 end)
    effect::set_target(d, e2, Some(lua_fn(summon_target)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Register a flag to this card everytime Normal and Special Summons happen
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(function(e,tp,eg) return eg and not eg:IsContains(e:GetHandler()) end)
    effect::set_condition(d, e4, Some(condition(flag_cond)));
    // e4:SetOperation(function(e,tp,eg,ep) e:GetHandler():RegisterFlagEffect(id+ep,RESETS_STANDARD_PHASE_END,0,1) end)
    effect::set_operation(d, e4, Some(operation(flag_op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Special Summon left count
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetCode(EFFECT_LEFT_SPSUMMON_COUNT)
    effect::set_code(d, e6, EFFECT_LEFT_SPSUMMON_COUNT);
    // e6:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetTargetRange(1,1)
    effect::set_target_range(d, e6, 1, 1);
    // e6:SetValue(s.countval)
    effect::set_value_fn(d, e6, lua_fn(countval));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.spcheck(sg,tp)`
fn spcheck(d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet) -> (bool, bool) {
    // return aux.ReleaseCheckMMZ(sg,tp) and not sg:IsExists(Card.IsLevelBelow,1,nil,sg:GetSum(Card.GetLevel)-11)
    let (ok, _) = aux::release_check_mmz(d, sg, tp, exg);
    if !ok {
        return (false, false);
    }
    let sum = group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64);
    // a negative Lua number reads as uint32 in IsLevelBelow
    let lv = (sum - 11) as u32;
    let exists = group::is_exists(d, sg, &|d, c| card::is_level_below(d, c, lv), 1, NO_EX);
    (!exists, false)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.HasLevel,1,99,false,s.spcheck,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&card::has_level), 1, 99, false, Some(&spcheck), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.HasLevel,1,99,false,s.spcheck,nil)
    let g = duel::select_release_group_cost(d, tp, Some(&card::has_level), 1, 99, false, Some(&spcheck), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,PLAYER_EITHER,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==1 then
    if !(card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 1)
    {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil):GetMaxGroup(Card.GetAttack)
    let all = duel::get_matching_group(d, Some(&|d, cc| card::is_faceup(d, cc)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let mut g = group::get_max_group(d, &all, &|d, cc| card::get_attack(d, cc) as i64).map_or(CardSet::new(), |x| x.0);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // if #g>1 then
        if g.len() > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // g=g:Select(tp,1,1,nil)
            g = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        }
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `function(e,c,tp) return e:GetHandler():GetFlagEffect(id+tp)>=4 end`
fn summon_target(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    let tp = a.int(2) as u32;
    card::get_flag_effect(d, h, ID + tp) >= 4
}
/// `function(e,tp,eg) return eg and not eg:IsContains(e:GetHandler()) end`
fn flag_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let Some(eg) = a.eg.clone() else { return false };
    let h = effect::get_handler(d, a.e).nil();
    !group::is_contains(&eg, h)
}
/// `function(e,tp,eg,ep) e:GetHandler():RegisterFlagEffect(id+ep,RESETS_STANDARD_PHASE_END,0,1) end`
fn flag_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID + a.ep as u32, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
}
/// `function(e,re,sump) local ct=e:GetHandler():GetFlagEffect(id+sump) if ct>=4 then return 0 else return 4-ct end end`
fn countval(d: &mut Duel, a: Args) -> i64 {
    let h = effect::get_handler(d, a.e(0)).nil();
    let sump = a.int(2) as u32;
    let ct = card::get_flag_effect(d, h, ID + sump) as i64;
    if ct >= 4 {
        0
    } else {
        4 - ct
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) return Duel.IsMainPhase() end` (the condition)
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
