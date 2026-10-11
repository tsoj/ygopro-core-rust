//! Commander of Swords
use super::prelude::*;

pub const ID: u32 = 12197543;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_HANDES+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_DRAW)
    effect::set_code(d, e1, EVENT_DRAW);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and r==REASON_RULE
    a.ep == 1 - a.tp && a.r == REASON_RULE
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return c:IsLocation(LOCATION_HAND) and not c:IsPublic() and (not e or c:IsRelateToEffect(e))
    card::is_location(d, c, LOCATION_HAND)
        && !card::is_public(d, c)
        && match e {
            None => true,
            Some(e) => card::is_relate_to_effect(d, c, e),
        }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(s.filter,1,nil) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| filter(d, c, None), 1, NO_EX);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local rg=eg:Filter(s.filter,nil,e)
    let eg = a.eg.clone().nil();
    let rg = group::filter(d, &eg, &|d, c| filter(d, c, Some(e)), NO_EX);
    // if #rg==0 then return end
    if rg.is_empty() {
        return;
    }
    // Duel.ConfirmCards(tp,rg)
    duel::confirm_cards(d, tp, rg.clone());
    // if not rg:IsExists(Card.IsSpellTrap,1,nil) then return Duel.ShuffleHand(1-tp) end
    if !group::is_exists(d, &rg, &|d, c| card::is_spell_trap(d, c), 1, NO_EX) {
        duel::shuffle_hand(d, 1 - tp);
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // local b1=rg:FilterCount(Card.IsDiscardable,nil,REASON_EFFECT)==#rg
    let b1 = group::filter_count(d, &rg, &|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None), NO_EX) == rg.len();
    // local b2=#g>0
    let b2 = !g.is_empty();
    // if not (b1 or b2) then return Duel.ShuffleHand(1-tp) end
    if !(b1 || b2) {
        duel::shuffle_hand(d, 1 - tp);
        return;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    if op == Some(1) {
        // Duel.SendtoGrave(rg,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, rg, REASON_EFFECT | REASON_DISCARD, None, None);
    } else if op == Some(2) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local dg=g:Select(tp,1,1,nil)
        let dg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.HintSelection(dg,true)
        duel::hint_selection(d, dg.clone(), true);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
    }
}
