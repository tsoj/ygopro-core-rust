//! Go! - D/D/D Divine Zero King Rage
use super::prelude::*;

pub const ID: u32 = 40227329;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Enable pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Take no effect damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.damval)
    effect::set_value_fn(d, e1, lua_fn(damval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Tribute summon "D/D" monster without tributing
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e3, EFFECT_SUMMON_PROC);
    // e3:SetRange(LOCATION_PZONE)
    effect::set_range(d, e3, LOCATION_PZONE);
    // e3:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e3, LOCATION_HAND, 0);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.ntcon)
    effect::set_condition(d, e3, Some(lua_fn(ntcon)));
    // e3:SetTarget(aux.FieldSummonProcTg(s.nttg))
    effect::set_target(d, e3, Some(aux::field_summon_proc_tg(Some(effect_card_fn(nttg)), None)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Apply effects
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCost(s.cost)
    effect::set_cost(d, e4, Some(cost_target(cost_fn)));
    // e4:SetTarget(s.target)
    effect::set_target(d, e4, Some(cost_target(target)));
    // e4:SetOperation(s.operation)
    effect::set_operation(d, e4, Some(operation(operation_fn)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Gain ATK equal to opponent's LP
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,5))
    effect::set_description(d, e5, aux::stringid(ID, 5));
    // e5:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e5, CATEGORY_ATKCHANGE);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e5:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e5, EVENT_ATTACK_ANNOUNCE);
    // e5:SetCondition(s.atkcon)
    effect::set_condition(d, e5, Some(condition(atkcon)));
    // e5:SetOperation(s.atkop)
    effect::set_operation(d, e5, Some(operation(atkop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Take no battle damage involving this card
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e6, EFFECT_AVOID_BATTLE_DAMAGE);
    // e6:SetValue(1)
    effect::set_value(d, e6, 1);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // Cannot be destroyed by battle
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE);
    // e7:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e7, EFFECT_INDESTRUCTABLE_BATTLE);
    // e7:SetValue(1)
    effect::set_value(d, e7, 1);
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
// s.damval(e,re,val,r,rp,rc)
fn damval(d: &mut Duel, a: Args) -> i64 {
    // local c=e:GetHandler()
    let e = a.e(0);
    let _c = effect::get_handler(d, e);
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // if r&REASON_EFFECT~=0 and Duel.GetFlagEffect(tp,id)==0 then
    if (a.int(3) & REASON_EFFECT as i64) != 0 && duel::get_flag_effect(d, tp, ID) == 0 {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // return 0
        return 0;
    }
    // return val
    a.int(2)
}
// s.ntcon(e,c,minc)
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return minc==0 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    a.int(2) == 0 && duel::get_location_count(d, card::get_controler(d, c), LOCATION_MZONE as u8, None, None, None).0 > 0
}
// s.nttg(e,c)
fn nttg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsLevelAbove(5) and c:IsSetCard(SET_DD)
    card::is_level_above(d, c, 5) && card::is_set_card(d, c, &[SET_DD as u16])
}
// s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,1,false,nil,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::check_release_group_cost(d, a.tp, None, 1, 1, false, None, h.into());
    }
    // local g=Duel.SelectReleaseGroupCost(tp,nil,1,1,false,nil,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_release_group_cost(d, a.tp, None, 1, 1, false, None, h.into());
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// Duel.IsAbleToEnterBP() and not e:GetHandler():IsHasEffect(EFFECT_DIRECT_ATTACK)
fn b1(d: &mut Duel, e: EffectId) -> bool {
    duel::is_able_to_enter_bp(d) && card::is_has_effect(d, effect::get_handler(d, e).nil(), EFFECT_DIRECT_ATTACK, None).is_empty()
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=Duel.IsAbleToEnterBP() and not e:GetHandler():IsHasEffect(EFFECT_DIRECT_ATTACK)
    let b1 = b1(d, a.e);
    // local b2=Duel.GetFlagEffect(1-tp,id+1)==0
    let b2 = duel::get_flag_effect(d, 1 - a.tp, ID + 1) == 0;
    // local b3=Duel.GetFlagEffect(1-tp,id+2)==0
    let b3 = duel::get_flag_effect(d, 1 - a.tp, ID + 2) == 0;
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    true
}
// s.operation(e,tp,eg,ep,ev,re,r,rp)
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local b1=Duel.IsAbleToEnterBP() and not e:GetHandler():IsHasEffect(EFFECT_DIRECT_ATTACK)
    let b1 = b1(d, a.e);
    // local b2=Duel.GetFlagEffect(1-tp,id+1)==0
    let b2 = duel::get_flag_effect(d, 1 - a.tp, ID + 1) == 0;
    // local b3=Duel.GetFlagEffect(1-tp,id+2)==0
    let b3 = duel::get_flag_effect(d, 1 - a.tp, ID + 2) == 0;
    // local dtab={}
    let mut dtab: Vec<u64> = Vec::new();
    // if b1 then table.insert(dtab,aux.Stringid(id,2)) end
    if b1 {
        dtab.push(aux::stringid(ID, 2));
    }
    // if b2 then table.insert(dtab,aux.Stringid(id,3)) end
    if b2 {
        dtab.push(aux::stringid(ID, 3));
    }
    // if b3 then table.insert(dtab,aux.Stringid(id,4)) end
    if b3 {
        dtab.push(aux::stringid(ID, 4));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RESOLVEEFFECT)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RESOLVEEFFECT as u64);
    // local op=Duel.SelectOption(tp,table.unpack(dtab))+1
    let mut op = duel::select_option(d, a.tp, true, &dtab) + 1;
    // if not (b1 or b2) then op=3 end
    if !(b1 || b2) {
        op = 3;
    }
    // if not (b1 or b3) then op=2 end
    if !(b1 || b3) {
        op = 2;
    }
    // if (b1 and b3 and not b2 and op==2) then op=3 end
    if b1 && b3 && !b2 && op == 2 {
        op = 3;
    }
    // if (b2 and b3 and not b1) then op=op+1 end
    if b2 && b3 && !b1 {
        op += 1;
    }
    if op == 1 {
        // Make itself be able to attack directly
        // if not c:IsRelateToEffect(e) then return end
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3205)
        effect::set_description(d, e1, 3205);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    } else if op == 2 {
        // Opponent cannot activate cards or effects in S/T zones
        // Duel.RegisterFlagEffect(1-tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, 1 - a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // aux.RegisterClientHint(c,nil,tp,0,1,aux.Stringid(id,6),nil)
        aux::register_client_hint(d, c, 0, a.tp, 0, 1, Some(aux::stringid(ID, 6)), 0, None);
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
        // e2:SetValue(s.aclimit1)
        effect::set_value_fn(d, e2, lua_fn(aclimit1));
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    } else if op == 3 {
        // Opponent cannot activate effects in hand or GY
        // Duel.RegisterFlagEffect(1-tp,id+2,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, 1 - a.tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
        // aux.RegisterClientHint(c,nil,tp,0,1,aux.Stringid(id,7),nil)
        aux::register_client_hint(d, c, 0, a.tp, 0, 1, Some(aux::stringid(ID, 7)), 0, None);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD);
        // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
        // e3:SetTargetRange(0,1)
        effect::set_target_range(d, e3, 0, 1);
        // e3:SetValue(s.aclimit2)
        effect::set_value_fn(d, e3, lua_fn(aclimit2));
        // e3:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, a.tp);
    }
}
// s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetLP(1-tp)<=4000
    duel::get_lp(d, 1 - a.tp) <= 4000
}
// s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local lp=Duel.GetLP(1-tp)
    let lp = duel::get_lp(d, 1 - a.tp);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetValue(lp)
    effect::set_value(d, e1, lp as i64);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.aclimit1(e,re,tp)
fn aclimit1(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    // return rc and rc:IsLocation(LOCATION_SZONE)
    match effect::get_handler(d, a.e(1)) {
        Some(rc) => card::is_location(d, rc, LOCATION_SZONE),
        None => false,
    }
}
// s.aclimit2(e,re,tp)
fn aclimit2(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    // return rc and rc:IsLocation(LOCATION_HAND|LOCATION_GRAVE)
    match effect::get_handler(d, a.e(1)) {
        Some(rc) => card::is_location(d, rc, LOCATION_HAND | LOCATION_GRAVE),
        None => false,
    }
}
