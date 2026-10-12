//! Sennet Cataract
use super::prelude::*;

pub const ID: u32 = 100458010;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ZENET],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.tgcon)
    effect::set_condition(d, e2, Some(condition(tgcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ctfilter(c)`: `(c:IsType(TYPE_NORMAL) or (c:IsSetCard(SET_ZENET) and c:IsRitualMonster())) and c:IsFaceup()`
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_type(d, c, TYPE_NORMAL) || (card::is_set_card(d, c, &[SET_ZENET as u16]) && card::is_ritual_monster(d, c))) && card::is_faceup(d, c)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(s.ctfilter,tp,LOCATION_MZONE,0,nil)
    let mut ct = duel::get_matching_group_count(d, Some(&ctfilter), tp, LOCATION_MZONE, 0, NO_EX) as u32;
    // if chk==0 then return ct>0 end
    if a.chk == Some(0) {
        return ct > 0;
    }
    // ct=math.min(ct,5)
    ct = ct.min(5);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local selected_zones=Duel.SelectFieldZone(tp,ct,0,LOCATION_MZONE,ZONES_EMZ<<16)
    let selected_zones = duel::select_field_zone(d, tp, ct as u8, 0, LOCATION_MZONE, Some(ZONES_EMZ << 16));
    // Duel.Hint(HINT_ZONE,tp,selected_zones)
    duel::hint(d, HINT_ZONE as u8, tp, selected_zones as u64);
    // e:GetChainData().selected_zones=selected_zones
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(selected_zones as i64));
    true
}
/// The condition of the continuous effect `e1` of `s.negop`:
/// `Chain.IsTriggeringLocation(ev,LOCATION_MMZONE) and Chain.IsTriggeringControler(ev,1-tp)`
fn negop_cond(d: &mut Duel, a: &EffArgs) -> bool {
    chain::is_triggering(d, a.ev as u8, chain::Prop::Location, &[LOCATION_MMZONE as i64]).truthy()
        && chain::is_triggering(d, a.ev as u8, chain::Prop::Controler, &[(1 - a.tp) as i64]).truthy()
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local selected_zones=e:GetChainData().selected_zones
    let selected_zones = chain::effect_data(d, a.e, None, chain::field::CARD).int() as u32;
    // Duel.Hint(HINT_ZONE,tp,selected_zones)
    duel::hint(d, HINT_ZONE as u8, tp, selected_zones as u64);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_condition(d, e1, Some(condition(negop_cond)));
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    // (the remaining zones are kept in the label of e1: they change while the effect runs)
    effect::set_operation(d, e1, Some(operation(negop_solving)));
    effect::set_label(d, e1, &[selected_zones as i64]);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// The operation of `e1` in `s.negop`: negate the first monster effect activated in each of the zones.
fn negop_solving(d: &mut Duel, a: &EffArgs) {
    // local zone=1<<(Chain.GetTriggeringSequence(ev)+16)
    let seq = chain::get_triggering(d, a.ev as u8, chain::Prop::Sequence).int();
    let zone = 1u32 << (seq + 16);
    // local selected_zones: kept in the label of e
    let mut selected_zones = effect::get_label(d, a.e) as u32;
    // if (selected_zones&zone)==0 then return end
    if selected_zones & zone == 0 {
        return;
    }
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
    // selected_zones=selected_zones&~zone
    selected_zones &= !zone;
    effect::set_label(d, a.e, &[selected_zones as i64]);
    // if selected_zones==0 then e:Reset() end
    if selected_zones == 0 {
        effect::reset(d, a.e);
    }
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsOriginalType,TYPE_NORMAL),tp,LOCATION_ONFIELD,0,nil):GetSum(Card.GetAttack)>3000
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_original_type(d, c, TYPE_NORMAL)), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    group::get_sum(d, &g, &|d, c| card::get_attack(d, c) as i64) > 3000
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsAbleToGrave() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_able_to_grave(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToGrave,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToGrave,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), 1, tp, 0);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.SendtoGrave(tc,REASON_EFFECT) end
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    }
}
