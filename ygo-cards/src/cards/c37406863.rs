//! Fengsheng Mirror
use super::prelude::*;

pub const ID: u32 = 37406863;

pub const INFO: ScriptInfo = ScriptInfo { listed_card_types: &[TYPE_SPIRIT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Discard 1 spirit monster from the opponent's hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local g=Duel.GetFieldGroup(p,0,LOCATION_HAND)
    let g = duel::get_field_group(d, p, 0, LOCATION_HAND);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(p,g)
        duel::confirm_cards(d, p, g.clone());
        // local tg=g:Filter(Card.IsType,nil,TYPE_SPIRIT)
        let tg = group::filter(d, &g, &|d, c| card::is_type(d, c, TYPE_SPIRIT), NO_EX);
        // if #tg>0 then
        if !tg.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TOGRAVE as u64);
            // local sg=tg:Select(p,1,1,nil)
            let sg = group::select(d, &tg, p, 1, 1, false, NO_EX).nil();
            // Duel.SendtoGrave(sg,REASON_DISCARD|REASON_EFFECT)
            duel::send_to_grave(d, sg, REASON_DISCARD | REASON_EFFECT, None, None);
        }
        // Duel.ShuffleHand(1-p)
        duel::shuffle_hand(d, 1 - p);
    }
}
