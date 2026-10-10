//! Number 38: Hope Harbinger Dragon Titanic Galaxy
use super::prelude::*;

pub const ID: u32 = 63767246;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(38), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,8,2)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, None, None, false, None);
    // Negate an activated Spell Card or effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change the attack target to this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()~=e:GetHandler() end)
    effect::set_condition(d, e2, Some(condition(e2con)));
    // e2:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetOperation(s.chngtgop)
    effect::set_operation(d, e2, Some(operation(chngtgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Make 1 of your Xyz monsters gain ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // return (loc&LOCATION_SZONE)>0 and re:IsSpellEffect() and Duel.IsChainDisablable(ev)
    (loc & LOCATION_SZONE as i64) > 0 && effect::is_spell_effect(d, a.re.nil()) && duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if Duel.NegateEffect(ev) and c:IsRelateToEffect(e) and rc:IsRelateToEffect(re) and c:IsType(TYPE_XYZ)
    //     and rc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT) then
    if duel::negate_effect(d, a.ev as u8)
        && card::is_relate_to_effect(d, c, a.e)
        && card::is_relate_to_effect(d, rc, re)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_can_be_xyz_material(d, rc, Some(c), Some(a.tp), Some(REASON_EFFECT))
    {
        // rc:CancelToGrave()
        card::cancel_to_grave(d, rc, true);
        // Duel.Overlay(c,rc,true)
        duel::overlay(d, c, rc, true);
    }
}
/// `function(e,tp) return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()~=e:GetHandler() end`
fn e2con(d: &mut Duel, a: &EffArgs) -> bool {
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.tp) && duel::get_attack_target(d) != effect::get_handler(d, a.e)
}
/// `s.chngtgop(e,tp,eg,ep,ev,re,r,rp)`
fn chngtgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // if at:CanAttack() and not at:IsImmuneToEffect(e) then
    if card::can_attack(d, at) && !card::is_immune_to_effect(d, at, a.e) {
        // Duel.CalculateDamage(at,c)
        duel::calculate_damage(d, at, Some(c), false);
    }
}
/// `s.atkconfilter(c,tp)`
fn atkconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsType(TYPE_XYZ) and c:GetBaseAttack()>0 and c:IsPreviousPosition(POS_FACEUP)
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_type(d, c, TYPE_XYZ)
        && card::get_base_attack(d, c) > 0
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.atkconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| atkconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsType(TYPE_XYZ) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && card::is_type(d, chkc, TYPE_XYZ) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return eg:IsExists(s.atkconfilter,1,nil,tp)
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsType,TYPE_XYZ),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| atkconfilter(d, c, a.tp), 1, NO_EX)
            && duel::is_existing_target(d, Some(&aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_XYZ))), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsType,TYPE_XYZ),tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_XYZ))), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // local mg=eg:Filter(s.atkconfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let mg = group::filter(d, &eg, &|d, c| atkconfilter(d, c, a.tp), NO_EX);
    // e:SetLabelObject(mg)
    effect::set_label_object(d, a.e, Value::Group(mg));
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsFaceup() and tc:IsRelateToEffect(e)) then return end
    if !(card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e)) {
        return;
    }
    // local g=e:GetLabelObject()
    let mut g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // if #g>=2 then
    if g.len() >= 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
        // g=g:Select(tp,1,1,nil)
        g = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(g:GetFirst():GetBaseAttack())
    let first = group::get_first(d, &g).nil();
    let base = card::get_base_attack(d, first);
    effect::set_value(d, e1, base as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
