//! Kunai with Chain
use super::prelude::*;

pub const ID: u32 = 37390589;
pub const INFO: ScriptInfo = ScriptInfo { self_equip_trap: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 or both of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET | EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return e:GetLabel()==2 and chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return effect::get_label(d, e) == 2 && card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // local ac=Duel.GetAttacker()
    let ac = duel::get_attacker(d);
    // local b1=Duel.CheckEvent(EVENT_ATTACK_ANNOUNCE) and ac:IsControler(1-tp) and ac:IsLocation(LOCATION_MZONE) and ac:IsAttackPos()
    //     and ac:IsCanChangePosition() and ac:IsCanBeEffectTarget(e)
    let b1 = duel::check_event(d, EVENT_ATTACK_ANNOUNCE).is_some() && {
        let ac = ac.nil();
        card::is_controler(d, ac, 1 - tp)
            && card::is_location(d, ac, LOCATION_MZONE)
            && card::is_attack_pos(d, ac)
            && card::is_can_change_position(d, ac)
            && card::is_can_be_effect_target(d, ac, Some(e))
    };
    // local b2=e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil)
    let b2 = effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE)
        && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b1 and b2,aux.Stringid(id,3)})
    let op = duel::select_effect(
        d,
        tp,
        &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b1 && b2, aux::stringid(ID, 3))],
    )
    .nil() as u32;
    // Duel.SetTargetParam(op)
    duel::set_target_param(d, op);
    // e:SetCategory(0)
    effect::set_category(d, e, 0);
    // if op==1 or op==3 then
    if op == 1 || op == 3 {
        // e:SetCategory(CATEGORY_POSITION)
        effect::set_category(d, e, CATEGORY_POSITION);
        let ac = ac.nil();
        // Duel.SetTargetCard(ac)
        duel::set_target_card(d, ac);
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,ac,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(ac)), 1, tp, 0);
    }
    // if op==2 or op==3 then
    if op == 2 || op == 3 {
        // e:SetCategory(e:GetCategory()|CATEGORY_EQUIP)
        let cat = effect::get_category(d, e);
        effect::set_category(d, e, cat | CATEGORY_EQUIP);
        // if e:GetLabel()==1 then
        //     aux.RemainFieldCost(e,tp,eg,ep,ev,re,r,rp,1)
        // end
        if effect::get_label(d, e) == 1 {
            let args = [
                Value::Effect(e),
                Value::Int(tp as i64),
                a.eg.clone().map_or(Value::Nil, Value::Group),
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                Value::effect(a.re),
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
                Value::Int(1),
            ];
            aux::remain_field_cost().call(d, &args);
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil)
        let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // e:SetLabelObject(g:GetFirst())
        let first = group::get_first(d, &g);
        effect::set_label_object(d, e, Value::card(first));
        // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
        let h = effect::get_handler(d, e);
        duel::set_operation_info(d, 0, CATEGORY_EQUIP, h.map(CardSet::from_card), 1, 0, 0);
    }
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local op=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let op = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if op==1 or op==3 then
    if op == 1 || op == 3 {
        // local tc=Duel.GetAttacker()
        let tc = duel::get_attacker(d).nil();
        // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:CanAttack() and not tc:IsStatus(STATUS_ATTACK_CANCELED) then
        if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) && card::can_attack(d, tc) && !card::is_status(d, tc, STATUS_ATTACK_CANCELED) {
            // Duel.ChangePosition(tc,POS_FACEUP_DEFENSE)
            duel::change_position(d, CardSet::from_card(tc), POS_FACEUP_DEFENSE as u8, None, None, None, false);
        }
    }
    // if op==2 or op==3 then
    if op == 2 || op == 3 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // if not c:IsRelateToEffect(e) or c:IsStatus(STATUS_LEAVE_CONFIRMED) then return end
        if !card::is_relate_to_effect(d, c, e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
            return;
        }
        // local tc=e:GetLabelObject()
        let tc = effect::get_label_object(d, e).as_card().nil();
        // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsControler(tp) and Duel.Equip(tp,c,tc) then
        if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) && card::is_controler(d, tc, tp) && duel::equip(d, tp, c, tc, true, false) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_EQUIP)
            effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(500)
            effect::set_value(d, e1, 500);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
            // e2:SetValue(function(_e,_c) return _c:IsControler(tp) end)
            effect::set_value_fn(d, e2, effect_card_fn(move |d, _e, c| card::is_controler(d, c, tp)));
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
        } else {
            // c:CancelToGrave(false)
            card::cancel_to_grave(d, c, false);
        }
    }
}
