//! Dark Magician the Dragon Knight
use super::prelude::*;

pub const ID: u32 = 41721210;

pub const INFO: ScriptInfo = ScriptInfo {
    material_setcode: &[SET_DARK_MAGICIAN],
    listed_names: &[CARD_DARK_MAGICIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,CARD_DARK_MAGICIAN,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_DARK_MAGICIAN),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), st, Some(tp)))),
        ],
    );
    // Change name
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetValue(CARD_DARK_MAGICIAN)
    effect::set_value(d, e1, CARD_DARK_MAGICIAN as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent effect target
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_ONFIELD,0)
    effect::set_target_range(d, e3, LOCATION_ONFIELD, 0);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsSpellTrap))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_spell_trap(d, c))));
    // e3:SetValue(aux.indoval)
    effect::set_value_fn(d, e3, aux::indoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Prevent destruction by opponent's effect
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e4:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e4, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e4:SetValue(aux.tgoval)
    effect::set_value_fn(d, e4, aux::tgoval());
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
