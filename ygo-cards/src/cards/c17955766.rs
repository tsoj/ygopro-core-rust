//! Neo-Spacian Aqua Dolphin
use super::prelude::*;

pub const ID: u32 = 17955766;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Look at your opponent's hand and choose 1 monster, if you control a monster with ATK greater than or equal to the ATK of the chosen card, destroy the chosen card, and if you do, inflict 500 damage to your opponent, otherwise, take 500 damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.Discard())
    effect::set_cost(d, e1, Some(cost::discard(None, false, None, None, None)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.destg(e,tp,...,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_HAND as i32);
    true
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, a.tp, g.clone());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_OPPO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_OPPO as u64);
    // local sc=g:FilterSelect(tp,Card.IsMonster,1,1,nil):GetFirst()
    let sel = group::filter_select(d, &g, a.tp, &|d, c| card::is_monster(d, c), 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // local atk=sc:GetAttack()
        let atk = card::get_attack(d, sc);
        // if sc:IsAttackAbove(0) and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttackAbove,atk),tp,LOCATION_MZONE,0,1,nil) then
        if card::is_attack_above(d, sc, 0) && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_attack_above(d, c, atk)), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
            // if Duel.Destroy(sc,REASON_EFFECT)>0 then
            if duel::destroy(d, sc, REASON_EFFECT, None, None) > 0 {
                // Duel.Damage(1-tp,500,REASON_EFFECT)
                duel::damage(d, 1 - a.tp, 500, REASON_EFFECT, false, None);
            }
        } else {
            // Duel.Damage(tp,500,REASON_EFFECT)
            duel::damage(d, a.tp, 500, REASON_EFFECT, false, None);
        }
    }
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - a.tp);
}
