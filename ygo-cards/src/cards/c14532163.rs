//! Lightning Storm
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 14532163;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
}
/// `s.descon(e,...)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_ONFIELD,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.destg(e,...,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g1=Duel.GetMatchingGroup(Card.IsAttackPos,tp,0,LOCATION_MZONE,nil)
    let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_attack_pos(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // local b1=#g1>0
    let b1 = !g1.is_empty();
    // local b2=#g2>0
    let b2 = !g2.is_empty();
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local cd=e:GetChainData()
    // cd.choice=Duel.SelectEffect(tp, {b1,aux.Stringid(id,0)}, {b2,aux.Stringid(id,1)})
    let choice = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1))]);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(choice.map_or(0, |x| x as i64)));
    // local g=(cd.choice==1 and g1 or g2)
    let g = if choice == Some(1) { g1 } else { g2 };
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local cd=e:GetChainData()
    // if cd.choice==1 then
    let choice = chain::effect_data(d, a.e, None, chain::field::CARD).int();
    if choice == 1 {
        // local g=Duel.GetMatchingGroup(Card.IsAttackPos,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_attack_pos(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
        // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
        if !g.is_empty() {
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    } else if choice == 2 {
        // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
        // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
        if !g.is_empty() {
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
