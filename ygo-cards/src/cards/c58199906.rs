//! Cybernetic Fusion Support
use super::prelude::*;

pub const ID: u32 = 58199906;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,...,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,math.floor(Duel.GetLP(tp)/2))
    let lp = duel::get_lp(d, a.tp);
    duel::pay_lp_cost(d, a.tp, lp.div_euclid(2) as u32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHAIN_MATERIAL)
    effect::set_code(d, e1, EFFECT_CHAIN_MATERIAL);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTarget(s.chain_target)
    effect::set_target(d, e1, Some(lua_fn(chain_target)));
    // e1:SetOperation(s.chain_operation)
    effect::set_operation(d, e1, Some(lua_fn(chain_operation)));
    // e1:SetValue(aux.FilterBoolFunction(Card.IsRace,RACE_MACHINE))
    effect::set_value_fn(d, e1, lua_fn(|d, a| card::is_race(d, a.c(0), RACE_MACHINE)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsMonster() and c:IsCanBeFusionMaterial() and c:IsAbleToRemove() and not c:IsImmuneToEffect(e)
    card::is_monster(d, c)
        && card::is_can_be_fusion_material(d, c, None, Some(SUMMON_TYPE_FUSION as u64), None)
        && card::is_able_to_remove(d, c, None, None, None)
        && !card::is_immune_to_effect(d, c, e)
}
/// `s.chain_target(e,te,tp,value)`
fn chain_target(d: &mut Duel, a: Args) -> Value {
    let te = a.e(1);
    let tp = a.p(2);
    let value = a.int(3);
    // if value and value&SUMMON_TYPE_FUSION==0 then return Group.CreateGroup() end
    if (value & SUMMON_TYPE_FUSION as i64) == 0 {
        return Value::Group(CardSet::new());
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        // return Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE|LOCATION_HAND,0,nil,te)
        Value::Group(duel::get_matching_group(d, Some(&|d, c| filter(d, c, te)), tp, LOCATION_MZONE | LOCATION_HAND, 0, NO_EX))
    } else {
        // return Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_HAND,0,nil,te)
        Value::Group(duel::get_matching_group(d, Some(&|d, c| filter(d, c, te)), tp, LOCATION_MZONE | LOCATION_GRAVE | LOCATION_HAND, 0, NO_EX))
    }
}
/// `s.chain_operation(e,te,tp,tc,mat,sumtype,sg,sumpos)`
fn chain_operation(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let tp = a.p(2);
    let tc = a.c(3);
    let mat = a.g(4);
    // if not sumtype then sumtype=SUMMON_TYPE_FUSION end
    let sumtype = match a.v(5) {
        Value::Nil | Value::Bool(false) => SUMMON_TYPE_FUSION as i64,
        v => v.int(),
    };
    // tc:SetMaterial(mat)
    card::set_material(d, tc, mat.clone());
    // Duel.Remove(mat,POS_FACEUP,REASON_EFFECT+REASON_MATERIAL+REASON_FUSION)
    duel::remove(d, mat, POS_FACEUP as u8, REASON_EFFECT + REASON_MATERIAL + REASON_FUSION, None, None);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if sg then sg:AddCard(tc) else Duel.SpecialSummonStep(tc,sumtype,tp,tp,false,false,sumpos) end
    match a.v(6) {
        Value::Nil | Value::Bool(false) => {
            let sumpos = a.int(7);
            duel::special_summon_step(d, tc, sumtype as u32, tp, tp, false, false, sumpos as u8, None);
        }
        // UNSURE: sg:AddCard(tc) on a group value cannot write back to the caller's group
        // (Rust groups are values). The fusion procedure always passes nil here.
        _ => crate::lua_error("sg:AddCard in s.chain_operation: group argument not supported"),
    }
    // e:Reset()
    effect::reset(d, e);
}
