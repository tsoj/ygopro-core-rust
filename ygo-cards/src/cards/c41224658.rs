//! Ambitious Gofer
use super::prelude::*;

pub const ID: u32 = 41224658;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_MZONE,1,2,nil)
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_MZONE, 1, 2, false, NO_EX).nil();
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)==Duel.GetMatchingGroupCount(Card.IsPublic,tp,0,LOCATION_HAND,nil)
    //     and not Duel.IsExistingMatchingCard(Card.IsMonster,tp,0,LOCATION_HAND,1,nil) then
    let hand = duel::get_field_group_count(d, tp, 0, LOCATION_HAND) as usize;
    let public = duel::get_matching_group_count(d, Some(&|d, c| card::is_public(d, c)), tp, 0, LOCATION_HAND, NO_EX);
    if hand == public && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_monster(d, c)), tp, 0, LOCATION_HAND, 1, NO_EX) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
        let n = g.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    }
    true
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsPublic() and c:IsMonster()
    !card::is_public(d, c) && card::is_monster(d, c)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsChainDisablable(0) then
    if duel::is_chain_disablable(d, 0) {
        // local sel=1
        // local cg=Duel.GetMatchingGroup(s.cfilter,tp,0,LOCATION_HAND,nil)
        let cg = duel::get_matching_group(d, Some(&cfilter), tp, 0, LOCATION_HAND, NO_EX);
        // Duel.Hint(HINT_SELECTMSG,1-tp,aux.Stringid(id,1))
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, aux::stringid(ID, 1));
        // if #cg>0 then sel=Duel.SelectOption(1-tp,1213,1214)
        // else sel=Duel.SelectOption(1-tp,1214)+1 end
        let sel = if !cg.is_empty() {
            duel::select_option(d, 1 - tp, true, &[1213, 1214])
        } else {
            duel::select_option(d, 1 - tp, true, &[1214]) + 1
        };
        // if sel==0 then
        if sel == 0 {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_CONFIRM)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_CONFIRM as u64);
            // local sg=cg:Select(1-tp,1,1,nil)
            let sg = group::select(d, &cg, 1 - tp, 1, 1, false, NO_EX).nil();
            // Duel.ConfirmCards(tp,sg)
            duel::confirm_cards(d, tp, sg);
            // Duel.ShuffleHand(1-tp)
            duel::shuffle_hand(d, 1 - tp);
            // Duel.NegateEffect(0)
            duel::negate_effect(d, 0);
            // return
            return;
        }
    }
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
