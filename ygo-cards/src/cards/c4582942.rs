//! Vaalmonica Followed Rhythm
//! 律導のヴァルモニカ
use super::prelude::*;

pub const ID: u32 = 4582942;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_VAALMONICA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RECOVER+CATEGORY_DESTROY+CATEGORY_DAMAGE+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_RECOVER + CATEGORY_DESTROY + CATEGORY_DAMAGE + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(lua_fn(activate_lua)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAALMONICA) and c:IsFaceup() and c:IsOriginalType(TYPE_MONSTER)
    card::is_set_card(d, c, &[SET_VAALMONICA as u16]) && card::is_faceup(d, c) && card::is_original_type(d, c, TYPE_MONSTER)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,500)
    duel::set_possible_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, 500);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,PLAYER_EITHER,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,500)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 500);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,PLAYER_EITHER,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    true
}
/// `s.linkfilter(c)`
fn linkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAALMONICA) and c:IsFaceup() and c:IsLinkMonster()
    card::is_set_card(d, c, &[SET_VAALMONICA as u16]) && card::is_faceup(d, c) && card::is_link_monster(d, c)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`: the operation of the activation
/// (registered as a raw function: "Angello Vaalmonica" / "Dimonno Vaalmonica" call it with a 9th argument)
fn activate_lua(d: &mut Duel, a: Args) {
    let extra = match a.v(8) {
        Value::Int(n) => Some(*n),
        _ => None,
    };
    activate_op(d, &EffArgs::new(a.0), extra);
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp,angello_or_dimonno)`: the 9th parameter is used by
/// "Angello Vaalmonica" and "Dimonno Vaalmonica"
pub fn activate_op(d: &mut Duel, a: &EffArgs, angello_or_dimonno: Option<i64>) {
    let tp = a.tp;
    // if not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil) then return end
    if !duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX) {
        return;
    }
    // local op=nil
    // if angello_or_dimonno then
    let op: Option<i64> = if let Some(v) = angello_or_dimonno {
        // op=angello_or_dimonno
        Some(v)
    } else {
        // local both=Duel.IsExistingMatchingCard(s.linkfilter,tp,LOCATION_MZONE,0,1,nil)
        let both = duel::is_existing_matching_card(d, Some(&linkfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
        // op=Duel.SelectEffect(tp,{true,aux.Stringid(id,1)},{true,aux.Stringid(id,2)},{both,aux.Stringid(id,3)})
        duel::select_effect(
            d,
            tp,
            &[(true, aux::stringid(ID, 1)), (true, aux::stringid(ID, 2)), (both, aux::stringid(ID, 3))],
        )
        .map(|v| v as i64)
    };
    // local break_chk=nil
    let mut break_chk = false;
    // if op==1 or op==3 then
    if op == Some(1) || op == Some(3) {
        // Gain 500 LP and Destroy 1 Spell/Trap on the field
        // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let g = duel::get_matching_group(d, Some(&card::is_spell_trap), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
        // if Duel.Recover(tp,500,REASON_EFFECT)>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
        if duel::recover(d, tp, 500, REASON_EFFECT, false, None) > 0 && !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 4)) {
            // break_chk=true
            break_chk = true;
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local dg=g:Select(tp,1,1,nil)
            let dg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(dg,true)
            duel::hint_selection(d, dg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
    // if op==2 or op==3 then
    if op == Some(2) || op == Some(3) {
        // Take 500 damage and return 1 monster on the field to the hand
        // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // if break_chk then Duel.BreakEffect() end
        if break_chk {
            duel::break_effect(d);
        }
        // if Duel.Damage(tp,500,REASON_EFFECT)>0 and #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,5)) then
        if duel::damage(d, tp, 500, REASON_EFFECT, false, None) > 0 && !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 5)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
            // local hg=g:Select(tp,1,1,nil)
            let hg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(hg,true)
            duel::hint_selection(d, hg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(hg,nil,REASON_EFFECT)
            duel::send_to_hand(d, hg, None, REASON_EFFECT, None);
        }
    }
}
