//! Evil Eye Repose
use super::prelude::*;

pub const ID: u32 = 79400597;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_EVIL_EYE_SELENE], listed_series: &[SET_EVIL_EYE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish the opponent's monster that your "Evil Eye" monster equipped with "Evil Eye of Selene" attacked
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.rmcon)
    effect::set_condition(d, e2, Some(condition(rmcon)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Return up to 3 of your banished "Evil Eye" cards to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.rtgcon)
    effect::set_condition(d, e3, Some(condition(rtgcon)));
    // e3:SetTarget(s.rtgtg)
    effect::set_target(d, e3, Some(cost_target(rtgtg)));
    // e3:SetOperation(s.rtgop)
    effect::set_operation(d, e3, Some(operation(rtgop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EVIL_EYE) and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_EVIL_EYE as u16]) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local op=0
    let mut op: i64 = 0;
    // if Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_GRAVE,0,1,nil)
    //     and Duel.IsPlayerCanDraw(tp,1) and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if duel::is_existing_matching_card(d, Some(&costfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_player_can_draw(d, tp, 1)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
    {
        // op=1
        op = 1;
    }
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==0 then
    if op == 0 {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    } else if op == 1 {
        // e:SetCategory(CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DRAW);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&costfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // Duel.Remove(g,POS_FACEUP,REASON_COST)
        duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    }
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // Duel.SetTargetPlayer(tp)
        duel::set_target_player(d, a.tp);
        // Duel.SetTargetParam(1)
        duel::set_target_param(d, 1);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()==0 then return end
    if effect::get_label(d, a.e) == 0 {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ac,bc=Duel.GetBattleMonster(tp)
    let (ac, bc) = duel::get_battle_monster(d, a.tp);
    // if ac and ac==Duel.GetAttacker() and bc and ac:IsSetCard(SET_EVIL_EYE)
    //     and ac:GetEquipGroup():IsExists(Card.IsCode,1,nil,CARD_EVIL_EYE_SELENE) then
    if let (Some(ac), Some(bc)) = (ac, bc) {
        if Some(ac) == duel::get_attacker(d) && card::is_set_card(d, ac, &[SET_EVIL_EYE as u16]) {
            let eq = card::get_equip_group(d, ac);
            if group::is_exists(d, &eq, &|d, x| card::is_code(d, x, &[CARD_EVIL_EYE_SELENE]), 1, NO_EX) {
                // e:SetLabelObject(bc)
                effect::set_label_object(d, a.e, Value::Card(bc));
                // return true
                return true;
            }
        }
    }
    // return false
    false
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card();
    // if chk==0 then return bc:IsAbleToRemove() end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, bc.nil(), None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,bc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(bc.nil())), 1, a.tp, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card().nil();
    // if bc:IsRelateToBattle() and bc:IsControler(1-tp) then
    if card::is_relate_to_battle(d, bc) && card::is_controler(d, bc, 1 - a.tp) {
        // Duel.Remove(bc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, bc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.rtgcon(e,tp,eg,ep,ev,re,r,rp)`
fn rtgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_EFFECT) and c:IsPreviousLocation(LOCATION_STZONE)
    card::is_reason(d, c, REASON_EFFECT) && card::is_previous_location(d, c, LOCATION_STZONE)
}
/// `s.rtgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
/// The Lua function has no `chkc` parameter, so `chkc` is the global nil and its branch never runs.
fn rtgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSetCard,SET_EVIL_EYE),tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x) && card::is_set_card(d, x, &[SET_EVIL_EYE as u16])), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsSetCard,SET_EVIL_EYE),tp,LOCATION_REMOVED,0,1,3,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, x| card::is_faceup(d, x) && card::is_set_card(d, x, &[SET_EVIL_EYE as u16])), a.tp, LOCATION_REMOVED, 0, 1, 3, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, a.tp, 0);
    true
}
/// `s.rtgop(e,tp,eg,ep,ev,re,r,rp)`
fn rtgop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoGrave(tg,REASON_EFFECT|REASON_RETURN)
        duel::send_to_grave(d, &tg, REASON_EFFECT | REASON_RETURN, None, None);
    }
}
