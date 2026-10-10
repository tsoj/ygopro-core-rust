//! Phantom Fortress Enterblathnir
use super::prelude::*;

pub const ID: u32 = 95113856;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 9 monsters
    // Xyz.AddProcedure(c,nil,9,2)
    xyz::add_procedure(d, c, None, Some(9), 2, None, None, None, None, false, None);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `Card.IsAbleToRemove` as a filter (no extra arguments)
fn able_to_remove(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&able_to_remove),
            a.tp,
            0,
            LOCATION_ONFIELD | LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK,
            1,
            NO_EX,
        );
    }
    // local b1=Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&able_to_remove), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_HAND,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&able_to_remove), a.tp, 0, LOCATION_HAND, 1, NO_EX);
    // local b3=Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,1,nil)
    let b3 = duel::is_existing_matching_card(d, Some(&able_to_remove), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    // local b4=Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_DECK,1,nil)
    let b4 = duel::is_existing_matching_card(d, Some(&able_to_remove), a.tp, 0, LOCATION_DECK, 1, NO_EX);
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)}, {b3,aux.Stringid(id,3)}, {b4,aux.Stringid(id,4)})
    let op = duel::select_effect(
        d,
        a.tp,
        &[
            (b1, aux::stringid(ID, 1)),
            (b2, aux::stringid(ID, 2)),
            (b3, aux::stringid(ID, 3)),
            (b4, aux::stringid(ID, 4)),
        ],
    );
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64)]);
    // local loc=(op==1 and LOCATION_ONFIELD) or (op==2 and LOCATION_HAND) or (op==3 and LOCATION_GRAVE) or (op==4 and LOCATION_DECK)
    let loc = match op {
        Some(1) => LOCATION_ONFIELD,
        Some(2) => LOCATION_HAND,
        Some(3) => LOCATION_GRAVE,
        Some(4) => LOCATION_DECK,
        _ => 0,
    };
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,loc)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, loc as i32);
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Banish 1 card your opponent controls
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&able_to_remove), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    } else if op == 2 {
        // Banish 1 random card from your opponent's hand
        // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_HAND,nil)
        let g = duel::get_matching_group(d, Some(&able_to_remove), a.tp, 0, LOCATION_HAND, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // local sg=g:RandomSelect(tp,1)
            let sg = group::random_select(d, &g, a.tp, 1);
            // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    } else if op == 3 {
        // Banish 1 card from your opponent's GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&able_to_remove), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    } else if op == 4 {
        // Banish the top card of your opponent's Deck, face-up
        // local g=Duel.GetDecktopGroup(1-tp,1)
        let g = duel::get_decktop_group(d, 1 - a.tp, 1);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.DisableShuffleCheck()
            duel::disable_shuffle_check(d, true);
            // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
