//! Lavalval Chain
use super::prelude::*;

pub const ID: u32 = 34086406;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>1 and Duel.IsExistingMatchingCard(Card.IsMonster,tp,LOCATION_DECK,0,1,nil)
    let b2 = duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 1
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_monster(d, c)), tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    } else if op == Some(2) {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Send 1 card from your Deck to the GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGrave,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then Duel.SendtoGrave(g,REASON_EFFECT) end
        if !g.is_empty() {
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    } else if op == 2 {
        // Place 1 monster from your Deck on top of your Deck
        // local ct=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
        let ct = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
        // if ct==0 then return end
        if ct == 0 {
            return;
        }
        // if ct==1 then return Duel.ConfirmDecktop(tp,1) end
        if ct == 1 {
            duel::confirm_decktop(d, tp, 1);
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
        // local tc=Duel.SelectMatchingCard(tp,Card.IsMonster,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_monster(d, c)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &g);
        // if tc then
        if let Some(tc) = tc {
            // Duel.ShuffleDeck(tp)
            duel::shuffle_deck(d, tp);
            // Duel.MoveSequence(tc,0)
            duel::move_sequence(d, tc, 0, None);
            // Duel.ConfirmDecktop(tp,1)
            duel::confirm_decktop(d, tp, 1);
        }
    }
}
