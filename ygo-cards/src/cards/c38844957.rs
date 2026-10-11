//! Urgent Ritual Art
use super::prelude::*;

pub const ID: u32 = 38844957;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRitualMonster),tp,LOCATION_MZONE,0,1,nil)
    !duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_ritual_monster(d, c)),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRitualSpell() and c:IsAbleToRemoveAsCost() and c:CheckActivateEffect(true,true,false)~=nil
    card::is_ritual_spell(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && card::check_activate_effect(d, c, true, true, false).is_some()
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// The arguments `(e,tp,eg,ep,ev,re,r,rp)` of an effect function, as the Lua values
fn base_values(a: &EffArgs) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        match &a.eg {
            Some(g) => Value::Group(g.clone()),
            None => Value::Nil,
        },
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        match a.re {
            Some(re) => Value::Effect(re),
            None => Value::Nil,
        },
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ]
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local te=e:GetLabelObject()
        let te = effect::get_label_object(d, a.e).as_effect().nil();
        // local tg=te:GetTarget()
        let tg = effect::get_target(d, te).nil();
        // return tg(e,tp,eg,ep,ev,re,r,rp,0,chkc)
        let mut args = base_values(a);
        args.push(Value::Int(0));
        args.push(Value::Card(chkc));
        return tg.call(d, &args).truthy();
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==0 then return false end
        if effect::get_label(d, a.e) == 0 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil)
        return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local te=g:GetFirst():CheckActivateEffect(true,true,false)
    let first = group::get_first(d, &g).nil();
    let te = card::check_activate_effect(d, first, true, true, false).map(|(te, _)| te);
    // e:SetLabelObject(te)
    let label = match te {
        Some(te) => Value::Effect(te),
        None => Value::Nil,
    };
    effect::set_label_object(d, a.e, label);
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetProperty(te:GetProperty())
    let te = te.nil();
    let (p1, p2) = effect::get_property(d, te);
    effect::set_property(d, a.e, p1, p2);
    // local tg=te:GetTarget()
    let tg = effect::get_target(d, te);
    // if tg then tg(e,tp,eg,ep,ev,re,r,rp,1) end
    if let Some(tg) = tg {
        let mut args = base_values(a);
        args.push(Value::Int(1));
        tg.call(d, &args);
    }
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    // if not te then return end
    let Some(te) = effect::get_label_object(d, a.e).as_effect() else { return };
    // local op=te:GetOperation()
    // if op then op(e,tp,eg,ep,ev,re,r,rp) end
    if let Some(op) = effect::get_operation(d, te) {
        op.call(d, &base_values(a));
    }
}
