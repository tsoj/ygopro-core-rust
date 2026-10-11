//! Shaddoll Zefranaga
use super::prelude::*;

pub const ID: u32 = 58016954;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHADDOLL, SET_ZEFRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // splimit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(lua_fn(splimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // spsummon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.condition1)
    effect::set_condition(d, e3, Some(condition(cond1)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetCondition(aux.TRUE)
    effect::set_condition(d, e4, Some(aux_true()));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let sumtype = a.int(3);
    // if c:IsSetCard(SET_SHADDOLL) or c:IsSetCard(SET_ZEFRA) then return false end
    if card::is_set_card(d, c, &[SET_SHADDOLL as u16]) || card::is_set_card(d, c, &[SET_ZEFRA as u16]) {
        return false;
    }
    // return (sumtype&SUMMON_TYPE_PENDULUM)==SUMMON_TYPE_PENDULUM
    (sumtype & SUMMON_TYPE_PENDULUM as i64) == SUMMON_TYPE_PENDULUM as i64
}
/// `s.condition1(e,tp,eg,ep,ev,re,r,rp)`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPendulumSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_pendulum_summoned(d, h)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZEFRA)
    card::is_set_card(d, c, &[SET_ZEFRA as u16])
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToHand()
    card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_PZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_PZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_PZONE,0,1,nil)
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_PZONE,LOCATION_PZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_PZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_PZONE, LOCATION_PZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_PZONE,LOCATION_PZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_PZONE, LOCATION_PZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_PZONE,0,1,nil) then return end
    if !duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_PZONE, 0, 1, NO_EX) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
