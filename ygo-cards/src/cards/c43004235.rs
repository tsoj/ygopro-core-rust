//! Plunder Patroll Parrrty
use super::prelude::*;

pub const ID: u32 = 43004235;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PLUNDER_PATROLL], self_equip_trap: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw, then shuffle card(s) from hand to deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equip this card to 1 of your "Plunder Patroll" monsters from GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1},EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e2, 1, ID, 1, EFFECT_COUNT_CODE_DUEL as u8);
    // e2:SetTarget(s.eqptg)
    effect::set_target(d, e2, Some(cost_target(eqptg)));
    // e2:SetOperation(s.eqpop)
    effect::set_operation(d, e2, Some(operation(eqpop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,...)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_PLUNDER_PATROLL),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsType,TYPE_EQUIP),tp,LOCATION_ONFIELD,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_EQUIP)), a.tp, LOCATION_ONFIELD, 0, NO_EX) as u32;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,ct+1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, ct + 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct+1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, (ct + 1) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,0,tp,ct,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, a.tp, ct as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsType,TYPE_EQUIP),p,LOCATION_ONFIELD,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_EQUIP)), p, LOCATION_ONFIELD, 0, NO_EX) as u32;
    // if Duel.Draw(p,ct+1,REASON_EFFECT)>0 and ct>0 then
    if duel::draw(d, p, ct + 1, REASON_EFFECT) > 0 && ct > 0 {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectMatchingCard(p,aux.TRUE,p,LOCATION_HAND,0,ct,ct,nil)
        let g = duel::select_matching_card(d, p, None, p, LOCATION_HAND, 0, ct as u16, ct as u16, false, NO_EX).nil();
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        // Duel.ShuffleDeck(p)
        duel::shuffle_deck(d, p);
    }
}
/// `s.eqptgfilter(c,tp)`
fn eqptgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_PLUNDER_PATROLL) and c:IsSummonLocation(LOCATION_EXTRA)
    //   and c:IsLocation(LOCATION_MZONE) and c:IsSummonPlayer(tp)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_PLUNDER_PATROLL as u16])
        && card::is_summon_location(d, c, LOCATION_EXTRA)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_summon_player(d, c, tp)
}
/// `s.eqptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return eg and eg:IsExists(s.eqptgfilter,1,nil,tp) and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 end
    if a.chk == Some(0) {
        let Some(eg) = a.eg.clone() else {
            return false;
        };
        return group::is_exists(d, &eg, &|d, c| eqptgfilter(d, c, a.tp), 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.eqpop(e,tp,eg,ep,ev,re,r,rp)`
fn eqpop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) or not eg then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let Some(eg) = a.eg.clone() else {
        return;
    };
    // local tc=eg:FilterSelect(tp,s.eqptgfilter,1,1,nil,tp):GetFirst()
    let sel = group::filter_select(d, &eg, a.tp, &|d, c| eqptgfilter(d, c, a.tp), 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc then
    if let Some(tc) = tc {
        // s.equipop(tc,e,tp,e:GetHandler())
        equipop(d, tc, a.e, a.tp, h);
    }
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc,nil,true) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, Some(true)) {
        return;
    }
    // Gains 500 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetValue(s.eqlimit)
    effect::set_value_fn(d, e2, effect_card_fn(eqlimit));
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetControler()==e:GetHandlerPlayer()
    card::get_controler(d, c) == effect::get_handler_player(d, e)
}
