//! Chain Material
use super::prelude::*;

pub const ID: u32 = 39980304;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,...,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetActivityCount(tp,ACTIVITY_ATTACK)==0 end
    if a.chk == Some(0) {
        return duel::get_activity_count(d, a.tp, &[ACTIVITY_ATTACK])[0] == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_OATH + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    true
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE);
    }
    true
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsMonster() and c:IsCanBeFusionMaterial() and c:IsAbleToRemove() and not c:IsImmuneToEffect(e)
    card::is_monster(d, c)
        && card::is_can_be_fusion_material(d, c, None, None, None)
        && card::is_able_to_remove(d, c, None, None, None)
        && !card::is_immune_to_effect(d, c, e)
}
/// `s.chain_target(e,te,tp,value)`
fn chain_target(d: &mut Duel, a: Args) -> Value {
    let te = a.e(1);
    let tp = a.p(2);
    // if not value or value&SUMMON_TYPE_FUSION==0 then return Group.CreateGroup() end
    if !a.v(3).truthy() || (a.int(3) & SUMMON_TYPE_FUSION as i64) == 0 {
        return Value::Group(CardSet::new());
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        // return Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE|LOCATION_HAND|LOCATION_DECK,0,nil,te)
        Value::Group(duel::get_matching_group(
            d,
            Some(&|d, c| filter(d, c, te)),
            tp,
            LOCATION_MZONE | LOCATION_HAND | LOCATION_DECK,
            0,
            NO_EX,
        ))
    } else {
        // return Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_HAND|LOCATION_DECK,0,nil,te)
        Value::Group(duel::get_matching_group(
            d,
            Some(&|d, c| filter(d, c, te)),
            tp,
            LOCATION_MZONE | LOCATION_GRAVE | LOCATION_HAND | LOCATION_DECK,
            0,
            NO_EX,
        ))
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
    match a.v(6) {
        Value::Nil | Value::Bool(false) => {
            // Duel.SpecialSummonStep(tc,sumtype,tp,tp,false,false,sumpos)
            let sumpos = a.int(7);
            duel::special_summon_step(d, tc, sumtype as u32, tp, tp, false, false, sumpos as u8, None);
            // tc:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET),0,1)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 0, 1, 0, 0);
        }
        // UNSURE: sg:AddCard(tc) on a group value cannot write back to the caller's group
        // (Rust groups are values). The fusion procedure always passes nil here.
        _ => crate::lua_error("sg:AddCard in s.chain_operation: group argument not supported"),
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_label_object(d, e1, Value::from(tc));
    effect::set_condition(d, e1, Some(lua_fn(descon)));
    effect::set_operation(d, e1, Some(lua_fn(desop)));
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    duel::register_effect(d, e1, tp);
}
/// `s.descon(e,tp,...)`
fn descon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tc = effect::get_label_object(d, e).as_card().nil();
    if card::get_flag_effect(d, tc, ID) != 0 {
        return true;
    }
    effect::reset(d, e);
    false
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: Args) {
    let tc = effect::get_label_object(d, a.e(0)).as_card().nil();
    duel::destroy(d, tc, REASON_EFFECT, None, None);
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CHAIN_MATERIAL);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    effect::set_target(d, e1, Some(lua_fn(chain_target)));
    effect::set_operation(d, e1, Some(lua_fn(chain_operation)));
    effect::set_value_fn(d, e1, aux_true());
    duel::register_effect(d, e1, a.tp);
}
