//! Enneacraft Reset
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 19504025;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[17621695], listed_series: &[SET_ENNEACRAFT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place 1 "Enneapolis" from your Deck or GY face-up in your Field Zone, or if you control "Enneapolis", you can negate the effects of face-up cards your opponent controls up to the number of cards in your Pendulum Zones instead
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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If your opponent activates a card or effect while this card is in your GY: You can banish this card; change "Enneacraft" monsters you control to face-down Defense Position up to the number of cards in your Pendulum Zones
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp end)
    effect::set_condition(d, e2, Some(condition(opp_chaining_cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.postg)
    effect::set_target(d, e2, Some(cost_target(postg)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp end`
fn opp_chaining_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(17621695) and not c:IsForbidden()
    card::is_code(d, c, &[17621695]) && !card::is_forbidden(d, c)
}
/// `Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,17621695),tp,LOCATION_ONFIELD,0,1,nil)
///   and Duel.GetFieldGroupCount(tp,LOCATION_PZONE,0)>0
///   and Duel.IsExistingMatchingCard(Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,nil)`
fn disable_chk(d: &mut Duel, tp: u8) -> bool {
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[17621695]))), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::get_field_group_count(d, tp, LOCATION_PZONE, 0) > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_negatable(d, c)), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local disable_chk=...
        // return disable_chk or Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil)
        return disable_chk(d, tp) || duel::is_existing_matching_card(d, Some(&plfilter), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local place_chk=Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.plfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil)
    let place_chk = duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, c| plfilter(d, c))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    // local pzone_card_count=Duel.GetFieldGroupCount(tp,LOCATION_PZONE,0)
    let pzone_card_count = duel::get_field_group_count(d, tp, LOCATION_PZONE, 0);
    // local disable_chk=...
    let disable_chk = disable_chk(d, tp);
    // local op=1
    let mut op: usize = 1;
    // if disable_chk then
    if disable_chk {
        // op=Duel.SelectEffect(tp,{place_chk,aux.Stringid(id,2)},{disable_chk,aux.Stringid(id,3)})
        op = duel::select_effect(d, tp, &[(place_chk, aux::stringid(ID, 2)), (disable_chk, aux::stringid(ID, 3))]).nil();
    }
    // if op==1 then
    if op == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.plfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| plfilter(d, c))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc then
        if let Some(sc) = sc {
            // Duel.MoveToField(sc,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
            duel::move_to_field(d, sc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
        }
    } else if op == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,pzone_card_count,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_negatable(d, c)), tp, 0, LOCATION_ONFIELD, 1, pzone_card_count as u16, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // for sc in g:Iter() do
        for sc in group::iter(d, &g) {
            // Negate their effects
            // sc:NegateEffects(c,nil,true)
            card::negate_effects(d, sc, c, None, true, None);
        }
    }
}
/// `s.posfilter(c)`
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ENNEACRAFT) and c:IsCanTurnSet()
    card::is_set_card(d, c, &[SET_ENNEACRAFT as u16]) && card::is_can_turn_set(d, c)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_PZONE,0)>0
    //   and Duel.IsExistingMatchingCard(s.posfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_PZONE, 0) > 0
            && duel::is_existing_matching_card(d, Some(&posfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local max_count=Duel.GetFieldGroupCount(tp,LOCATION_PZONE,0)
    let max_count = duel::get_field_group_count(d, tp, LOCATION_PZONE, 0);
    // if max_count==0 then return end
    if max_count == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.posfilter,tp,LOCATION_MZONE,0,1,max_count,nil)
    let g = duel::select_matching_card(d, tp, Some(&posfilter), tp, LOCATION_MZONE, 0, 1, max_count as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
