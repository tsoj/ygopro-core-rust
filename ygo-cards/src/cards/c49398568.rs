//! Serial Spell
use super::prelude::*;

pub const ID: u32 = 49398568;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:GetActiveType()==TYPE_SPELL and re:IsHasType(EFFECT_TYPE_ACTIVATE) and rp==tp
    let re = a.re.nil();
    effect::get_active_type(d, re) == TYPE_SPELL && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && a.rp == a.tp
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsDiscardable() and c:IsAbleToGraveAsCost()
    card::is_discardable(d, c, None, None) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local hg=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let mut hg = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    // hg:RemoveCard(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    group::remove_card(&mut hg, &CardSet::from_card(h));
    // if chk==0 then return #hg>0 and hg:FilterCount(s.cfilter,nil)==#hg end
    if a.chk == Some(0) {
        let n = hg.len();
        return n > 0 && group::filter_count(d, &hg, &cfilter, NO_EX) == n;
    }
    // Duel.SendtoGrave(hg,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, hg, REASON_COST | REASON_DISCARD, None, None);
    true
}
/// The arguments `(e,tp,eg,ep,ev,re,r,rp)` of the effect `a`, as Lua passes them
fn args8(a: &EffArgs) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ]
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ftg=re:GetTarget()
    let re = a.re.nil();
    let ftg = effect::get_target(d, re);
    // if chkc then return ftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc) end
    if a.chkc().is_some() {
        let mut args = args8(a);
        args.push(a.chk.map_or(Value::Nil, Value::Int));
        args.extend(a.extra.iter().cloned());
        return ftg.nil().call(d, &args).truthy();
    }
    // if chk==0 then return not ftg or ftg(e,tp,eg,ep,ev,re,r,rp,chk) end
    if a.chk == Some(0) {
        return match ftg {
            None => true,
            Some(f) => {
                let mut args = args8(a);
                args.push(a.chk.map_or(Value::Nil, Value::Int));
                f.call(d, &args).truthy()
            }
        };
    }
    // if re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then e:SetProperty(EFFECT_FLAG_CARD_TARGET) end
    if effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
    }
    // if ftg then ftg(e,tp,eg,ep,ev,re,r,rp,chk) end
    if let Some(f) = ftg {
        let mut args = args8(a);
        args.push(a.chk.map_or(Value::Nil, Value::Int));
        f.call(d, &args);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local fop=re:GetOperation()
    let fop = effect::get_operation(d, a.re.nil());
    // fop(e,tp,eg,ep,ev,re,r,rp)
    fop.nil().call(d, &args8(a));
}
