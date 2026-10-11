//! Ancient Gear Gadjiltron Dragon
use super::prelude::*;

pub const ID: u32 = 50933533;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[41172955, 86445415, 13839120], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Material check
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    // e1:SetValue(s.valcheck)
    effect::set_value_fn(d, e1, lua_fn(valcheck));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gain effects based on the monsters used for its summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCondition(s.regcon)
    effect::set_condition(d, e2, Some(condition(regcon)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // Prevent Spell/Trap activations when it battles
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetValue(s.aclimit)
    effect::set_value_fn(d, e3, lua_fn(aclimit));
    // e3:SetCondition(s.actcon)
    effect::set_condition(d, e3, Some(condition(actcon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.e(1);
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker()==e:GetHandler()
    duel::get_attacker(d) == effect::get_handler(d, a.e)
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) {
    let (e, c) = (a.e(0), a.c(1));
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local flag=0
    let mut flag: i64 = 0;
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // local code=tc:GetCode()
        let code = card::get_code(d, tc)[0];
        // if code==41172955 then flag=(flag|0x1)
        if code == 41172955 {
            flag |= 0x1;
        // elseif code==86445415 then flag=(flag|0x2)
        } else if code == 86445415 {
            flag |= 0x2;
        // elseif code==13839120 then flag=(flag|0x4)
        } else if code == 13839120 {
            flag |= 0x4;
        }
    }
    // e:SetLabel(flag)
    effect::set_label(d, e, &[flag]);
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local flag=e:GetLabelObject():GetLabel()
    let e1 = effect::get_label_object(d, e).as_effect().nil();
    let flag = effect::get_label(d, e1);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if (flag&0x1)~=0 then
    if (flag & 0x1) != 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_PIERCE)
        effect::set_code(d, e1, EFFECT_PIERCE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // if (flag&0x2)~=0 then
    if (flag & 0x2) != 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetCategory(CATEGORY_DAMAGE)
        effect::set_category(d, e1, CATEGORY_DAMAGE);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EVENT_BATTLE_DAMAGE)
        effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
        // e1:SetCondition(s.damcon1)
        effect::set_condition(d, e1, Some(condition(damcon1)));
        // e1:SetTarget(s.damtg1)
        effect::set_target(d, e1, Some(cost_target(damtg1)));
        // e1:SetOperation(s.damop)
        effect::set_operation(d, e1, Some(operation(damop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // if (flag&0x4)~=0 then
    if (flag & 0x4) != 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetCategory(CATEGORY_DAMAGE)
        effect::set_category(d, e1, CATEGORY_DAMAGE);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetCode(EVENT_BATTLE_DESTROYING)
        effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
        // e1:SetTarget(s.damtg2)
        effect::set_target(d, e1, Some(cost_target(damtg2)));
        // e1:SetOperation(s.damop)
        effect::set_operation(d, e1, Some(operation(damop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.damcon1(e,tp,eg,ep,ev,re,r,rp)`
fn damcon1(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp
    a.ep == 1 - a.tp
}
/// `s.damtg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(400)
    duel::set_target_param(d, 400);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,400)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 400);
    true
}
/// `s.damtg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(600)
    duel::set_target_param(d, 600);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,600)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 600);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
