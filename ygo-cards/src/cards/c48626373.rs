//! Kashtira Arise-Heart
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 48626373;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[73542331], listed_series: &[SET_KASHTIRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 3 Level 7 monsters
    // Xyz.AddProcedure(c,nil,7,3,s.xyzfilter,aux.Stringid(id,0),nil,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(7),
        3,
        Some(xyz::alter_filter(xyzfilter)),
        Some(aux::stringid(ID, 0)),
        None,
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // Cards sent to the GY are banished instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_ALL,LOCATION_ALL)
    effect::set_target_range(d, e1, LOCATION_ALL, LOCATION_ALL);
    // e1:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e1, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach 1 banished card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(s.ovcost)
    effect::set_cost(d, e2, Some(cost_target(ovcost)));
    // e2:SetOperation(s.ovop)
    effect::set_operation(d, e2, Some(operation(ovop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish 1 card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e3:SetCost(Cost.DetachFromSelf(3,3))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(3), Some(cost::Count::N(3)), None)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Check if "Kashtira Shangri-lra" activated its effects this turn
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,function(re) return not re:GetHandler():IsCode(73542331) end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(counterfilter));
}
/// `function(re) return not re:GetHandler():IsCode(73542331) end` (the activity counter filter)
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    let re = a.e(0);
    let h = effect::get_handler(d, re).nil();
    !card::is_code(d, h, &[73542331])
}
/// `s.xyzfilter(c,tp,xyzc)`
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_KASHTIRA,xyzc,SUMMON_TYPE_XYZ,tp)
    card::is_faceup(d, c) && card::is_set_card_ctx(d, c, &[SET_KASHTIRA as u16], Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0
    //     and (Duel.GetCustomActivityCount(id,tp,ACTIVITY_CHAIN)>0 or Duel.GetCustomActivityCount(id,1-tp,ACTIVITY_CHAIN)>0) end
    if a.int(2) == 0 {
        return duel::get_flag_effect(d, tp, ID) == 0
            && (duel::get_custom_activity_count(d, ID, tp, ACTIVITY_CHAIN) > 0 || duel::get_custom_activity_count(d, ID, 1 - tp, ACTIVITY_CHAIN) > 0);
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0);
    // return true
    true
}
/// `s.ovcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ovcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(id)==0 and c:IsType(TYPE_XYZ) end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0 && card::is_type(d, c, TYPE_XYZ);
    }
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    true
}
/// `s.ovfilter(c,xc,tp,e)`
fn ovfilter(d: &mut Duel, c: CardId, xc: CardId, tp: u8, e: EffectId) -> bool {
    // return c:IsCanBeXyzMaterial(xc,tp,REASON_EFFECT) and not c:IsImmuneToEffect(e)
    card::is_can_be_xyz_material(d, c, Some(xc), Some(tp), Some(REASON_EFFECT)) && !card::is_immune_to_effect(d, c, e)
}
/// `s.ovop(e,tp,eg,ep,ev,re,r,rp)`
fn ovop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local g=Duel.SelectMatchingCard(tp,s.ovfilter,tp,LOCATION_REMOVED,LOCATION_REMOVED,1,1,nil,c,tp,e)
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| ovfilter(d, cc, c, tp, a.e)), tp, LOCATION_REMOVED, LOCATION_REMOVED, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Overlay(c,g)
        duel::overlay(d, c, g, false);
    }
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToRemove(tp,POS_FACEDOWN) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_remove(d, chkc, Some(tp), Some(POS_FACEDOWN as u8), None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil,tp,POS_FACEDOWN) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil,tp,POS_FACEDOWN)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, CardSet::from_card(tc), POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    }
}
