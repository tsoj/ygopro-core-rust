//! Sea Stealth Attack
use super::prelude::*;

pub const ID: u32 = 19089195;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_UMI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent destruction
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.econ)
    effect::set_condition(d, e2, Some(condition(econ)));
    // e2:SetCost(s.remcost)
    effect::set_cost(d, e2, Some(cost_target(remcost)));
    // e2:SetOperation(s.remop)
    effect::set_operation(d, e2, Some(operation(remop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy battling monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e3, EVENT_BATTLE_START);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.filter(c,tp)
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(CARD_UMI) and c:GetActivateEffect() and c:GetActivateEffect():IsActivatable(tp,true)
    //     and (c:IsType(TYPE_FIELD) or Duel.GetLocationCount(tp,LOCATION_SZONE)>0)
    if !card::is_code(d, c, &[CARD_UMI]) {
        return false;
    }
    let Some(ae) = card::get_activate_effect(d, c).first().copied() else { return false };
    effect::is_activatable(d, ae, tp, true, false)
        && (card::is_type(d, c, TYPE_FIELD) || duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filter),tp,LOCATION_HAND|LOCATION_GRAVE,0,nil,tp)
    let nvf = aux::necro_valley_filter(|d, c| filter(d, c, a.tp));
    let g = duel::get_matching_group(d, Some(&nvf), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
        // local tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // if tc:IsType(TYPE_FIELD) then
        if card::is_type(d, tc, TYPE_FIELD) {
            // Duel.ActivateFieldSpell(tc,e,tp,eg,ep,ev,re,r,rp)
            duel::activate_field_spell(d, Some(tc), a, None);
        } else {
            // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
            duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
            // local te=tc:GetActivateEffect()
            let te = card::get_activate_effect(d, tc).first().copied().nil();
            // local tep=tc:GetControler()
            let tep = card::get_controler(d, tc);
            // local cost=te:GetCost()
            // if cost then cost(te,tep,eg,ep,ev,re,r,rp,1) end
            if let Some(cost) = effect::get_cost(d, te) {
                let args = [
                    Value::Effect(te),
                    Value::Int(tep as i64),
                    a.eg.clone().map_or(Value::Nil, Value::Group),
                    Value::Int(a.ep as i64),
                    Value::Int(a.ev as i64),
                    Value::effect(a.re),
                    Value::Int(a.r as i64),
                    Value::Int(a.rp as i64),
                    Value::Int(1),
                ];
                cost.call(d, &args);
            }
        }
    }
}
// function s.econ(e,tp,eg,ep,ev,re,r,rp)
fn econ(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsEnvironment(CARD_UMI)
    duel::is_environment(d, CARD_UMI, None, None)
}
// function s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_WATER) and c:IsAbleToRemoveAsCost()
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_able_to_remove_as_cost(d, c, None)
}
// function s.remcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn remcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if Duel.Remove(g,POS_FACEUP,REASON_COST+REASON_TEMPORARY)~=0 then
    if duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST + REASON_TEMPORARY, None, None) != 0 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetLabelObject(g:GetFirst())
        let first = group::get_first(d, &g).nil();
        effect::set_label_object(d, e1, Value::Card(first));
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
    true
}
// function s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetLabelObject())
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, tc, None, None);
}
// function s.remop(e,tp,eg,ep,ev,re,r,rp)
fn remop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetTargetRange(LOCATION_ONFIELD,0)
    effect::set_target_range(d, e1, LOCATION_ONFIELD, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSpellTrap))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_spell_trap(d, c))));
    // e1:SetValue(aux.indoval)
    effect::set_value_fn(d, e1, aux::indoval());
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not s.econ(e,tp,eg,ep,ev,re,r,rp) then return false end
    if !econ(d, a) {
        return false;
    }
    // local tc=Duel.GetAttacker()
    let tc0 = duel::get_attacker(d);
    // local bc=Duel.GetAttackTarget()
    let bc0 = duel::get_attack_target(d);
    // if not bc then return false end
    let Some(mut bc) = bc0 else { return false };
    let mut tc = tc0.nil();
    // if tc:IsControler(1-tp) then tc,bc=bc,tc end
    if card::is_controler(d, tc, 1 - a.tp) {
        std::mem::swap(&mut tc, &mut bc);
    }
    // if tc:IsFaceup() and tc:GetOriginalLevel()>=5 and tc:IsAttribute(ATTRIBUTE_WATER) then
    if card::is_faceup(d, tc) && card::get_original_level(d, tc) >= 5 && card::is_attribute(d, tc, ATTRIBUTE_WATER) {
        // e:SetLabelObject(bc)
        effect::set_label_object(d, a.e, Value::Card(bc));
        // return true
        true
    } else {
        // else return false end
        false
    }
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(bc)), 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card();
    // if bc and bc:IsRelateToBattle() then
    if let Some(bc) = bc {
        if card::is_relate_to_battle(d, bc) {
            // Duel.Destroy(bc,REASON_EFFECT)
            duel::destroy(d, bc, REASON_EFFECT, None, None);
        }
    }
}
