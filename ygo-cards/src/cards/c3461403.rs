//! The Immortal Bushi Mourns the Mortal Body
use super::prelude::*;

pub const ID: u32 = 3461403;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Monsters in your GY become Warrior
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CHANGE_RACE)
    effect::set_code(d, e2, EFFECT_CHANGE_RACE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_GRAVE,0)
    effect::set_target_range(d, e2, LOCATION_GRAVE, 0);
    // e2:SetValue(RACE_WARRIOR)
    effect::set_value(d, e2, RACE_WARRIOR as i64);
    // e2:SetTarget(s.tg)
    effect::set_target(d, e2, Some(effect_card_fn(tg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Code check
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(id)
    effect::set_code(d, e3, ID);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetValue(s.val)
    effect::set_value_fn(d, e3, lua_fn(val));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --Destroy all monsters you control during the End Phase
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.tg(e,c)
fn tg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // if c:GetFlagEffect(1)==0 then
    if card::get_flag_effect(d, c, 1) == 0 {
        // c:RegisterFlagEffect(1,0,0,0)
        card::register_flag_effect(d, c, 1, 0, 0, 0, 0, 0);
        // local eff={c:GetCardEffect(EFFECT_NECRO_VALLEY)}
        let eff = card::get_card_effect(d, c, EFFECT_NECRO_VALLEY);
        // c:ResetFlagEffect(1)
        card::reset_flag_effect(d, c, 1);
        // for _,te in ipairs(eff) do
        for te in eff {
            // local op=te:GetOperation()
            // if not op or op(e,c) then return false end
            match effect::get_operation(d, te) {
                None => return false,
                Some(op) => {
                    if op.call(d, &[Value::Effect(e), Value::Card(c)]).truthy() {
                        return false;
                    }
                }
            }
        }
    }
    // return c:IsMonster()
    card::is_monster(d, c)
}
// function s.val(e,c,re,chk)
fn val(_d: &mut Duel, a: Args) -> Value {
    // if chk==0 then return true end
    if a.int(3) == 0 {
        return Value::Bool(true);
    }
    // return RACE_WARRIOR
    Value::Int(RACE_WARRIOR as i64)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local dg=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,0,nil)
    let dg = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return #dg>0 end
    if a.chk == Some(0) {
        return !dg.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local dg=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,0,nil)
    let dg = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.Destroy(dg,REASON_EFFECT)
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
