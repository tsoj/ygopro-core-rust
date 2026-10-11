//! Protocol Gardna
//! scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 5329790;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_CYBERSE),2,2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_CYBERSE, sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot be battle target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(s.atlimit)
    effect::set_value_fn(d, e1, effect_card_fn(atlimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // battle indes
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.valcon)
    effect::set_value_fn(d, e2, lua_fn(|d, a: Args| valcon(d, a.e(0), a.int(2))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // no damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_AVOID_BATTLE_DAMAGE);
    // e3:SetValue(s.damlimit)
    effect::set_value_fn(d, e3, effect_card_fn(damlimit));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandler():GetLinkedGroup():IsContains(c)
    let h = effect::get_handler(d, e).nil();
    let lg = card::get_linked_group(d, h);
    group::is_contains(&lg, c)
}
/// `s.valcon(e,re,r,rp)`
fn valcon(d: &mut Duel, e: EffectId, r: i64) -> bool {
    // if r&REASON_BATTLE~=0 then
    if r & REASON_BATTLE as i64 != 0 {
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        let h = effect::get_handler(d, e).nil();
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.damlimit(e,c)`
fn damlimit(d: &mut Duel, e: EffectId, _c: CardId) -> bool {
    // if e:GetHandler():GetFlagEffect(id)==0 then
    let h = effect::get_handler(d, e).nil();
    if card::get_flag_effect(d, h, ID) == 0 {
        // return true
        true
    } else {
        // else return false end
        false
    }
}
