//! Great Phantom Thief
//! 世紀の大泥棒
use super::prelude::*;

pub const ID: u32 = 10809984;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // handes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(s.drcon)
    effect::set_condition(d, e1, Some(condition(drcon)));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp
    a.ep != a.tp
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CODE as u64);
    // s.announce_filter={TYPE_EXTRA,OPCODE_ISTYPE,OPCODE_NOT}
    // local ac=Duel.AnnounceCard(tp,table.unpack(s.announce_filter))
    let announce_filter = [TYPE_EXTRA as u64, OPCODE_ISTYPE as u64, OPCODE_NOT as u64];
    crate::class::set_announce_filter(d, ID, &announce_filter);
    let ac = duel::announce_card(d, a.tp, &announce_filter);
    // Duel.SetTargetParam(ac)
    duel::set_target_param(d, ac as u32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD_FILTER)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, a.tp, ANNOUNCE_CARD_FILTER as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local ac=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM]);
    let ac = info[0].int();
    // local g=Duel.GetMatchingGroup(Card.IsCode,tp,0,LOCATION_HAND,nil,ac)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_code(d, c, &[ac as u32])), a.tp, 0, LOCATION_HAND, NO_EX);
    // local hg=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let hg = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    // Duel.ConfirmCards(tp,hg)
    duel::confirm_cards(d, a.tp, hg);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
    }
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - a.tp);
}
