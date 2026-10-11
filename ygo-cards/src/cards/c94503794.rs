//! Kerzebs, the Gaoled Glutton
use super::prelude::*;

pub const ID: u32 = 94503794;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 7 monsters
    // Xyz.AddProcedure(c,nil,7,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(7), 2, None, None, Some(xyz::MaxCt::Infinite), None, false, None);
    // Gains 700 ATK for each material attached to it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return c:GetOverlayCount()*700 end)
    effect::set_value_fn(d, e1, effect_card_value(|d, _e, c| card::get_overlay_count(d, c) as i64 * 700));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach that activated Normal or Quick-Play Spell Card on the field to this face-up card, and if you do, this card is unaffected by that activated effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e2:SetCondition(s.attachcon)
    effect::set_condition(d, e2, Some(condition(attachcon)));
    // e2:SetOperation(s.attachop)
    effect::set_operation(d, e2, Some(operation(attachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Set 1 Spell attached to this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.settg)
    effect::set_target(d, e3, Some(cost_target(settg)));
    // e3:SetOperation(s.setop)
    effect::set_operation(d, e3, Some(operation(setop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.attachcon(e,tp,eg,ep,ev,re,r,rp)`
fn attachcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and (rc:IsNormalSpell() or rc:IsQuickPlaySpell()) and rc:IsOnField()
    //     and rc:IsCanBeXyzMaterial(e:GetHandler(),tp,REASON_EFFECT)
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && (card::is_normal_spell(d, rc) || card::is_quick_play_spell(d, rc))
        && card::is_on_field(d, rc)
        && {
            let h = effect::get_handler(d, a.e).nil();
            card::is_can_be_xyz_material(d, rc, Some(h), Some(a.tp), Some(REASON_EFFECT))
        }
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and rc:IsRelateToEffect(re)
    //     and not c:IsImmuneToEffect(e) and not rc:IsImmuneToEffect(e)
    //     and rc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT) then
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_faceup(d, c)
        && card::is_relate_to_effect(d, rc, re)
        && !card::is_immune_to_effect(d, c, a.e)
        && !card::is_immune_to_effect(d, rc, a.e)
        && card::is_can_be_xyz_material(d, rc, Some(c), Some(a.tp), Some(REASON_EFFECT))
    {
        // Duel.Overlay(c,rc)
        duel::overlay(d, c, rc, false);
        // if not c:GetOverlayGroup():IsContains(rc) then return end
        let ov = card::get_overlay_group(d, c);
        if !group::is_contains(&ov, rc) {
            return;
        }
        // rc:CancelToGrave()
        card::cancel_to_grave(d, rc, true);
        // This card is unaffected by that activated effect
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_IMMUNE_EFFECT)
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        // e1:SetValue(function(e,te) return te==re end)
        effect::set_value_fn(d, e1, lua_fn(move |_d, v| v.e(1) == re));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CHAIN)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and c:IsSSetable()
    card::is_spell(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():GetOverlayGroup():IsExists(s.setfilter,1,nil) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        let ov = card::get_overlay_group(d, c);
        return group::is_exists(d, &ov, &setfilter, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=c:GetOverlayGroup():FilterSelect(tp,s.setfilter,1,1,nil)
    let ov = card::get_overlay_group(d, c);
    let g = group::filter_select(d, &ov, a.tp, &setfilter, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g)
        duel::s_set(d, a.tp, g, None, true);
    }
}
