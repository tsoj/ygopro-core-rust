//! No.86 H－C ロンゴミアント
//! Number 86: Heroic Champion - Rhongomyniad
use super::prelude::*;

pub const ID: u32 = 63504681;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(86), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_WARRIOR),4,2,nil,nil,5)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_WARRIOR, sc, st, Some(tp)))),
        Some(4),
        2,
        None,
        None,
        Some(xyz::MaxCt::N(5)),
        None,
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // remove material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.rmcon)
    effect::set_condition(d, e1, Some(condition(rmcon)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // battle indestructable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // e2:SetLabel(1)
    effect::set_label(d, e2, &[1]);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atk/def
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetValue(1500)
    effect::set_value(d, e3, 1500);
    // e3:SetCondition(s.effcon)
    effect::set_condition(d, e3, Some(condition(effcon)));
    // e3:SetLabel(2)
    effect::set_label(d, e3, &[2]);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // immune
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e5, EFFECT_IMMUNE_EFFECT);
    // e5:SetValue(s.efilter)
    effect::set_value_fn(d, e5, lua_fn(efilter));
    // --e5:SetCondition(s.effcon) --handled in Value for mid-resolution updating
    // e5:SetLabel(3)
    effect::set_label(d, e5, &[3]);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // disable spsummon
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e6, EFFECT_CANNOT_SUMMON);
    // e6:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e6:SetTargetRange(0,1)
    effect::set_target_range(d, e6, 0, 1);
    // e6:SetCondition(s.effcon)
    effect::set_condition(d, e6, Some(condition(effcon)));
    // e6:SetLabel(4)
    effect::set_label(d, e6, &[4]);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=e6:Clone()
    let e7 = effect::clone(d, e6);
    // e7:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e7, EFFECT_CANNOT_SPECIAL_SUMMON);
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // destroy
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e8, aux::stringid(ID, 1));
    // e8:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e8, CATEGORY_DESTROY);
    // e8:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e8, EFFECT_TYPE_IGNITION);
    // e8:SetCountLimit(1)
    effect::set_count_limit(d, e8, 1, 0, 0, 0);
    // e8:SetRange(LOCATION_MZONE)
    effect::set_range(d, e8, LOCATION_MZONE);
    // e8:SetCondition(s.effcon)
    effect::set_condition(d, e8, Some(condition(effcon)));
    // e8:SetTarget(s.destg)
    effect::set_target(d, e8, Some(cost_target(destg)));
    // e8:SetOperation(s.desop)
    effect::set_operation(d, e8, Some(operation(desop)));
    // e8:SetLabel(5)
    effect::set_label(d, e8, &[5]);
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetOverlayCount()>0 then
    if card::get_overlay_count(d, c) > 0 {
        // c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)
        card::remove_overlay_card(d, c, tp, 1, 1, REASON_EFFECT);
    }
}
/// `s.effcon(e,...)`: `e:GetHandler():GetOverlayCount()>=e:GetLabel()`
fn effcon_e(d: &mut Duel, e: EffectId) -> bool {
    let c = effect::get_handler(d, e).nil();
    card::get_overlay_count(d, c) as i64 >= effect::get_label(d, e)
}
/// `s.effcon` as a condition
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    effcon_e(d, a.e)
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    // return te:GetOwner()~=e:GetOwner() and s.effcon(e) --condition handling for mid-resolution updating
    effect::get_owner(d, te) != effect::get_owner(d, e) && effcon_e(d, e)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.TRUE,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
