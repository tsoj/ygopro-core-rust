//! Excode Talker
use super::prelude::*;

pub const ID: u32 = 40669071;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_CYBERSE),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_CYBERSE, sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // Make zones unusable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.lzcon)
    effect::set_condition(d, e1, Some(condition(lzcon)));
    // e1:SetTarget(s.lztg)
    effect::set_target(d, e1, Some(cost_target(lztg)));
    // e1:SetOperation(s.lzop)
    effect::set_operation(d, e1, Some(operation(lzop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent destruction of monsters it points to
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(effect_card_fn(tgtg)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Increase ATK of monsters it points to
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetProperty(0)
    effect::set_property(d, e3, 0, 0);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetValue(500)
    effect::set_value(d, e3, 500);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// s.lzcon(e,tp,eg,ep,ev,re,r,rp)
fn lzcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
/// s.lzfilter(c)
fn lzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()>4
    card::get_sequence(d, c) > 4
}
/// s.lztg(e,tp,eg,ep,ev,re,r,rp,chk)
fn lztg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ct1=Duel.GetMatchingGroupCount(s.lzfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let ct1 = duel::get_matching_group_count(d, Some(&lzfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as i32;
    // local ct2=Duel.GetLocationCount(tp,LOCATION_MZONE)+Duel.GetLocationCount(1-tp,LOCATION_MZONE)
    let ct2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0
        + duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ct1>0 and ct2>=ct1 end
    if a.chk == Some(0) {
        return ct1 > 0 && ct2 >= ct1;
    }
    // local dis=Duel.SelectDisableField(tp,ct1,LOCATION_MZONE,LOCATION_MZONE,0)
    let dis = duel::select_disable_field(d, tp, ct1 as u8, LOCATION_MZONE, LOCATION_MZONE, Some(0), false);
    // Duel.Hint(HINT_ZONE,tp,dis)
    duel::hint(d, HINT_ZONE as u8, tp, dis as u64);
    // e:SetLabel(dis)
    effect::set_label(d, e, &[dis as i64]);
    true
}
/// s.lzop(e,tp,eg,ep,ev,re,r,rp)
fn lzop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_DISABLE_FIELD)
    effect::set_code(d, e1, EFFECT_DISABLE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(lua_fn(disop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetLabel(e:GetLabel())
    let label = effect::get_label(d, e);
    effect::set_label(d, e1, &[label]);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
}
/// s.disop(e,tp)
fn disop(d: &mut Duel, a: Args) -> i64 {
    // return e:GetLabel()
    let e = a.e(0);
    effect::get_label(d, e)
}
/// s.tgtg(e,c)
fn tgtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandler():GetLinkedGroup():IsContains(c)
    let h = effect::get_handler(d, e).nil();
    let g = card::get_linked_group(d, h);
    group::is_contains(&g, c)
}
