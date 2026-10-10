//! Forbidden Droplet
use super::prelude::*;

pub const ID: u32 = 24299458;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,...,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,0,LOCATION_MZONE,nil)
    let faceup_effect = aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_EFFECT));
    let ct = duel::get_matching_group_count(d, Some(&faceup_effect), tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return ct>0 and Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_ONFIELD|LOCATION_HAND,0,1,e:GetHandler())
        return ct > 0 && {
            let h = effect::get_handler(d, a.e).nil();
            duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_ONFIELD | LOCATION_HAND, 0, 1, h.into())
        };
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGraveAsCost,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,ct,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, ct as u16, false, h.into()).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // e:SetLabel(#og)
    effect::set_label(d, a.e, &[og.len() as i64]);
    // local typ=0
    let mut typ: u32 = 0;
    // for tc in og:Iter() do
    for tc in group::iter(d, &og) {
        // typ = typ|tc:GetOriginalType()
        typ |= card::get_original_type(d, tc);
    }
    // Duel.SetChainLimit(s.chainlimit(typ&(TYPE_MONSTER|TYPE_SPELL|TYPE_TRAP)))
    duel::set_chain_limit(d, chainlimit(typ & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP)));
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,nil,#og,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, og.len() as u8, 0, 0);
    true
}
/// `s.chainlimit(typ)`: the chain limit function `function(e,rp,tp)`
fn chainlimit(typ: u32) -> LuaFn {
    lua_fn(move |d, a: Args| {
        // return tp==rp or e:GetHandler():GetOriginalType() & typ == 0
        if a.p(2) == a.p(1) {
            return true;
        }
        let h = effect::get_handler(d, a.e(0)).nil();
        (card::get_original_type(d, h) & typ) == 0
    })
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // local sg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,0,LOCATION_MZONE,nil)
    let faceup_effect = aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_EFFECT));
    let sg = duel::get_matching_group(d, Some(&faceup_effect), tp, 0, LOCATION_MZONE, NO_EX);
    // if #sg<ct then return end
    if (sg.len() as i64) < ct {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local g=sg:Select(tp,ct,ct,nil)
    let g = group::select(d, &sg, tp, ct as u16, ct as u16, false, NO_EX).nil();
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // local e0=Effect.CreateEffect(c)
        let e0 = effect::create_effect(d, c);
        // e0:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
        // e0:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e0, EFFECT_SET_ATTACK_FINAL);
        // e0:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e0, RESETS_STANDARD_PHASE_END, 1);
        // e0:SetValue(math.ceil(tc:GetAttack()/2))
        let atk = card::get_attack(d, tc);
        effect::set_value(d, e0, (atk as f64 / 2.0).ceil() as i64);
        // tc:RegisterEffect(e0)
        card::register_effect(d, tc, e0, false);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
