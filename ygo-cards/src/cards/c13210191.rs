//! Storm
use super::prelude::*;

pub const ID: u32 = 13210191;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_ONFIELD,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // local g1=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,LOCATION_ONFIELD,0,c)
    let mut g1 = duel::get_matching_group(d, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, 0, c.into());
    // local g2=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let g2 = duel::get_matching_group(d, Some(&card::is_spell_trap), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // local ct1=#g1
    // local ct2=#g2
    let ct1 = g1.len();
    let ct2 = g2.len();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g1,ct1+((ct1>ct2) and ct2 or ct1),0,0)
    let extra = if ct1 > ct2 { ct2 } else { ct1 };
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1), (ct1 + extra) as u8, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g1=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,LOCATION_ONFIELD,0,e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let g1 = duel::get_matching_group(d, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, 0, c.into());
    // local ct1=Duel.Destroy(g1,REASON_EFFECT)
    let ct1 = duel::destroy(d, g1, REASON_EFFECT, None, None);
    // if ct1==0 then return end
    if ct1 == 0 {
        return;
    }
    // local g2=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let g2 = duel::get_matching_group(d, Some(&card::is_spell_trap), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // local ct2=#g2
    let ct2 = g2.len() as i32;
    // if ct2==0 then return end
    if ct2 == 0 {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if ct2<=ct1 then
    if ct2 <= ct1 {
        // Duel.Destroy(g2,REASON_EFFECT)
        duel::destroy(d, g2, REASON_EFFECT, None, None);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g3=g2:Select(tp,ct1,ct1,nil)
        let g3 = group::select(d, &g2, a.tp, ct1 as u16, ct1 as u16, false, NO_EX).nil();
        // Duel.HintSelection(g3)
        duel::hint_selection(d, g3.clone(), true);
        // Duel.Destroy(g3,REASON_EFFECT)
        duel::destroy(d, g3, REASON_EFFECT, None, None);
    }
}
