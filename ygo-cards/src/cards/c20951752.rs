//! Voltanis the Adjudicator
use super::prelude::*;

pub const ID: u32 = 20951752;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // counter
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetOperation(s.chop1)
    effect::set_operation(d, e1, Some(operation(chop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetOperation(s.chop2)
    effect::set_operation(d, e2, Some(operation(chop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e3, EVENT_CHAIN_END);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCondition(s.sumcon)
    effect::set_condition(d, e3, Some(condition(sumcon)));
    // e3:SetCost(s.sumcost)
    effect::set_cost(d, e3, Some(cost_target(sumcost)));
    // e3:SetTarget(s.sumtg)
    effect::set_target(d, e3, Some(cost_target(sumtg)));
    // e3:SetOperation(s.sumop)
    effect::set_operation(d, e3, Some(operation(sumop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // e1:SetLabelObject(e3)
    effect::set_label_object(d, e1, Value::Effect(e3));
    // e2:SetLabelObject(e3)
    effect::set_label_object(d, e2, Value::Effect(e3));
    // destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetCondition(s.descon)
    effect::set_condition(d, e4, Some(condition(descon)));
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // e3:SetLabelObject(e4)
    effect::set_label_object(d, e3, Value::Effect(e4));
}
/// `s.chop1(e,tp,eg,ep,ev,re,r,rp)`
fn chop1(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():SetLabel(0)
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, lo, &[0]);
}
/// `s.chop2(e,tp,eg,ep,ev,re,r,rp)`
fn chop2(d: &mut Duel, a: &EffArgs) {
    // if rp~=tp or not re:IsHasType(EFFECT_TYPE_ACTIVATE) or not re:IsActiveType(TYPE_COUNTER) then return end
    if a.rp != a.tp || !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) || !effect::is_active_type(d, a.re.nil(), TYPE_COUNTER) {
        return;
    }
    // e:GetLabelObject():SetLabel(1)
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, lo, &[1]);
}
/// `s.sumcon(e,tp,eg,ep,ev,re,r,rp)`
fn sumcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabel()==1
    effect::get_label(d, a.e) == 1
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:GetSequence()<5 and c:IsControler(tp)
    card::get_sequence(d, c) < 5 && card::is_controler(d, c, tp)
}
/// `s.sumcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g,exg=Duel.GetReleaseGroup(tp):Split(aux.ReleaseCostFilter,nil,tp)
    let rg = duel::get_release_group(d, tp, false, false, None);
    let (mut g, exg) = group::split(d, &rg, &|d, c| aux::release_cost_filter(d, c, tp), NO_EX);
    // if chk==0 then return #g>0 and g:FilterCount(s.cfilter,nil,tp)+Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        let fc = group::filter_count(d, &g, &|d, c| cfilter(d, c, tp), NO_EX) as i64;
        return g.len() > 0 && fc + duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64 > 0;
    }
    // if #(exg-g)>0 and Duel.SelectYesNo(tp,aux.Stringid(59160188,2)) then
    let mut exg_minus_g = exg.clone();
    group::remove_card(&mut exg_minus_g, &g);
    if exg_minus_g.len() > 0 && duel::select_yes_no(d, tp, aux::stringid(59160188, 2)) {
        // g=g+(exg-g)
        group::add_card(&mut g, &exg_minus_g);
    }
    // local ct=g:FilterCount(Card.IsRace,nil,RACE_FAIRY)
    let ct = group::filter_count(d, &g, &|d, c| card::is_race(d, c, RACE_FAIRY), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // e:GetLabelObject():SetLabel(ct)
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, lo, &[ct as i64]);
    true
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, h, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(h.into()), 1, 0, 0);
    true
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp)`
fn sumop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,1,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 1, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SPECIAL + 1
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.TRUE,tp,0,LOCATION_ONFIELD,1,nil) and e:GetLabel()>0 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX) && effect::get_label(d, a.e) > 0;
    }
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local dg=g:Select(tp,1,e:GetLabel(),nil)
    let dg = group::select(d, &g, tp, 1, effect::get_label(d, a.e) as u16, false, NO_EX).nil();
    // Duel.HintSelection(dg)
    duel::hint_selection(d, dg.clone(), true);
    // Duel.Destroy(dg,REASON_EFFECT)
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
