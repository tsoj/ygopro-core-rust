//! B.E.S. Big Core MK-3
use super::prelude::*;

pub const ID: u32 = 82821760;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x1f)
    card::enable_counter_permit(d, c, 0x1f, None, None);
    // --special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SPSUM_PARAM)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SPSUM_PARAM, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetTargetRange(POS_FACEUP_DEFENSE,0)
    effect::set_target_range(d, e1, POS_FACEUP_DEFENSE as u32, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.sprcon)
    effect::set_condition(d, e1, Some(lua_fn(sprcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e2, CATEGORY_COUNTER);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.cttg)
    effect::set_target(d, e2, Some(cost_target(cttg)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --indes
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_BATTLE);
    // e4:SetValue(1)
    effect::set_value(d, e4, 1);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // --remove counter
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e5, CATEGORY_DESTROY);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e5, EVENT_DAMAGE_STEP_END);
    // e5:SetTarget(s.rcttg)
    effect::set_target(d, e5, Some(cost_target(rcttg)));
    // e5:SetOperation(s.rctop)
    effect::set_operation(d, e5, Some(operation(rctop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // --to deck
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e6, aux::stringid(ID, 2));
    // e6:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e6, CATEGORY_TODECK);
    // e6:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e6, EFFECT_TYPE_IGNITION);
    // e6:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e6, LOCATION_GRAVE);
    // e6:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e6, Some(cost::self_banish()));
    // e6:SetTarget(s.tdtg)
    effect::set_target(d, e6, Some(cost_target(tdtg)));
    // e6:SetOperation(s.tdop)
    effect::set_operation(d, e6, Some(operation(tdop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.sprcon(e,c)`
fn sprcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
        && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,3,0,0x1f)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 3, 0, 0x1f);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // e:GetHandler():AddCounter(0x1f,3)
        card::add_counter(d, h, 0x1f, 3, false);
    }
}
/// `s.rcttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rcttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if not e:GetHandler():IsCanRemoveCounter(tp,0x1f,1,REASON_EFFECT) then
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_can_remove_counter(d, h, a.tp, 0x1f, 1, REASON_EFFECT) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(h.into()), 1, 0, 0);
    }
    true
}
/// `s.rctop(e,tp,eg,ep,ev,re,r,rp)`
fn rctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // if c:IsCanRemoveCounter(tp,0x1f,1,REASON_EFFECT) then
        if card::is_can_remove_counter(d, c, a.tp, 0x1f, 1, REASON_EFFECT) {
            // c:RemoveCounter(tp,0x1f,1,REASON_EFFECT)
            card::remove_counter(d, c, a.tp, 0x1f, 1, REASON_EFFECT);
        } else {
            // Duel.Destroy(c,REASON_EFFECT)
            duel::destroy(d, c, REASON_EFFECT, None, None);
        }
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BES) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_BES as u16]) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| tdfilter(d, c)), a.tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| tdfilter(d, c)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| tdfilter(d, c)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
