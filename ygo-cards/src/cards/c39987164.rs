//! Vylon Disigma
use super::prelude::*;

pub const ID: u32 = 39987164;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,4,3)
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e1)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| eqval(d, a.c(0), a.p(2))),
        lua_fn(|d, a| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e1,
        None,
        None,
        None,
    );
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, ec: CardId, tp: u8) -> bool {
    // return ec:IsControler(1-tp) and ec:IsAttackPos() and ec:IsType(TYPE_EFFECT)
    card::is_controler(d, ec, 1 - tp) && card::is_attack_pos(d, ec) && card::is_type(d, ec, TYPE_EFFECT)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttackPos() and c:IsType(TYPE_EFFECT) and c:IsAbleToChangeControler()
    card::is_faceup(d, c) && card::is_attack_pos(d, c) && card::is_type(d, c, TYPE_EFFECT) && card::is_able_to_change_controler(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, 0, 0);
    true
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // c:EquipByEffectAndLimitRegister(e,tp,tc,id)
    card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), None);
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsAttackPos() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::is_attack_pos(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // s.equipop(c,e,tp,tc)
            equipop(d, c, a.e, a.tp, tc);
        }
    }
}
/// `s.desfilter(c,att)`
fn desfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:GetFlagEffect(id)>0 and c:IsAttribute(att)
    card::get_flag_effect(d, c, ID) > 0 && card::is_attribute(d, c, att)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local dt=Duel.GetAttacker()
    let mut dt = duel::get_attacker(d);
    // if dt==c then dt=Duel.GetAttackTarget() end
    if dt == Some(c) {
        dt = duel::get_attack_target(d);
    }
    // if not dt or dt:IsFacedown() then return false end
    let Some(dt) = dt else { return false };
    if card::is_facedown(d, dt) {
        return false;
    }
    // e:SetLabelObject(dt)
    effect::set_label_object(d, a.e, Value::Card(dt));
    // local att=dt:GetAttribute()
    let att = card::get_attribute(d, dt);
    // return c:GetEquipGroup():IsExists(s.desfilter,1,nil,att)
    let eg = card::get_equip_group(d, c);
    group::is_exists(d, &eg, &|d, x| desfilter(d, x, att), 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetLabelObject(),1,0,0)
    let dt = effect::get_label_object(d, a.e).as_card();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, dt.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local dt=e:GetLabelObject()
    let dt = effect::get_label_object(d, a.e).as_card();
    // local eqg=e:GetHandler():GetEquipGroup()
    let h = effect::get_handler(d, a.e).nil();
    let eqg = card::get_equip_group(d, h);
    // if #eqg==0 or not eqg:IsExists(s.desfilter,1,nil,dt:GetAttribute()) then return end
    if eqg.is_empty() {
        return;
    }
    let att = card::get_attribute(d, dt.nil());
    if !group::is_exists(d, &eqg, &|d, x| desfilter(d, x, att), 1, NO_EX) {
        return;
    }
    // if dt:IsRelateToBattle() then
    let dt = dt.nil();
    if card::is_relate_to_battle(d, dt) {
        // Duel.Destroy(dt,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(dt), REASON_EFFECT, None, None);
    }
}
