//! The World Tree
use super::prelude::*;

pub const ID: u32 = 5973663;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x18], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x18)
    card::enable_counter_permit(d, c, 0x18, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(s.ctcon)
    effect::set_condition(d, e2, Some(condition(ctcon)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atkup defup
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCost(s.cost1)
    effect::set_cost(d, e3, Some(cost_target(cost1)));
    // e3:SetTarget(s.tg1)
    effect::set_target(d, e3, Some(cost_target(tg1)));
    // e3:SetOperation(s.op1)
    effect::set_operation(d, e3, Some(operation(op1)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCost(s.cost2)
    effect::set_cost(d, e4, Some(cost_target(cost2)));
    // e4:SetTarget(s.tg2)
    effect::set_target(d, e4, Some(cost_target(tg2)));
    // e4:SetOperation(s.op2)
    effect::set_operation(d, e4, Some(operation(op2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // special summon
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e5, aux::stringid(ID, 2));
    // e5:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCost(s.cost3)
    effect::set_cost(d, e5, Some(cost_target(cost3)));
    // e5:SetTarget(s.tg3)
    effect::set_target(d, e5, Some(cost_target(tg3)));
    // e5:SetOperation(s.op3)
    effect::set_operation(d, e5, Some(operation(op3)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.ctfilter(c)
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP)
    //   and (c:GetPreviousRaceOnField()&RACE_PLANT)~=0
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
        && (card::get_previous_race_on_field(d, c) & RACE_PLANT) != 0
}
// function s.ctcon(e,tp,eg,ep,ev,re,r,rp)
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.ctfilter,1,nil)
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| ctfilter(d, c), 1, NO_EX)
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0x18,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x18, 1, false);
}
// function s.cost1(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost1(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return e:GetHandler():IsCanRemoveCounter(tp,0x18,1,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, h, tp, 0x18, 1, REASON_COST);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // e:GetHandler():RemoveCounter(tp,0x18,1,REASON_COST)
    card::remove_counter(d, h, tp, 0x18, 1, REASON_COST);
    true
}
// function s.filter1(c)
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_PLANT)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT)
}
// function s.tg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tg1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter1(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter1(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter1), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter1), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g,1,0,500)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, g, 1, 0, 500);
    true
}
// function s.op1(e,tp,eg,ep,ev,re,r,rp)
fn op1(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // e1:SetValue(400)
            effect::set_value(d, e1, 400);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_UPDATE_DEFENSE)
            effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
// function s.cost2(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost2(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return e:GetHandler():IsCanRemoveCounter(tp,0x18,2,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, h, tp, 0x18, 2, REASON_COST);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // e:GetHandler():RemoveCounter(tp,0x18,2,REASON_COST)
    card::remove_counter(d, h, tp, 0x18, 2, REASON_COST);
    true
}
// function s.tg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
// function s.op2(e,tp,eg,ep,ev,re,r,rp)
fn op2(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
// function s.cost3(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost3(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return e:GetHandler():IsCanRemoveCounter(tp,0x18,3,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, h, tp, 0x18, 3, REASON_COST);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    // e:GetHandler():RemoveCounter(tp,0x18,3,REASON_COST)
    card::remove_counter(d, h, tp, 0x18, 3, REASON_COST);
    true
}
// function s.filter3(c,e,tp)
fn filter3(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_PLANT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_PLANT) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.tg3(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tg3(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter3(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter3(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.filter3,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| filter3(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter3,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| filter3(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
// function s.op3(e,tp,eg,ep,ev,re,r,rp)
fn op3(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
