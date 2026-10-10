//! Doomcaliber Knight
use super::prelude::*;

pub const ID: u32 = 78700060;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // Negate
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_F);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsMonsterEffect()
    effect::is_monster_effect(d, a.re.nil())
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    if card::is_destructable(d, h, None) && card::is_relate_to_effect(d, h, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    if duel::get_current_chain(d, false) != a.ev as i32 + 1 {
        return;
    }
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let h = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, h, re) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
