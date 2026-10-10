//! Signal Warrior
use super::prelude::*;

pub const ID: u32 = 9634146;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SIGNAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Place Signal Counters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by battle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e) return e:GetHandler():GetCounter(COUNTER_SIGNAL)>0 end)
    effect::set_condition(
        d,
        e2,
        Some(condition(|d, a| {
            let c = effect::get_handler(d, a.e).nil();
            card::get_counter(d, c, COUNTER_SIGNAL as u16) > 0
        })),
    );
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot be destroyed by your opponent's card effects
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetValue(aux.indoval)
    effect::set_value_fn(d, e3, aux::indoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Remove Signal Counters and apply 1 effect
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCost(s.effcost)
    effect::set_cost(d, e4, Some(cost_target(effcost)));
    // e4:SetOperation(s.effop)
    effect::set_operation(d, e4, Some(operation(effop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,0,LOCATION_FZONE,LOCATION_FZONE,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), 0, LOCATION_FZONE, LOCATION_FZONE, NO_EX);
    // if c:IsRelateToEffect(e) then g:AddCard(c) end
    if card::is_relate_to_effect(d, c, a.e) {
        group::add_card(&mut g, &CardSet::from_card(c));
    }
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // if tc:IsCanAddCounter(COUNTER_SIGNAL,1) then
        if card::is_can_add_counter(d, tc, COUNTER_SIGNAL as u16, 1, false, 0) {
            // tc:AddCounter(COUNTER_SIGNAL,1)
            card::add_counter(d, tc, COUNTER_SIGNAL as u16, 1, false);
        }
    }
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.IsCanRemoveCounter(tp,1,1,COUNTER_SIGNAL,4,REASON_COST)
    let b1 = duel::is_can_remove_counter(d, tp, 1, 1, COUNTER_SIGNAL as u16, 4, REASON_COST);
    // local b2=Duel.IsCanRemoveCounter(tp,1,1,COUNTER_SIGNAL,7,REASON_COST) and Duel.IsPlayerCanDraw(tp,1)
    let b2 = duel::is_can_remove_counter(d, tp, 1, 1, COUNTER_SIGNAL as u16, 7, REASON_COST) && duel::is_player_can_draw(d, tp, 1);
    // local b3=Duel.IsCanRemoveCounter(tp,1,1,COUNTER_SIGNAL,10,REASON_COST)
    //     and Duel.IsExistingMatchingCard(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let b3 = duel::is_can_remove_counter(d, tp, 1, 1, COUNTER_SIGNAL as u16, 10, REASON_COST)
        && duel::is_existing_matching_card(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)},{b3,aux.Stringid(id,3)})
    let op = duel::select_effect(
        d,
        tp,
        &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))],
    )
    .nil() as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // local ct=4+(op-1)*3
    let ct = 4 + (op - 1) * 3;
    // Duel.RemoveCounter(tp,1,1,COUNTER_SIGNAL,ct,REASON_COST)
    duel::remove_counter(d, tp, 1, 1, COUNTER_SIGNAL as u16, ct as u32, REASON_COST);
    if op == 1 {
        // e:SetCategory(CATEGORY_DAMAGE)
        effect::set_category(d, a.e, CATEGORY_DAMAGE);
        // e:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_PLAYER_TARGET, 0);
        // Duel.SetTargetPlayer(1-tp)
        duel::set_target_player(d, 1 - tp);
        // Duel.SetTargetParam(800)
        duel::set_target_param(d, 800);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,800)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 800);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DRAW);
        // e:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_PLAYER_TARGET, 0);
        // Duel.SetTargetPlayer(tp)
        duel::set_target_player(d, tp);
        // Duel.SetTargetParam(1)
        duel::set_target_param(d, 1);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Inflict 800 damage to your opponent
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        // Duel.Damage(p,d,REASON_EFFECT)
        duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
    } else if op == 2 {
        // Draw 1 card
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        // Duel.Draw(p,d,REASON_EFFECT)
        duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
    } else if op == 3 {
        // Destroy 1 card on the field
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
