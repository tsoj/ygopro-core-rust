//! Witchcrafter Bystreet
//! ウィッチクラフト・バイストリート
use super::prelude::*;

pub const ID: u32 = 83289866;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WITCHCRAFTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // indestructable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_WITCHCRAFTER))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_WITCHCRAFTER as u16]))));
    // e2:SetValue(s.indct)
    effect::set_value_fn(d, e2, lua_fn(indct));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // replace discard effect
    // local e3=Witchcrafter.CreateCostReplaceEffect(c)
    let e3 = aux::witchcrafter::create_cost_replace_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(function(base) return base:GetHandler():IsAbleToGraveAsCost() end)
    effect::set_condition(d, e3, Some(lua_fn(replace_cond)));
    // e3:SetOperation(function(base) Duel.SendtoGrave(base:GetHandler(),REASON_COST) end)
    effect::set_operation(d, e3, Some(lua_fn(replace_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // to S/T Zone
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e4, LOCATION_GRAVE);
    // e4:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(s.tfcond)
    effect::set_condition(d, e4, Some(condition(tfcond)));
    // e4:SetTarget(s.tftg)
    effect::set_target(d, e4, Some(cost_target(tftg)));
    // e4:SetOperation(s.tfop)
    effect::set_operation(d, e4, Some(operation(tfop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(base) return base:GetHandler():IsAbleToGraveAsCost() end`
fn replace_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_able_to_grave_as_cost(d, h)
}
/// `function(base) Duel.SendtoGrave(base:GetHandler(),REASON_COST) end`
fn replace_op(d: &mut Duel, a: Args) {
    let h = effect::get_handler(d, a.e(0)).nil();
    duel::send_to_grave(d, h, REASON_COST, None, None);
}
/// `s.indct(e,re,r,rp)`
fn indct(_d: &mut Duel, a: Args) -> i64 {
    // if (r&REASON_BATTLE+REASON_EFFECT)~=0 then
    if (a.int(2) as u32 & (REASON_BATTLE + REASON_EFFECT)) != 0 {
        // return 1
        1
    } else {
        // return 0
        0
    }
}
/// `s.tfcond(e,tp,eg,ep,ev,re,r,rp)`
fn tfcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_WITCHCRAFTER),tp,LOCATION_MZONE,0,1,nil) and Duel.IsTurnPlayer(tp)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_WITCHCRAFTER as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_turn_player(d, a.tp)
}
/// `s.tftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.tfop(e,tp,eg,ep,ev,re,r,rp)`
fn tfop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}
