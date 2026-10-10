//! Tistina, the Divinity that Defies Darkness
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 87676171;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_CRYSTAL_GOD_TISTINA], listed_series: &[SET_TISTINA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2 Level 10 monsters
    // Xyz.AddProcedure(c,nil,10,2)
    xyz::add_procedure(d, c, None, Some(10), 2, None, None, None, None, false, None);
    // Send all face-down cards the opponent controls to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.gytg)
    effect::set_target(d, e1, Some(cost_target(gytg)));
    // e1:SetOperation(s.gyop)
    effect::set_operation(d, e1, Some(operation(gyop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gain 2000 ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,CARD_CRYSTAL_GOD_TISTINA) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        let og = card::get_overlay_group(d, h);
        group::is_exists(d, &og, &|d, x| card::is_code(d, x, &[CARD_CRYSTAL_GOD_TISTINA]), 1, NO_EX)
    })));
    // e2:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Tistina" monster from the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Check materials before leaving field
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD_P);
    // e4:SetOperation(function(e) e:GetLabelObject():SetLabel(e:GetHandler():GetOverlayCount()) end)
    effect::set_operation(d, e4, Some(operation(|d, a| {
        let e3 = effect::get_label_object(d, a.e).as_effect().nil();
        let h = effect::get_handler(d, a.e).nil();
        let count = card::get_overlay_count(d, h) as i64;
        effect::set_label(d, e3, &[count]);
    })));
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsFacedown,Card.IsAbleToGrave),tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_facedown(d, x) && card::is_able_to_grave(d, x, None)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, 0, 0);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsFacedown,Card.IsAbleToGrave),tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_facedown(d, x) && card::is_able_to_grave(d, x, None)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // --Gains 2000 ATK until the end of your opponent's turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(2000)
        effect::set_value(d, e1, 2000);
        // e1:SetReset(RESETS_STANDARD_PHASE_END,2)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 2);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and e:GetLabel()>0 and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
    a.rp == 1 - tp
        && effect::get_label(d, a.e) > 0
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_TISTINA) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_TISTINA as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
