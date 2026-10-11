//! Malefic Tune
use super::prelude::*;

pub const ID: u32 = 44763696;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MALEFIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate to draw
    // local e1=Effect.CreateEffect(c)
    // e1:SetCategory(CATEGORY_DRAW)
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    // e1:SetCode(EVENT_DESTROYED)
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    // e1:SetCountLimit(1,id)
    // e1:SetCondition(s.condition)
    // e1:SetTarget(s.target)
    // e1:SetOperation(s.activate)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_DESTROYED);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // banish to search
    // local e2=Effect.CreateEffect(c)
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    // e2:SetCode(EVENT_DESTROYED)
    // e2:SetRange(LOCATION_GRAVE)
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    // e2:SetCountLimit(1,{id,1})
    // e2:SetCost(Cost.SelfBanish)
    // e2:SetCondition(s.thcd)
    // e2:SetTarget(s.thtg)
    // e2:SetOperation(s.thop)
    // c:RegisterEffect(e2)
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_condition(d, e2, Some(condition(thcd)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MALEFIC)
    //     and (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()~=tp))
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    card::is_set_card(d, c, &[SET_MALEFIC as u16])
        && (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) != tp))
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 2);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(2)
    duel::set_target_param(d, 2);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
// function s.condfilter(c,tp)
fn condfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MALEFIC)
    //     and not c:IsReason(REASON_BATTLE)
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    card::is_set_card(d, c, &[SET_MALEFIC as u16])
        && !card::is_reason(d, c, REASON_BATTLE)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
}
// function s.thcd(e,tp,eg,ep,ev,re,r,rp)
fn thcd(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.condfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| condfilter(d, c, tp), 1, NO_EX)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MALEFIC) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MALEFIC as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
