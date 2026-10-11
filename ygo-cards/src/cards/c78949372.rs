//! Majespecter Supercell
use super::prelude::*;

pub const ID: u32 = 78949372;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAJESPECTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // inactivatable
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_INACTIVATE)
    effect::set_code(d, e4, EFFECT_CANNOT_INACTIVATE);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCondition(s.tgcon)
    effect::set_condition(d, e4, Some(condition(tgcon)));
    // e4:SetValue(s.effectfilter)
    effect::set_value_fn(d, e4, lua_fn(effectfilter_value));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_CANNOT_DISEFFECT)
    effect::set_code(d, e5, EFFECT_CANNOT_DISEFFECT);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCondition(s.tgcon)
    effect::set_condition(d, e5, Some(condition(tgcon)));
    // e5:SetValue(s.effectfilter)
    effect::set_value_fn(d, e5, lua_fn(effectfilter_value));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAJESPECTER) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_MAJESPECTER as u16]) && card::is_able_to_deck(d, c)
}
/// `s.drtg(e,tp,...,chk,chkc)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,5,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 5, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,5,5,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 5, 5, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,5,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 5, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,...)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).remove(0).as_group().cloned();
    // if not tg or tg:FilterCount(Card.IsRelateToEffect,nil,e)~=5 then return end
    let Some(tg) = tg else { return };
    let e = a.e;
    if group::filter_count(d, &tg, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX) != 5 {
        return;
    }
    // Duel.SendtoDeck(tg,nil,SEQ_DECKTOP,REASON_EFFECT)
    duel::send_to_deck(d, tg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // if g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, a.tp);
    }
    // local ct=g:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let ct = group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if ct==5 then
    if ct == 5 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
}
/// `s.tgcon(e)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsSetCard,e:GetHandlerPlayer(),LOCATION_PZONE,0,1,nil,SET_MAJESPECTER)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_set_card(d, c, &[SET_MAJESPECTER as u16])), p, LOCATION_PZONE, 0, 1, NO_EX)
}
/// `s.effectfilter(e,ct)`, as the value function of EFFECT_CANNOT_INACTIVATE / EFFECT_CANNOT_DISEFFECT
fn effectfilter_value(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let ct = a.int(1);
    effectfilter(d, e, ct)
}
fn effectfilter(d: &mut Duel, e: EffectId, ct: i64) -> bool {
    // local p=e:GetHandlerPlayer()
    let p = effect::get_handler_player(d, e);
    // local te,tp,loc=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT,CHAININFO_TRIGGERING_PLAYER,CHAININFO_TRIGGERING_LOCATION)
    let info = duel::get_chain_info(d, ct as u8, &[CHAININFO_TRIGGERING_EFFECT, CHAININFO_TRIGGERING_PLAYER, CHAININFO_TRIGGERING_LOCATION]);
    // local tc=te:GetHandler()
    let te = info[0].as_effect().nil();
    let tc = effect::get_handler(d, te).nil();
    // return p==tp and (loc&LOCATION_ONFIELD)~=0 and tc:IsSetCard(SET_MAJESPECTER) and tc~=e:GetHandler()
    if !matches!(info[1], Value::Int(v) if v == p as i64) {
        return false;
    }
    let loc = info[2].int() as u32;
    (loc & LOCATION_ONFIELD) != 0 && card::is_set_card(d, tc, &[SET_MAJESPECTER as u16]) && Some(tc) != effect::get_handler(d, e)
}
