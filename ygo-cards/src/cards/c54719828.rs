//! Number 16: Shock Master
use super::prelude::*;

pub const ID: u32 = 54719828;

pub const INFO: ScriptInfo = ScriptInfo {
    xyz_number: Some(16),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,3)
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, None, None, false, None);
    // Declare 1 card type ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=not Duel.HasFlagEffect(tp,id+1)
    let b1 = !duel::has_flag_effect(d, tp, ID + 1, None);
    // local b2=not Duel.HasFlagEffect(tp,id+2)
    let b2 = !duel::has_flag_effect(d, tp, ID + 2, None);
    // local b3=not Duel.HasFlagEffect(tp,id+3)
    let b3 = !duel::has_flag_effect(d, tp, ID + 3, None);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp, {b1,DECLTYPE_MONSTER}, {b2,DECLTYPE_SPELL}, {b3,DECLTYPE_TRAP})
    let op = duel::select_effect(d, tp, &[(b1, DECLTYPE_MONSTER as u64), (b2, DECLTYPE_SPELL as u64), (b3, DECLTYPE_TRAP as u64)]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e) as u32;
    // if Duel.HasFlagEffect(tp,id+op) then return end
    if duel::has_flag_effect(d, tp, ID + op, None) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id+op,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID + op, RESET_PHASE | PHASE_END, 0, 1, 0);
    // That type of card ... cannot be activated ...
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,op))
    effect::set_description(d, e1, aux::stringid(ID, op));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // if op==1 then
    if op == 1 {
        // e1:SetValue(function(e,re,tp) return re:IsMonsterEffect() end)
        effect::set_value_fn(d, e1, lua_fn(|d, a: Args| effect::is_monster_effect(d, a.e(1))));
    // elseif op==2 then
    } else if op == 2 {
        // e1:SetValue(function(e,re,tp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() end)
        effect::set_value_fn(
            d,
            e1,
            lua_fn(|d, a: Args| effect::is_has_type(d, a.e(1), EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, a.e(1))),
        );
    // elseif op==3 then
    } else if op == 3 {
        // e1:SetValue(function(e,re,tp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsTrapEffect() end)
        effect::set_value_fn(
            d,
            e1,
            lua_fn(|d, a: Args| effect::is_has_type(d, a.e(1), EFFECT_TYPE_ACTIVATE) && effect::is_trap_effect(d, a.e(1))),
        );
    }
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,1)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
