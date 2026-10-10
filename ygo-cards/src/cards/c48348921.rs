//! Number C62: Neo Galaxy-Eyes Prime Photon Dragon
use super::prelude::*;

pub const ID: u32 = 48348921;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(62), listed_names: &[31801517, CARD_GALAXYEYES_P_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_LIGHT),8,3,s.ovfilter,aux.Stringid(id,0))
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, sc, st, Some(tp)))),
        Some(8),
        3,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        None,
        None,
        false,
        None,
    );
    // Make multiple attacks this Battle Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE|PHASE_BATTLE_START)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE_START);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(_,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e1, Some(lua_fn(turn_cond)));
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Unaffected by opponent's monster effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.econ)
    effect::set_condition(d, e2, Some(lua_fn(econ)));
    // e2:SetValue(s.efilter)
    effect::set_value_fn(d, e2, lua_fn(efilter));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Increase ATK
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSummonCode(lc,SUMMON_TYPE_XYZ,tp,31801517)
    card::is_faceup(d, c) && card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_XYZ as u64, tp, &[31801517])
}
/// `function(_,tp) return Duel.IsTurnPlayer(tp) end`
fn turn_cond(d: &mut Duel, a: Args) -> bool {
    duel::is_turn_player(d, a.p(1))
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    // Can make up to 3 attacks on monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK_MONSTER);
    // e1:SetValue(2)
    effect::set_value(d, e1, 2);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.econ(e)`: `e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,CARD_GALAXYEYES_P_DRAGON)`
fn econ(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    let og = card::get_overlay_group(d, h);
    group::is_exists(d, &og, &|d, c| card::is_code(d, c, &[CARD_GALAXYEYES_P_DRAGON]), 1, NO_EX)
}
/// `s.efilter(e,te)`: `te:IsMonsterEffect() and te:GetOwnerPlayer()==1-e:GetHandlerPlayer()`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    effect::is_monster_effect(d, te) && effect::get_owner_player(d, te) == 1 - effect::get_handler_player(d, e)
}
/// `s.atkval(e,c)`: `sum*100` for the Levels and Ranks of the Overlay Units
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local g=c:GetOverlayGroup()
    let g = card::get_overlay_group(d, c);
    // local sum=g:GetSum(Card.GetLevel)+g:GetSum(Card.GetRank)
    let sum = group::get_sum(d, &g, &|d, c| card::get_level(d, c) as i64) + group::get_sum(d, &g, &|d, c| card::get_rank(d, c) as i64);
    // return sum*100
    sum * 100
}
