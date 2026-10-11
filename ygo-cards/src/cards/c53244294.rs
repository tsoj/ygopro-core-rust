//! Number 57: Tri-Head Dust Dragon
use super::prelude::*;

pub const ID: u32 = 53244294;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(57), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,4,3)
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // disable zone
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.zcon)
    effect::set_condition(d, e2, Some(condition(zcon)));
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.ztg)
    effect::set_target(d, e2, Some(cost_target(ztg)));
    // e2:SetOperation(s.zop)
    effect::set_operation(d, e2, Some(operation(zop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&|d, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_UPDATE_ATTACK)
                effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                // e1:SetValue(tc:GetAttack())
                let atk = card::get_attack(d, tc) as i64;
                effect::set_value(d, e1, atk);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
                // c:RegisterEffect(e1)
                card::register_effect(d, c, e1, false);
            }
        }
    }
}
/// `s.zcon(e,tp,eg,ep,ev,re,r,rp)`
fn zcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0)
    let mine = duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD);
    let theirs = duel::get_field_group_count(d, tp, LOCATION_ONFIELD, 0);
    mine > theirs
}
/// `s.ztg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ztg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE,PLAYER_NONE,0)+...>0 end
    if a.chk == Some(0) {
        let p_none = Some(PLAYER_NONE as u8);
        let total = duel::get_location_count(d, tp, LOCATION_MZONE as u8, p_none, Some(0), None).0
            + duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, p_none, Some(0), None).0
            + duel::get_location_count(d, tp, LOCATION_SZONE as u8, p_none, Some(0), None).0
            + duel::get_location_count(d, 1 - tp, LOCATION_SZONE as u8, p_none, Some(0), None).0;
        return total > 0;
    }
    // local dis=Duel.SelectDisableField(tp,1,LOCATION_ONFIELD,LOCATION_ONFIELD,0)
    let dis = duel::select_disable_field(d, tp, 1, LOCATION_ONFIELD, LOCATION_ONFIELD, Some(0), false);
    // Duel.Hint(HINT_ZONE,tp,dis)
    duel::hint(d, HINT_ZONE as u8, tp, dis as u64);
    // e:SetLabel(dis)
    effect::set_label(d, a.e, &[dis as i64]);
    true
}
/// `s.zop(e,tp,eg,ep,ev,re,r,rp)`
fn zop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_DISABLE_FIELD)
    effect::set_code(d, e1, EFFECT_DISABLE_FIELD);
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(lua_fn(disop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // e1:SetLabel(e:GetLabel())
    let label = effect::get_label(d, a.e);
    effect::set_label(d, e1, &[label]);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, h, e1, false);
}
/// `s.disop(e,tp)`: returns the disabled zones stored in the label
fn disop(d: &mut Duel, a: Args) -> i64 {
    // return e:GetLabel()
    let e = a.e(0);
    effect::get_label(d, e)
}
