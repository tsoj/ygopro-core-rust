//! The Sanctum of Parshath
use super::prelude::*;

pub const ID: u32 = 15449853;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SANCTUARY_SKY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // name change
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e2, EFFECT_CHANGE_CODE);
    // e2:SetRange(LOCATION_SZONE|LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_SZONE | LOCATION_GRAVE);
    // e2:SetValue(CARD_SANCTUARY_SKY)
    effect::set_value(d, e2, CARD_SANCTUARY_SKY as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atk/def increase
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_FAIRY))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_FAIRY))));
    // e3:SetValue(300)
    effect::set_value(d, e3, 300);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // cannot be target/indestructable
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e5, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e5:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e5, EFFECT_INDESTRUCTABLE_EFFECT);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetTargetRange(LOCATION_SZONE,LOCATION_SZONE)
    effect::set_target_range(d, e5, LOCATION_SZONE, LOCATION_SZONE);
    // e5:SetTarget(aux.TargetBoolFunction(Card.IsPosition,POS_FACEDOWN))
    effect::set_target(d, e5, Some(effect_card_fn(|d, _e, c| card::is_position(d, c, POS_FACEDOWN))));
    // e5:SetValue(1)
    effect::set_value(d, e5, 1);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e5:Clone()
    let e6 = effect::clone(d, e5);
    // e6:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e6, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e6:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e6, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // send cards to deck
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e7, CATEGORY_TODECK);
    // e7:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e7, EFFECT_TYPE_IGNITION);
    // e7:SetRange(LOCATION_SZONE)
    effect::set_range(d, e7, LOCATION_SZONE);
    // e7:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e7, EFFECT_FLAG_CARD_TARGET, 0);
    // e7:SetCountLimit(1)
    effect::set_count_limit(d, e7, 1, 0, 0, 0);
    // e7:SetTarget(s.tdtg)
    effect::set_target(d, e7, Some(cost_target(tdtg)));
    // e7:SetOperation(s.tdop)
    effect::set_operation(d, e7, Some(operation(tdop)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
fn tdfilter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return (c:IsRace(RACE_FAIRY) or c:IsType(TYPE_COUNTER)) and c:IsAbleToDeck() and (not e or c:IsCanBeEffectTarget(e))
    (card::is_race(d, c, RACE_FAIRY) || card::is_type(d, c, TYPE_COUNTER))
        && card::is_able_to_deck(d, c)
        && e.map_or(true, |e| card::is_can_be_effect_target(d, c, Some(e)))
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE,0,nil,e)
    let e = a.e;
    let g = duel::get_matching_group(d, Some(&|d, c| tdfilter(d, c, Some(e))), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return g:GetClassCount(Card.GetCode)>=3 end
    if a.chk == Some(0) {
        return group::get_class_count(d, &g, &|d, c| card::get_code(d, c).first().copied().unwrap_or(0) as i64) >= 3;
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_TODECK)
    let dn = aux::dncheck();
    let tg = aux::select_unselect_group(d, &g, Some(e), a.tp, Some(3), Some(3), Some(&dn), 1, a.tp, Some(HINTMSG_TODECK as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,#tg,0,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg), n, 0, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tg=Duel.GetTargetCards(e)
    // if not tg or #tg==0 then return end
    let Some(tg) = duel::get_target_cards(d, a.e) else {
        return;
    };
    if tg.is_empty() {
        return;
    }
    // if Duel.SendtoDeck(tg,nil,SEQ_DECKTOP,REASON_EFFECT)==0 then return end
    if duel::send_to_deck(d, tg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // local ct=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_DECK)
    let op = duel::get_operated_group(d);
    let ct = group::filter_count(d, &op, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // if ct>0 then Duel.SortDecktop(tp,tp,ct) end
    if ct > 0 {
        duel::sort_decktop(d, tp, tp, ct as u16);
    }
}
