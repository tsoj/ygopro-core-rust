//! Seleglare the Luminous Lunar Dragon
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 29303524;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // no tribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.ntcon)
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    // e1:SetOperation(s.ntop)
    effect::set_operation(d, e1, Some(lua_fn(ntop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_SET_PROC)
    effect::set_code(d, e2, EFFECT_SET_PROC);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // control
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_CONTROL)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_CONTROL);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e3:SetCondition(s.concon)
    effect::set_condition(d, e3, Some(condition(concon)));
    // e3:SetTarget(s.contg)
    effect::set_target(d, e3, Some(cost_target(contg)));
    // e3:SetOperation(s.conop)
    effect::set_operation(d, e3, Some(operation(conop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return minc==0 and c:IsLevelAbove(5) and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    a.int(2) == 0
        && card::is_level_above(d, c, 5)
        && duel::get_location_count(d, card::get_controler(d, c), LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.ntop(e,tp,eg,ep,ev,re,r,rp,c)`
fn ntop(d: &mut Duel, a: Args) {
    let c = a.c(8);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    // e1:SetValue(1500)
    effect::set_value(d, e1, 1500);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.concon(e,tp,eg,ep,ev,re,r,rp)`
fn concon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
/// `s.confilter(c,atk)`
fn confilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:GetAttack()<=atk and c:IsControlerCanBeChanged()
    card::is_faceup(d, c) && card::get_attack(d, c) <= atk && card::is_controler_can_be_changed(d, c, false, Some(0xff))
}
/// `s.contg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn contg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=c:GetAttack()
    let atk = card::get_attack(d, c);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.confilter(chkc,atk) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && confilter(d, chkc, atk);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.confilter,tp,0,LOCATION_MZONE,1,nil,atk) and c:IsAbleToHand() end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| confilter(d, x, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX) && card::is_able_to_hand(d, c, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.confilter,tp,0,LOCATION_MZONE,1,1,nil,atk)
    let g = duel::select_target(d, tp, Some(&|d, x| confilter(d, x, atk)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
/// `s.conop(e,tp,eg,ep,ev,re,r,rp)`
fn conop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and Duel.SendtoHand(c,nil,REASON_EFFECT)~=0 and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None) != 0
        && card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        // Duel.GetControl(tc,tp,PHASE_END,1)
        duel::get_control(d, CardSet::from_card(tc.nil()), tp, PHASE_END as u16, 1, None, None);
    }
}
