//! Supply
use super::prelude::*;

pub const ID: u32 = 44072894;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,e)
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsReason(REASON_FUSION) and c:IsMonster() and c:IsAbleToHand() and c:IsCanBeEffectTarget(e)
    card::is_reason(d, c, REASON_FUSION) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.e);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil,e)
    let e = a.e;
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g>=2 then
    if g.len() >= 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,2,2,nil)
        let sg = group::select(d, &g, a.tp, 2, 2, false, NO_EX).nil();
        // Duel.SetTargetCard(sg)
        duel::set_target_card(d, &sg);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,sg,2,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(sg), 2, 0, 0);
    }
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    // if not g then return end
    let Some(g) = info[0].as_group().cloned() else {
        return;
    };
    // g=g:Filter(Card.IsRelateToEffect,nil,e)
    let e = a.e;
    let g = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    // if #g==2 then
    if g.len() == 2 {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
