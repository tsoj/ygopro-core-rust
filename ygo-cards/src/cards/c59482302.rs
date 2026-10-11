//! Ally Salvo
use super::prelude::*;

pub const ID: u32 = 59482302;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsReason(REASON_BATTLE)
    //     and e:GetHandler():GetReasonCard():IsAttribute(ATTRIBUTE_LIGHT)
    let h = effect::get_handler(d, a.e).nil();
    if !(card::is_location(d, h, LOCATION_GRAVE) && card::is_reason(d, h, REASON_BATTLE)) {
        return false;
    }
    let rc = card::get_reason_card(d, h).nil();
    card::is_attribute(d, rc, ATTRIBUTE_LIGHT)
}
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsCanBeEffectTarget(e)
    card::is_can_be_effect_target(d, c, Some(e))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and s.filter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && filter(d, chkc, a.e);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if #g>1 then
    if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local sg=g:Select(tp,2,2,nil)
        let sg = group::select(d, &g, a.tp, 2, 2, false, NO_EX).nil();
        // Duel.SetTargetCard(sg)
        duel::set_target_card(d, sg.clone());
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,2,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), 2, 0, 0);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    // if not g then return end
    let Some(g) = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned() else {
        return;
    };
    // local dg=g:Filter(Card.IsRelateToEffect,nil,e)
    let dg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // Duel.Destroy(dg,REASON_EFFECT)
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
