//! Inherited Fortune
//! フォーチュン・インハーリット
use super::prelude::*;

pub const ID: u32 = 20057949;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FORTUNE_LADY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // s[0]=false
        // s[1]=false
        set_s_var(d, ID, 0, Value::Bool(false));
        set_s_var(d, ID, 1, Value::Bool(false));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DESTROYED)
        effect::set_code(d, ge1, EVENT_DESTROYED);
        // ge1:SetOperation(s.checkop1)
        effect::set_operation(d, ge1, Some(operation(checkop1)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // aux.AddValuesReset(function()
        //     s[0]=false
        //     s[1]=false
        // end)
        aux::add_values_reset(d, reset_s);
    });
    // end)
}
/// The reset of `s[0]` and `s[1]` (`aux.AddValuesReset`)
fn reset_s(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Bool(false));
    set_s_var(d, ID, 1, Value::Bool(false));
}
/// `s.checkop1(e,tp,eg,ep,ev,re,r,rp)`
fn checkop1(d: &mut Duel, a: &EffArgs) {
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &a.eg()) {
        // if tc:IsPreviousLocation(LOCATION_MZONE) and tc:IsPreviousPosition(POS_FACEUP) and tc:IsPreviousSetCard(SET_FORTUNE_LADY) then
        if card::is_previous_location(d, tc, LOCATION_MZONE)
            && card::is_previous_position(d, tc, POS_FACEUP)
            && card::is_previous_set_card(d, tc, &[SET_FORTUNE_LADY as u16])
        {
            // s[tc:GetPreviousControler()]=true
            let p = card::get_previous_controler(d, tc);
            set_s_var(d, ID, p as u64, Value::Bool(true));
        }
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return s[tp]
    s_var(d, ID, a.tp as u64).truthy()
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetLabel(Duel.GetTurnCount())
    effect::set_label(d, e1, &[duel::get_turn_count(d, None) as i64]);
    // e1:SetCondition(s.spcon1)
    effect::set_condition(d, e1, Some(condition(spcon1)));
    // e1:SetOperation(s.spop1)
    effect::set_operation(d, e1, Some(operation(spop1)));
    // if Duel.IsTurnPlayer(tp) then
    if duel::is_turn_player(d, a.tp) {
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,1)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 1);
    }
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.spcon1(e,tp,eg,ep,ev,re,r,rp)`
fn spcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()~=e:GetLabel() and Duel.IsTurnPlayer(tp)
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e) && duel::is_turn_player(d, a.tp)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_FORTUNE_LADY) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_FORTUNE_LADY as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spop1(e,tp,eg,ep,ev,re,r,rp)`
fn spop1(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
    {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
        // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetTarget(s.sptg2)
        effect::set_target(d, e1, Some(cost_target(sptg2)));
        // e1:SetOperation(s.spop2)
        effect::set_operation(d, e1, Some(operation(spop2)));
        // e1:SetReset(RESET_PHASE|PHASE_STANDBY)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if ft>2 then ft=2 end
    if ft > 2 {
        ft = 2;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,ft,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, ft as u16, false, NO_EX).nil();
    // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
