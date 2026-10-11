//! Terrifying Trap Hole Nightmare
use super::prelude::*;

pub const ID: u32 = 75294187;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRAPTRIX, SET_HOLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 monster with 2000 or more ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(aux.Stringid(id,0))
    effect::set_category(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCondition(function(_,tp) return Duel.GetActivityCount(1-tp,ACTIVITY_SPSUMMON)>0 end)
    effect::set_condition(d, e1, Some(lua_fn(cond)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `e1:SetCondition(function(_,tp) ... end)`
fn cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // Duel.GetActivityCount(1-tp,ACTIVITY_SPSUMMON)>0
    duel::get_activity_count(d, 1 - tp, &[ACTIVITY_SPSUMMON])[0] > 0
}
/// `aux.FaceupFilter(Card.IsAttackAbove,2000)`
fn above_2000() -> impl Fn(&mut Duel, CardId) -> bool {
    aux::faceup_filter(|d, c| card::is_attack_above(d, c, 2000))
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() and chkc:IsAttackAbove(2000) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_faceup(d, chkc) && card::is_attack_above(d, chkc, 2000);
    }
    let f = above_2000();
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAttackAbove,2000),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&f), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAttackAbove,2000),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&f), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
/// `s.hlfilter(c)`
fn hlfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalTrap() and c:IsSetCard({SET_HOLE,SET_TRAP_HOLE})
    card::is_normal_trap(d, c) && card::is_set_card(d, c, &[SET_HOLE as u16, SET_TRAP_HOLE as u16])
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove() and aux.SpElimFilter(c)
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or Duel.Destroy(tc,REASON_EFFECT)==0
    //     or not Duel.IsExistingMatchingCard(s.hlfilter,tp,LOCATION_GRAVE,0,1,nil) then return end
    if !card::is_relate_to_effect(d, tc, a.e)
        || duel::destroy(d, tc, REASON_EFFECT, None, None) == 0
        || !duel::is_existing_matching_card(d, Some(&hlfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX)
    {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&rmfilter), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, NO_EX);
    // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
    if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rg=g:Select(tp,1,1,nil)
    let rg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
