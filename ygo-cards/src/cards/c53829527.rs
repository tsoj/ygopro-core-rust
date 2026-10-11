//! Crystron Cluster
//! クリストロン・クラスター
use super::prelude::*;

pub const ID: u32 = 53829527;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CRYSTRON], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // "Crystron" cards you control cannot be banished by your opponent's card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_REMOVE)
    effect::set_code(d, e1, EFFECT_CANNOT_REMOVE);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetTarget(s.rmlimit)
    effect::set_target(d, e1, Some(lua_fn(rmlimit)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle 1 "Crystron" card from your GY or banishment and destroy 1 face-up card on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn rmlimit(d: &mut Duel, a: Args) -> bool {
    // return c:IsSetCard(SET_CRYSTRON) and c:IsOnField() and c:IsFaceup() and c:IsControler(e:GetHandlerPlayer())
    //     and not c:IsImmuneToEffect(e) and r&REASON_EFFECT>0
    let e = a.e(0);
    let c = a.c(1);
    let r = a.int(3) as u32;
    card::is_set_card(d, c, &[SET_CRYSTRON as u16])
        && card::is_on_field(d, c)
        && card::is_faceup(d, c)
        && card::is_controler(d, c, effect::get_handler_player(d, e))
        && !card::is_immune_to_effect(d, c, e)
        && r & REASON_EFFECT > 0
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CRYSTRON) and c:IsFaceup() and c:IsAbleToDeck() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_CRYSTRON as u16]) && card::is_faceup(d, c) && card::is_able_to_deck(d, c) && !card::is_code(d, c, &[ID])
}
fn crystsyncfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CRYSTRON) and c:IsType(TYPE_SYNCHRO)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CRYSTRON as u16]) && card::is_type(d, c, TYPE_SYNCHRO)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=not c:IsStatus(STATUS_EFFECT_ENABLED) and c or nil
    let exc = if !card::is_status(d, c, STATUS_EFFECT_ENABLED) { Some(c) } else { None };
    // if chkc then return chkc:IsOnField() and chkc:IsFaceup() and (not exc or chkc~=exc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_faceup(d, chkc) && (exc.is_none() || exc != Some(chkc));
    }
    let ex: Exclude = match exc {
        Some(x) => x.into(),
        None => NO_EX,
    };
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,exc)
    //     and Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| card::is_faceup(d, cc)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, ex)
            && duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // local ct=Duel.IsExistingMatchingCard(s.crystsyncfilter,tp,LOCATION_MZONE,0,1,nil) and 2 or 1
    let ct: u16 = if duel::is_existing_matching_card(d, Some(&crystsyncfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX) { 2 } else { 1 };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,ct,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, cc| card::is_faceup(d, cc)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, ct, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(tdfilter)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // if Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 then return end
    if duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
