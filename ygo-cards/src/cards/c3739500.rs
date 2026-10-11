//! End of the World Ruler
use super::prelude::*;

pub const ID: u32 = 3739500;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_END_OF_THE_WORLD}
    listed_series: &[SET_END_OF_THE_WORLD as u32],
    // s.listed_names={72426662} --"Demise, King of Armageddon"
    listed_names: &[72426662],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // This card's name becomes "Demise, King of Armageddon" while in the hand or field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    // e1:SetValue(72426662)
    effect::set_value(d, e1, 72426662);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply the effect of a Ritual Spell that Ritual Summons
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.AND(Cost.SelfReveal,Cost.PayLP(2000),s.applycost))
    effect::set_cost(d, e2, Some(cost::and(vec![cost::self_reveal(), cost::pay_lp(cost::PayLp::Amount(2000)), cost_target(applycost)])));
    // e2:SetTarget(s.applytg)
    effect::set_target(d, e2, Some(cost_target(applytg)));
    // e2:SetOperation(s.applyop)
    effect::set_operation(d, e2, Some(operation(applyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy as many other cards on the field as possible, then this card gains 2900 ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(Cost.PayLP(2000))
    effect::set_cost(d, e3, Some(cost::pay_lp(cost::PayLp::Amount(2000))));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn applyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRitualSpell() and c:IsAbleToRemoveAsCost() and c:CheckActivateEffect(true,true,false)~=nil
    card::is_ritual_spell(d, c) && card::is_able_to_remove_as_cost(d, c, None) && card::check_activate_effect(d, c, true, true, false).is_some()
}
fn applycost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.applyfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&applyfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
// The arguments of `tg(e,tp,eg,ep,ev,re,r,rp, ...)` for the effect `e`
fn event_args(a: &EffArgs) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ]
}
fn applytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then
    if a.chkc().is_some() {
        // local te=e:GetLabelObject()
        let _te = effect::get_label_object(d, a.e);
        // return tg and tg(e,tp,eg,ep,ev,re,r,rp,0,chkc)
        // (`tg` is a global here, so it is nil and the function returns nil)
        return false;
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.applyfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&applyfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.applyfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&applyfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // local te=g:GetFirst():CheckActivateEffect(true,true,false)
    let first = group::get_first(d, &g).nil();
    let te = card::check_activate_effect(d, first, true, true, false).nil().0;
    // e:SetLabel(te:GetLabel())
    let te_label = effect::get_label(d, te);
    effect::set_label(d, a.e, &[te_label]);
    // e:SetLabelObject(te:GetLabelObject())
    let te_obj = effect::get_label_object(d, te);
    effect::set_label_object(d, a.e, te_obj);
    // local tg=te:GetTarget()
    let tg = effect::get_target(d, te);
    // e:SetProperty(te:GetProperty())
    let (p1, p2) = effect::get_property(d, te);
    effect::set_property(d, a.e, p1, p2);
    // if tg then tg(e,tp,eg,ep,ev,re,r,rp,1) end
    if let Some(tg) = tg {
        let mut args = event_args(a);
        args.push(Value::Int(1));
        tg.call(d, &args);
    }
    // te:SetLabel(e:GetLabel())
    let l = effect::get_label(d, a.e);
    effect::set_label(d, te, &[l]);
    // te:SetLabelObject(e:GetLabelObject())
    let o = effect::get_label_object(d, a.e);
    effect::set_label_object(d, te, o);
    // e:SetLabelObject(te)
    effect::set_label_object(d, a.e, Value::Effect(te));
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
fn applyop(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    let te = effect::get_label_object(d, a.e).as_effect();
    // if te then
    if let Some(te) = te {
        // e:SetLabel(te:GetLabel())
        let l = effect::get_label(d, te);
        effect::set_label(d, a.e, &[l]);
        // e:SetLabelObject(te:GetLabelObject())
        let o = effect::get_label_object(d, te);
        effect::set_label_object(d, a.e, o);
        // local op=te:GetOperation()
        // if op then op(e,tp,eg,ep,ev,re,r,rp) end
        if let Some(op) = effect::get_operation(d, te) {
            op.call(d, &event_args(a));
        }
        // te:SetLabel(e:GetLabel())
        let l = effect::get_label(d, a.e);
        effect::set_label(d, te, &[l]);
        // te:SetLabelObject(e:GetLabelObject())
        let o = effect::get_label_object(d, a.e);
        effect::set_label_object(d, te, o);
    }
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc = if card::is_relate_to_effect(d, c, a.e) { Some(c) } else { None };
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,exc)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, exc.into());
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 and c:IsRelateToEffect(e) and c:IsFaceup() then
    if !g.is_empty()
        && duel::destroy(d, g, REASON_EFFECT, None, None) > 0
        && card::is_relate_to_effect(d, c, a.e)
        && card::is_faceup(d, c)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // This card gains 2900 ATK
        // c:UpdateAttack(2900)
        card::update_attack(d, c, 2900, None, None, None);
    }
}
