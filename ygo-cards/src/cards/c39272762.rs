//! Neo Galaxy-Eyes Photon Dragon
use super::prelude::*;

pub const ID: u32 = 39272762;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_GALAXYEYES_P_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(8), 3, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // negate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(negcon)));
    effect::set_operation(d, e1, Some(operation(negop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e2, effect_card_value(valcheck));
    effect::set_label_object(d, e2, Value::Effect(e1));
    card::register_effect(d, c, e2, false);
    // attack up
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e3, Some(cost_target(attg)));
    effect::set_operation(d, e3, Some(operation(atop)));
    card::register_effect(d, c, e3, false);
}
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    // if g:IsExists(Card.IsCode,1,nil,CARD_GALAXYEYES_P_DRAGON) then e:GetLabelObject():SetLabel(1) else e:GetLabelObject():SetLabel(0) end
    let g = card::get_material(d, c);
    let found = group::is_exists(d, &g, &|d, x| card::is_code(d, x, &[CARD_GALAXYEYES_P_DRAGON]), 1, NO_EX);
    let le = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, le, &[if found { 1 } else { 0 }]);
    0
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned() and e:GetLabel()==1
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h) && effect::get_label(d, a.e) == 1
}
fn negop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e2, false);
        if card::is_type(d, tc, TYPE_TRAPMONSTER) {
            let e3 = effect::create_effect(d, c);
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e3, false);
        }
    }
}
fn attg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetOverlayCount(tp,0,1)~=0 end
    if a.chk == Some(0) {
        return duel::get_overlay_count(d, a.tp, 0, 1, None) != 0;
    }
    true
}
fn atop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_overlay_group(d, a.tp, 0, 1, None);
    if g.is_empty() {
        return;
    }
    duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None);
    duel::break_effect(d);
    let c = effect::get_handler(d, a.e).nil();
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, g.len() as i64 * 500);
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK);
    effect::set_value(d, e2, g.len() as i64 - 1);
    effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, c, e2, false);
}
