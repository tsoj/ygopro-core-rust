//! Magical Musketeer Kidbrave
//! 魔弾の射手 ザ・キッド
use super::prelude::*;

pub const ID: u32 = 5230799;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MAGICAL_MUSKET],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate "Magical Musket" Spell/Traps from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_QP_ACT_IN_NTPHAND)
    effect::set_code(d, e1, EFFECT_QP_ACT_IN_NTPHAND);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e1, LOCATION_HAND, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MAGICAL_MUSKET))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16]))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Draw 2 cards
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.drcon)
    effect::set_condition(d, e3, Some(condition(drcon)));
    // e3:SetCost(s.drcost)
    effect::set_cost(d, e3, Some(cost_target(drcost)));
    // e3:SetTarget(s.drtg)
    effect::set_target(d, e3, Some(cost_target(drtg)));
    // e3:SetOperation(s.drop)
    effect::set_operation(d, e3, Some(operation(drop_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.GetCurrentPhase()&(PHASE_DAMAGE|PHASE_DAMAGE_CAL)>0 or not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return false end
    if duel::get_current_phase(d) & (PHASE_DAMAGE | PHASE_DAMAGE_CAL) > 0 {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local p,seq=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_CONTROLER,CHAININFO_TRIGGERING_SEQUENCE)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_SEQUENCE]);
    let p = info[0].int() as u8;
    let seq = info[1].int() as u32;
    // return c:IsColumn(seq,p,LOCATION_SZONE)
    card::is_column(d, c, seq, Some(p), Some(LOCATION_SZONE))
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAGICAL_MUSKET) and c:IsDiscardable()
    card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16]) && card::is_discardable(d, c, None, None)
}
/// `s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
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
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
