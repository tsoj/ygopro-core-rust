//! Vaalmonica Chosen Melody
use super::prelude::*;

pub const ID: u32 = 31971040;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VAALMONICA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RECOVER+CATEGORY_DAMAGE+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_RECOVER + CATEGORY_DAMAGE + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(lua_fn(activate_lua)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.vaalmonicafilter(c)
fn vaalmonicafilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAALMONICA) and c:IsMonsterCard() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_VAALMONICA as u16]) && card::is_monster_card(d, c) && card::is_faceup(d, c)
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.vaalmonicafilter,tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&vaalmonicafilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,500)
    duel::set_possible_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 500);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,500)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 500);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
// function s.linkfilter(c)
fn linkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAALMONICA) and c:IsFaceup() and c:IsLinkMonster()
    card::is_set_card(d, c, &[SET_VAALMONICA as u16]) && card::is_faceup(d, c) && card::is_link_monster(d, c)
}
// function s.disfilter(c)
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNegatableMonster() and c:IsType(TYPE_EFFECT)
    card::is_negatable_monster(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp,angello_or_dimonno)
/// (registered as a raw function: "Angello Vaalmonica" / "Dimonno Vaalmonica" call it with a 9th argument)
fn activate_lua(d: &mut Duel, a: Args) {
    let extra = match a.v(8) {
        Value::Int(n) => Some(*n),
        _ => None,
    };
    activate_with(d, &EffArgs::new(a.0), extra);
}
/// `s.activate` with the extra argument that "Angello Vaalmonica" and "Dimonno Vaalmonica" pass
pub fn activate_with(d: &mut Duel, a: &EffArgs, angello_or_dimonno: Option<i64>) {
    // if not Duel.IsExistingMatchingCard(s.vaalmonicafilter,tp,LOCATION_ONFIELD,0,1,nil) then return end
    if !duel::is_existing_matching_card(d, Some(&vaalmonicafilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX) {
        return;
    }
    // local op=nil
    let op: Option<i64> = if angello_or_dimonno.is_some() {
        // op=angello_or_dimonno
        angello_or_dimonno
    } else {
        // local both=Duel.IsExistingMatchingCard(s.linkfilter,tp,LOCATION_MZONE,0,1,nil)
        let both = duel::is_existing_matching_card(d, Some(&linkfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
        // op=Duel.SelectEffect(tp,{true,aux.Stringid(id,1)},{true,aux.Stringid(id,2)},{both,aux.Stringid(id,3)})
        duel::select_effect(d, a.tp, &[(true, aux::stringid(ID, 1)), (true, aux::stringid(ID, 2)), (both, aux::stringid(ID, 3))]).map(|x| x as i64)
    };
    // local break_chk=nil
    let mut break_chk = false;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if op==1 or op==3 then
    if op == Some(1) || op == Some(3) {
        // Gain 500 LP and apply the targeting procetion effect
        // if Duel.Recover(tp,500,REASON_EFFECT)>0 then
        if duel::recover(d, a.tp, 500, REASON_EFFECT, false, None) > 0 {
            // break_chk=true
            break_chk = true;
        }
        // if not Duel.HasFlagEffect(tp,id) then
        if !duel::has_flag_effect(d, a.tp, ID, None) {
            // break_chk=true
            break_chk = true;
            // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,aux.Stringid(id,4))
            duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, aux::stringid(ID, 4) as u32);
            // Your opponent cannot target "Vaalmonica" Monster Cards you control with card effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
            effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
            // e1:SetTargetRange(LOCATION_ONFIELD,0)
            effect::set_target_range(d, e1, LOCATION_ONFIELD, 0);
            // e1:SetTarget(function(e,c) return s.vaalmonicafilter(c) end)
            effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| vaalmonicafilter(d, c))));
            // e1:SetValue(aux.tgoval)
            effect::set_value_fn(d, e1, aux::tgoval());
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
        }
    }
    // if op==2 or op==3 then
    if op == Some(2) || op == Some(3) {
        // Take 500 damage and negate the effects of 1 opponent's Effect Monster
        // local g=Duel.GetMatchingGroup(s.disfilter,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&disfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
        // if break_chk then Duel.BreakEffect() end
        if break_chk {
            duel::break_effect(d);
        }
        // if Duel.Damage(tp,500,REASON_EFFECT)>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,5)) then
        if duel::damage(d, a.tp, 500, REASON_EFFECT, false, None) > 0 && !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 5)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
            // local nc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            let nc = group::get_first(d, &sel).nil();
            // Duel.HintSelection(nc)
            duel::hint_selection(d, nc, true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // nc:NegateEffects(c,RESET_PHASE|PHASE_END)
            card::negate_effects(d, nc, c, Some(RESET_PHASE | PHASE_END), false, None);
        }
    }
}
