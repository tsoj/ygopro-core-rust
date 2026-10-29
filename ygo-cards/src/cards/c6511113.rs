//! Traptrix Rafflesia
use super::prelude::*;

pub const ID: u32 = 6511113;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={id}
    listed_names: &[ID],
    // s.listed_series={SET_TRAPTRIX,SET_TRAP_HOLE,SET_HOLE}
    listed_series: &[SET_TRAPTRIX, SET_TRAP_HOLE, SET_HOLE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Unaffected by Trap effects while it has material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return e:GetHandler():GetOverlayCount()>0 end)
    effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        card::get_overlay_count(d, h) > 0
    })));
    // e1:SetValue(function(e,te) return te:IsTrapEffect() end)
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| effect::is_trap_effect(d, a.e(1))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // "Traptrix" monsters you control, except "Traptrix Rafflesia", cannot be destroyed by battle or card effects
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD);
    // e2a:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2a, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2a, LOCATION_MZONE, 0);
    // e2a:SetTarget(function(e,c) return c:IsSetCard(SET_TRAPTRIX) and not c:IsCode(id) end)
    effect::set_target(d, e2a, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_TRAPTRIX as u16]) && !card::is_code(d, c, &[ID]))));
    // e2a:SetValue(1)
    effect::set_value(d, e2a, 1);
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2b, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // Your opponent cannot target "Traptrix" monsters you control, except "Traptrix Rafflesia", with card effects
    // local e2c=e2a:Clone()
    let e2c = effect::clone(d, e2a);
    // e2c:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2c, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2c:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2c, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2c:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2c, aux::tgoval());
    // c:RegisterEffect(e2c)
    card::register_effect(d, c, e2c, false);
    // This effect becomes that sent "Hole" Normal Trap's effect when that card is activated
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMING_BATTLE_START|TIMING_MSET|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMING_BATTLE_START | TIMING_MSET | TIMINGS_CHECK_MONSTER_E));
    // e3:SetCost(Cost.AND(Cost.DetachFromSelf(1),s.effcost))
    effect::set_cost(d, e3, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(1), None, None), cost_target(effcost)])));
    // e3:SetTarget(s.efftg)
    effect::set_target(d, e3, Some(cost_target(efftg)));
    // e3:SetOperation(s.effop)
    effect::set_operation(d, e3, Some(operation(effop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.copyfilter(c)`
fn copyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalTrap() and c:IsSetCard({SET_HOLE,SET_TRAP_HOLE}) and c:IsAbleToGraveAsCost()
    //     and c:CheckActivateEffect(false,true,true)~=nil
    card::is_normal_trap(d, c)
        && card::is_set_card(d, c, &[SET_HOLE as u16, SET_TRAP_HOLE as u16])
        && card::is_able_to_grave_as_cost(d, c)
        && card::check_activate_effect(d, c, false, true, true).is_some()
}
/// The event values of a stored trap activation: `(ceg, cep, cev, cre, cr, crp)`.
fn event_values(info: &Option<ygo_core::duel::Event>) -> Vec<Value> {
    match info {
        Some(ev) => vec![
            ev.event_cards.clone().map_or(Value::Nil, Value::Group),
            Value::Int(ev.event_player as i64),
            Value::Int(ev.event_value as i64),
            Value::effect(ev.reason_effect),
            Value::Int(ev.reason as i64),
            Value::Int(ev.reason_player as i64),
        ],
        None => vec![Value::Nil; 6],
    }
}
/// The label object table `{te,ceg,cep,cev,cre,cr,crp}` (empty if the label object is not one).
fn stored(d: &Duel, e: EffectId) -> Vec<Value> {
    match effect::get_label_object(d, e) {
        Value::Multi(t) => t,
        _ => Vec::new(),
    }
}
/// Element `i` of a stored table (nil if missing).
fn at(t: &[Value], i: usize) -> Value {
    t.get(i).cloned().unwrap_or(Value::Nil)
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // e:SetLabel(-100)
    effect::set_label(d, e, &[-100]);
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.copyfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&copyfilter), tp, LOCATION_DECK, 0, NO_EX);
        // e:SetLabelObject(g)
        let empty = g.is_empty();
        effect::set_label_object(d, e, Value::Group(g));
        // return #g>0
        return !empty;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local sc=e:GetLabelObject():Select(tp,1,1,nil):GetFirst()
    let g = effect::get_label_object(d, e).as_group().cloned().nil();
    let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel).nil();
    // e:SetLabelObject(sc)
    effect::set_label_object(d, e, Value::Card(sc));
    // Duel.SendtoGrave(sc,REASON_COST)
    duel::send_to_grave(d, sc, REASON_COST, None, None);
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local te,ceg,cep,cev,cre,cr,crp=table.unpack(e:GetLabelObject())
        let t = stored(d, e);
        // return te and te:GetTarget() and te:GetTarget()(e,tp,ceg,cep,cev,cre,cr,crp,chk,chkc)
        let Some(te) = at(&t, 0).as_effect() else { return false };
        let Some(tg) = effect::get_target(d, te) else { return false };
        let mut args = vec![Value::Effect(e), Value::Int(tp as i64)];
        args.extend((1..=6).map(|i| at(&t, i)));
        args.push(Value::Int(a.chk.unwrap_or(0)));
        args.push(Value::Card(chkc));
        return tg.call(d, &args).truthy();
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=e:GetLabel()==-100
        let res = effect::get_label(d, e) == -100;
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return res
        return res;
    }
    // local sc=e:GetLabelObject()
    let sc = effect::get_label_object(d, e).as_card().nil();
    // local te,ceg,cep,cev,cre,cr,crp=sc:CheckActivateEffect(true,true,true)
    let (te, info) = card::check_activate_effect(d, sc, true, true, true).nil();
    // e:SetLabel(te:GetLabel())
    let label = effect::get_labels(d, te);
    effect::set_label(d, e, &label);
    // e:SetLabelObject(te:GetLabelObject())
    let lo = effect::get_label_object(d, te);
    effect::set_label_object(d, e, lo);
    // local tg=te:GetTarget()
    // if tg then
    if let Some(tg) = effect::get_target(d, te) {
        // e:SetProperty(te:GetProperty())
        let (p1, p2) = effect::get_property(d, te);
        effect::set_property(d, e, p1, p2);
        // tg(e,tp,ceg,cep,cev,cre,cr,crp,1)
        let mut args = vec![Value::Effect(e), Value::Int(tp as i64)];
        args.extend(event_values(&info));
        args.push(Value::Int(1));
        tg.call(d, &args);
        // te:SetLabel(e:GetLabel())
        let label = effect::get_labels(d, e);
        effect::set_label(d, te, &label);
        // te:SetLabelObject(e:GetLabelObject())
        let lo = effect::get_label_object(d, e);
        effect::set_label_object(d, te, lo);
        // Duel.ClearOperationInfo(0)
        duel::clear_operation_info(d, 0);
    }
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    // e:SetLabelObject({te,ceg,cep,cev,cre,cr,crp})
    let mut t = vec![Value::Effect(te)];
    t.extend(event_values(&info));
    effect::set_label_object(d, e, Value::Multi(t));
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local te,ceg,cep,cev,cre,cr,crp=table.unpack(e:GetLabelObject())
    let t = stored(d, e);
    // if not te then return end
    let Some(te) = at(&t, 0).as_effect() else { return };
    // local op=te:GetOperation()
    let op = effect::get_operation(d, te);
    // if op then
    if let Some(op) = op {
        // e:SetLabel(te:GetLabel())
        let label = effect::get_labels(d, te);
        effect::set_label(d, e, &label);
        // e:SetLabelObject(te:GetLabelObject())
        let lo = effect::get_label_object(d, te);
        effect::set_label_object(d, e, lo);
        // op(e,tp,ceg,cep,cev,cre,cr,crp)
        let mut args = vec![Value::Effect(e), Value::Int(tp as i64)];
        args.extend((1..=6).map(|i| at(&t, i)));
        op.call(d, &args);
    }
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    // e:SetLabelObject(nil)
    effect::set_label_object(d, e, Value::Nil);
}
