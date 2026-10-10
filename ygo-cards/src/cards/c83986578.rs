//! King Tiger Wanghu
use super::prelude::*;

pub const ID: u32 = 83986578;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
}
/// `function s.filter(c,e) return c:IsFaceup() and c:IsAttackBelow(1400) and (not e or c:IsRelateToEffect(e)) end`
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    if !(card::is_faceup(d, c) && card::is_attack_below(d, c, 1400)) {
        return false;
    }
    match e {
        None => true,
        Some(e) => card::is_relate_to_effect(d, c, e),
    }
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local eg = a.eg (the event group)
    let eg = a.eg.clone().nil();
    // return eg:IsExists(s.filter,1,nil) and not eg:IsContains(e:GetHandler()) and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED)
    if !group::is_exists(d, &eg, &|d, c| filter(d, c, None), 1, NO_EX) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if group::is_contains(&eg, h) {
        return false;
    }
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=eg:Filter(s.filter,nil)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, None), NO_EX);
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, &eg);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local g=eg:Filter(s.filter,nil,e)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, Some(a.e)), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
