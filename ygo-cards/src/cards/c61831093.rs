//! Greenkappa
use super::prelude::*;

pub const ID: u32 = 61831093;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFacedown() and c:IsCanBeEffectTarget(e)
    card::is_facedown(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and s.filter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && filter(d, chkc, a.e);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_SZONE,LOCATION_SZONE,nil,e)
    let mut g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e)), a.tp, LOCATION_SZONE, LOCATION_SZONE, NO_EX);
    // if #g<2 then
    if g.len() < 2 {
        // g:Clear()
        g = CardSet::new();
        // Duel.SetTargetCard(g)
        duel::set_target_card(d, g);
        // return
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local sg=g:Select(tp,2,2,nil)
    let sg = group::select(d, &g, a.tp, 2, 2, false, NO_EX).nil();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), 2, 0, 0);
    true
}
/// `s.dfilter(c,e)`
fn dfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsRelateToEffect(e) and c:IsFacedown()
    card::is_relate_to_effect(d, c, e) && card::is_facedown(d, c)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    // if not tg then return end
    let Some(tg) = info[0].as_group().cloned() else { return };
    // local dg=tg:Filter(s.dfilter,nil,e)
    let dg = group::filter(d, &tg, &|d, c| dfilter(d, c, a.e), NO_EX);
    // Duel.Destroy(dg,REASON_EFFECT)
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
