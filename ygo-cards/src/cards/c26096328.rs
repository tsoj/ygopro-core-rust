//! Superdreadnought Rail Cannon Juggernaut Liebe
use super::prelude::*;

pub const ID: u32 = 26096328;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,11,3,s.ovfilter,aux.Stringid(id,0),3,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(11),
        3,
        Some(xyz::alter_filter(|d, c, tp, lc| ovfilter(d, c, tp, lc))),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(3)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // boost
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // multi attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK_MONSTER);
    // e2:SetValue(s.raval)
    effect::set_value_fn(d, e2, effect_card_value(raval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsFaceup() and c:GetRank()==10 and c:IsType(TYPE_XYZ,lc,SUMMON_TYPE_XYZ,tp) and c:IsRace(RACE_MACHINE,lc,SUMMON_TYPE_XYZ,tp)
    card::is_faceup(d, c)
        && card::get_rank(d, c) == 10
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(lc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_race_ctx(d, c, RACE_MACHINE, Some(lc), SUMMON_TYPE_XYZ as u64, Some(tp))
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    let tp = a.p(1);
    if a.int(2) == 0 {
        return duel::get_flag_effect(d, tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // return true
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // e1:SetValue(2000)
        effect::set_value(d, e1, 2000);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
    // local e0=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e0 = effect::create_effect(d, h);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e0, EFFECT_CANNOT_ATTACK);
    // e0:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e0, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e0:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e0, LOCATION_MZONE, 0);
    // e0:SetTarget(s.ftarget)
    effect::set_target(d, e0, Some(effect_card_fn(ftarget)));
    // e0:SetLabel(c:GetFieldID())
    effect::set_label(d, e0, &[card::get_field_id(d, c) as i64]);
    // e0:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e0, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e0,tp)
    duel::register_effect(d, e0, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,2),nil)
    aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
}
/// `s.ftarget(e,c)`
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetLabel()~=c:GetFieldID()
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
/// `s.raval(e,c)`
fn raval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local oc=e:GetHandler():GetOverlayCount()
    let h = effect::get_handler(d, e).nil();
    let oc = card::get_overlay_count(d, h) as i64;
    // return math.max(0,oc)
    oc.max(0)
}
