//! Arc Rebellion Xyz Dragon
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 64276752;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz summon procedure
    // Xyz.AddProcedure(c,nil,5,3)
    xyz::add_procedure(d, c, None, Some(5), 3, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Cannot be destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetCondition(s.indescon)
    effect::set_condition(d, e1, Some(condition(indescon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase ATK and negate monsters effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_NEGATE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_NEGATE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.indescon(e)`
fn indescon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h)
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetBaseAttack()>0
    card::is_faceup(d, c) && card::get_base_attack(d, c) > 0
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&atkfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, h.into());
    }
    true
}
/// `s.ovfilter(c)`
fn ovfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsType(TYPE_XYZ)
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e0=Effect.CreateEffect(e:GetHandler())
    let e0 = effect::create_effect(d, c);
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
    let fid = card::get_field_id(d, c);
    effect::set_label(d, e0, &[fid as i64]);
    // e0:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e0, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e0,tp)
    duel::register_effect(d, e0, tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // if not (c:IsRelateToEffect(e) and c:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.atkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,c)
    let g = duel::get_matching_group(d, Some(&atkfilter), tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
    // local atk=g:GetSum(Card.GetBaseAttack)
    let atk = group::get_sum(d, &g, &|d, x| card::get_base_attack(d, x) as i64);
    // if c:UpdateAttack(atk)==atk and c:GetOverlayGroup():IsExists(s.ovfilter,1,nil) then
    if card::update_attack(d, c, atk, None, None, None) as i64 == atk {
        let og = card::get_overlay_group(d, c);
        if group::is_exists(d, &og, &ovfilter, 1, NO_EX) {
            // local g1=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,c)
            let g1 = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
            // if #g1>0 then Duel.BreakEffect() end
            if !g1.is_empty() {
                duel::break_effect(d);
            }
            // local ng=g1:Filter(Card.IsNegatableMonster,nil)
            let ng = group::filter(d, &g1, &|d, x| card::is_negatable_monster(d, x), NO_EX);
            // for nc in aux.Next(ng) do
            for nc in group::iter(d, &ng) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_DISABLE)
                effect::set_code(d, e1, EFFECT_DISABLE);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // nc:RegisterEffect(e1)
                card::register_effect(d, nc, e1, false);
                // local e2=Effect.CreateEffect(c)
                let e2 = effect::create_effect(d, c);
                // e2:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                // e2:SetCode(EFFECT_DISABLE_EFFECT)
                effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
                // nc:RegisterEffect(e2)
                card::register_effect(d, nc, e2, false);
                // if nc:IsType(TYPE_TRAPMONSTER) then
                if card::is_type(d, nc, TYPE_TRAPMONSTER) {
                    // local e3=Effect.CreateEffect(c)
                    let e3 = effect::create_effect(d, c);
                    // e3:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                    // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
                    effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
                    // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
                    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
                    // nc:RegisterEffect(e3)
                    card::register_effect(d, nc, e3, false);
                }
            }
        }
    }
}
/// `s.ftarget(e,c)`
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetLabel()~=c:GetFieldID()
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
