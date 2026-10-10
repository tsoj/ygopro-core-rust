//! True King of All Calamities
use super::prelude::*;

pub const ID: u32 = 88581108;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,9,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(9), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Attribute change and prevent effect activation
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetHintTiming(0,TIMING_DRAW_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_DRAW_PHASE));
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.attg)
    effect::set_target(d, e1, Some(cost_target(attg)));
    // e1:SetOperation(s.atop)
    effect::set_operation(d, e1, Some(operation(atop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // "True Draco" and "True King" opponent destruction flag
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(id)
    effect::set_code(d, e2, ID);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.attg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTRIBUTE as u64);
    // e:SetLabel(Duel.AnnounceAttribute(tp,1,ATTRIBUTE_ALL))
    let attr = duel::announce_attribute(d, a.tp, 1, ATTRIBUTE_ALL);
    effect::set_label(d, a.e, &[attr as i64]);
    true
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local attr=e:GetLabel()
    let attr = effect::get_label(d, a.e) as u32;
    // This turn, all face-up monsters on the field become that Attribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetValue(attr)
    effect::set_value(d, e1, attr as i64);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // All monsters with that Attribute cannot activate their effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetLabel(attr)
    effect::set_label(d, e2, &[attr as i64]);
    // e2:SetValue(s.aclimit)
    effect::set_value_fn(d, e2, lua_fn(aclimit));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // All monsters with that Attribute cannot attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetLabel(attr)
    effect::set_label(d, e3, &[attr as i64]);
    // e3:SetTarget(s.atktarget)
    effect::set_target(d, e3, Some(effect_card_fn(atktarget)));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local c=re:GetHandler()
    let re = a.e(1);
    let c = effect::get_handler(d, re).nil();
    // return re:IsMonsterEffect() and c:IsAttribute(e:GetLabel())
    effect::is_monster_effect(d, re) && card::is_attribute(d, c, effect::get_label(d, a.e(0)) as u32)
}
/// `s.atktarget(e,c)`
fn atktarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(e:GetLabel())
    card::is_attribute(d, c, effect::get_label(d, e) as u32)
}
