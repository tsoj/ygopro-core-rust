//! Lady Labrynth of the Silver Castle
use super::prelude::*;

pub const ID: u32 = 81497285;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LABRYNTH], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be targeted by the opponent's card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.tgcond)
    effect::set_condition(d, e1, Some(lua_fn(tgcond)));
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by the opponent's card effects
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetValue(aux.indoval)
    effect::set_value_fn(d, e2, aux::indoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon itself in Defense position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetHintTiming(TIMING_END_PHASE,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, TIMING_END_PHASE, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(function() return Duel.GetCustomActivityCount(id,0,ACTIVITY_CHAIN)>0 or Duel.GetCustomActivityCount(id,1,ACTIVITY_CHAIN)>0 end)
    effect::set_condition(
        d,
        e3,
        Some(lua_fn(|d, _a: Args| duel::get_custom_activity_count(d, ID, 0, ACTIVITY_CHAIN) > 0 || duel::get_custom_activity_count(d, ID, 1, ACTIVITY_CHAIN) > 0)),
    );
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Set 1 Normal Trap from the Deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_SET)
    effect::set_category(d, e4, CATEGORY_SET);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(s.setcond)
    effect::set_condition(d, e4, Some(condition(setcond)));
    // e4:SetTarget(s.settg)
    effect::set_target(d, e4, Some(cost_target(settg)));
    // e4:SetOperation(s.setop)
    effect::set_operation(d, e4, Some(operation(setop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Check if a "Labrynth" card or effect or a Normal Trap Card was activated this turn
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,s.checkop)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(checkop));
}
// function s.tgcond(e)
fn tgcond(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsFacedown,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
// function s.checkop(re)
fn checkop(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(0);
    let rc = effect::get_handler(d, re).nil();
    // return not ((rc:IsSetCard(SET_LABRYNTH) and not rc:IsCode(id)) or (re:IsHasType(EFFECT_TYPE_ACTIVATE) and rc:IsNormalTrap()))
    !((card::is_set_card(d, rc, &[SET_LABRYNTH as u16]) && !card::is_code(d, rc, &[ID]))
        || (effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && card::is_normal_trap(d, rc)))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_HAND as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
// re:GetHandler():GetCode() (the first code: the extra codes are dropped by the filter)
fn handler_code(d: &mut Duel, re: EffectId) -> u32 {
    let rc = effect::get_handler(d, re).nil();
    card::get_code(d, rc)[0]
}
// function s.setcond(e,tp,eg,ep,ev,re,r,rp)
fn setcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:GetActiveType()==TYPE_TRAP and re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.re.nil();
    effect::get_active_type(d, re) == TYPE_TRAP && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
// function s.setfilter(c,code)
fn setfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsNormalTrap() and c:IsSSetable() and not c:IsCode(code)
    card::is_normal_trap(d, c) && card::is_s_setable(d, c, false, None) && !card::is_code(d, c, &[code])
}
// function s.settg(e,tp,eg,ep,ev,re,r,rp,chk)
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil,re:GetHandler():GetCode()) end
    if a.chk == Some(0) {
        let re = a.re.nil();
        let code = handler_code(d, re);
        return duel::is_existing_matching_card(d, Some(&|d, c| setfilter(d, c, code)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
// function s.setop(e,tp,eg,ep,ev,re,r,rp)
fn setop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil,re:GetHandler():GetCode())
    let re = a.re.nil();
    let code = handler_code(d, re);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| setfilter(d, c, code)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g)
        duel::s_set(d, a.tp, g, None, true);
    }
}
