//! X-Saber Souza
use super::prelude::*;

pub const ID: u32 = 63612442;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_X_SABER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsSetCard,SET_X_SABER),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_X_SABER as u16], sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Gain 1 of 2 effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsSetCard,1,false,nil,e:GetHandler(),SET_X_SABER) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::check_release_group_cost(d, tp, Some(&|d, c| card::is_set_card(d, c, &[SET_X_SABER as u16])), 1, 1, false, None, h.into());
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsSetCard,1,1,false,nil,e:GetHandler(),SET_X_SABER)
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| card::is_set_card(d, c, &[SET_X_SABER as u16])), 1, 1, false, None, h.into());
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:GetFlagEffect(id)==0 or c:GetFlagEffect(id+1)==0 end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0 || card::get_flag_effect(d, c, ID + 1) == 0;
    }
    // local t1=c:GetFlagEffect(id)
    let t1 = card::get_flag_effect(d, c, ID);
    // local t2=c:GetFlagEffect(id+1)
    let t2 = card::get_flag_effect(d, c, ID + 1);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    // local op=0 (assigned on every path below)
    let op: i32;
    // if t1==0 and t2==0 then
    if t1 == 0 && t2 == 0 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    } else if t1 == 0 {
        // elseif t1==0 then op=Duel.SelectOption(tp,aux.Stringid(id,1))
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]);
    } else {
        // else Duel.SelectOption(tp,aux.Stringid(id,2)) op=1 end
        duel::select_option(d, tp, true, &[aux::stringid(ID, 2)]);
        op = 1;
    }
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==0 then c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    if op == 0 {
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    } else {
        // else c:RegisterFlagEffect(id+1,RESETS_STANDARD_PHASE_END,0,1) end
        card::register_flag_effect(d, c, ID + 1, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // Destroy opponent's monster at start of damage step
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,3))
        effect::set_description(d, e1, aux::stringid(ID, 3));
        // e1:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, e1, CATEGORY_DESTROY);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
        // e1:SetCode(EVENT_BATTLE_START)
        effect::set_code(d, e1, EVENT_BATTLE_START);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // e1:SetCondition(s.descon)
        effect::set_condition(d, e1, Some(condition(descon)));
        // e1:SetTarget(s.destg)
        effect::set_target(d, e1, Some(cost_target(destg)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    } else {
        // Cannot be destroyed by trap effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3012)
        effect::set_description(d, e1, 3012);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetValue(s.efilter)
        effect::set_value_fn(d, e1, lua_fn(|d, a| efilter(d, a)));
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // local d=Duel.GetAttackTarget()
    let mut dd = duel::get_attack_target(d);
    // if d==e:GetHandler() then d=Duel.GetAttacker() end
    if dd == Some(c) {
        dd = duel::get_attacker(d);
    }
    // e:SetLabelObject(d)
    effect::set_label_object(d, a.e, dd.map_or(Value::Nil, Value::Card));
    // return d~=nil
    dd.is_some()
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetLabelObject(),1,0,0)
    let tc = effect::get_label_object(d, a.e).as_card();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local d=e:GetLabelObject()
    let dd = effect::get_label_object(d, a.e).as_card().nil();
    // if d:IsRelateToBattle() then
    if card::is_relate_to_battle(d, dd) {
        // Duel.Destroy(d,REASON_EFFECT)
        duel::destroy(d, dd, REASON_EFFECT, None, None);
    }
}
/// `s.efilter(e,re)`
fn efilter(d: &mut Duel, a: Args<'_>) -> bool {
    // return re:GetOwner():IsTrap()
    let owner = effect::get_owner(d, a.e(1)).nil();
    card::is_trap(d, owner)
}
