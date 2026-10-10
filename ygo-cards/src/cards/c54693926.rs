//! Dark Ruler No More
use super::prelude::*;

pub const ID: u32 = 54693926;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `function(te,rp,tp) return not te:IsMonsterEffect() end` (the chain limit of the Activate)
fn chain_limit(d: &mut Duel, a: Args) -> bool {
    !effect::is_monster_effect(d, a.e(0))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_negatable_monster), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    //     Duel.SetChainLimit(function(te,rp,tp) return not te:IsMonsterEffect() end)
    // end
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        duel::set_chain_limit(d, lua_fn(chain_limit));
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&card::is_negatable_monster), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Negate the effects of all face-up monsters your opponent currently controls, until the end of this turn
    // for tc in g:Iter() do tc:NegateEffects(c,RESET_PHASE|PHASE_END) end
    for tc in group::iter(d, &g) {
        card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), false, None);
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // Your opponent takes no damage for the rest of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_value(d, e1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
