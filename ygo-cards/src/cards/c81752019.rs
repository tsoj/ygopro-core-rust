//! Flower Cardian Pine
use super::prelude::*;

pub const ID: u32 = 81752019;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FLOWER_CARDIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw 1 card, show it, then send it to the GY unless it is a "Flower Cardian" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.nsdrtg)
    effect::set_target(d, e1, Some(cost_target(nsdrtg)));
    // e1:SetOperation(s.nsdrop)
    effect::set_operation(d, e1, Some(operation(nsdrop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.desdrcon)
    effect::set_condition(d, e2, Some(condition(desdrcon)));
    // e2:SetTarget(s.desdrtg)
    effect::set_target(d, e2, Some(cost_target(desdrtg)));
    // e2:SetOperation(s.desdrop)
    effect::set_operation(d, e2, Some(operation(desdrop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.nsdrtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn nsdrtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.nsdrop(e,tp,eg,ep,ev,re,r,rp)`
fn nsdrop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, dn) = (info[0].int() as u8, info[1].int() as u32);
    // if Duel.Draw(p,d,REASON_EFFECT)>0 then
    if duel::draw(d, p, dn, REASON_EFFECT) > 0 {
        // local dc=Duel.GetOperatedGroup():GetFirst()
        let og = duel::get_operated_group(d);
        let dc = group::get_first(d, &og).nil();
        // Duel.ConfirmCards(1-tp,dc)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(dc));
        // if not (dc:IsSetCard(SET_FLOWER_CARDIAN) and dc:IsMonster()) then
        if !(card::is_set_card(d, dc, &[SET_FLOWER_CARDIAN as u16]) && card::is_monster(d, dc)) {
            // Duel.SendtoGrave(dc,REASON_EFFECT)
            duel::send_to_grave(d, CardSet::from_card(dc), REASON_EFFECT, None, None);
        }
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
    }
}
/// `s.desdrcon(e,tp,eg,ep,ev,re,r,rp)`
fn desdrcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // return c:IsReason(REASON_BATTLE) or (rp==1-tp and c:IsReason(REASON_DESTROY) and c:IsReason(REASON_EFFECT)
    //     and c:IsPreviousControler(tp))
    card::is_reason(d, c, REASON_BATTLE)
        || (a.rp == 1 - tp && card::is_reason(d, c, REASON_DESTROY) && card::is_reason(d, c, REASON_EFFECT) && card::is_previous_controler(d, c, tp))
}
/// `s.desdrtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desdrtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.desdrop(e,tp,eg,ep,ev,re,r,rp)`
fn desdrop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, dn) = (info[0].int() as u8, info[1].int() as u32);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, p, dn, REASON_EFFECT);
}
