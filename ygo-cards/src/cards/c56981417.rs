//! Spellbook of the Master
use super::prelude::*;

pub const ID: u32 = 56981417;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SPELLBOOK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // copy a "Spellbook" Spell
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
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
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_SPELLCASTER))), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.cffilter(c)`
fn cffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPELLBOOK) and not c:IsPublic()
    card::is_set_card(d, c, &[SET_SPELLBOOK as u16]) && !card::is_public(d, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(s.cffilter,tp,LOCATION_HAND,0,1,e:GetHandler())
        return duel::is_existing_matching_card(d, Some(&cffilter), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cffilter,tp,LOCATION_HAND,0,1,1,e:GetHandler())
    let g = duel::select_matching_card(d, a.tp, Some(&cffilter), a.tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPELLBOOK) and not c:IsCode(id) and c:IsNormalSpell() and c:CheckActivateEffect(true,true,false)~=nil
    card::is_set_card(d, c, &[SET_SPELLBOOK as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_normal_spell(d, c)
        && card::check_activate_effect(d, c, true, true, false).is_some()
}
/// The arguments `(e,tp,eg,ep,ev,re,r,rp)` of the copied effect, as a Lua call passes them.
fn call_args(a: &EffArgs) -> Vec<Value> {
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
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local tg=e:GetLabelObject():GetTarget()
        let te = effect::get_label_object(d, a.e).as_effect().nil();
        let tg = effect::get_target(d, te);
        // return tg and tg(e,tp,eg,ep,ev,re,r,rp,0,chkc)
        return match tg {
            Some(tg) => {
                let mut args = call_args(a);
                args.push(Value::Int(0));
                args.push(Value::Card(chkc));
                tg.call(d, &args).truthy()
            }
            None => false,
        };
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
    // e:SetCategory(0)
    effect::set_category(d, a.e, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local te=g:GetFirst():CheckActivateEffect(true,true,false)
    let gc = group::get_first(d, &g).nil();
    let te = card::check_activate_effect(d, gc, true, true, false).nil().0;
    // Duel.ClearTargetCard()
    duel::clear_target_card(d);
    // e:SetProperty(te:GetProperty())
    let (prop1, prop2) = effect::get_property(d, te);
    effect::set_property(d, a.e, prop1, prop2);
    // e:SetLabel(te:GetLabel())
    let label = effect::get_labels(d, te);
    effect::set_label(d, a.e, &label);
    // e:SetLabelObject(te:GetLabelObject())
    let lo = effect::get_label_object(d, te);
    effect::set_label_object(d, a.e, lo);
    // local tg=te:GetTarget()
    // if tg then tg(e,tp,eg,ep,ev,re,r,rp,1) end
    if let Some(tg) = effect::get_target(d, te) {
        let mut args = call_args(a);
        args.push(Value::Int(1));
        tg.call(d, &args);
    }
    // te:SetLabel(e:GetLabel())
    let label = effect::get_labels(d, a.e);
    effect::set_label(d, te, &label);
    // te:SetLabelObject(e:GetLabelObject())
    let lo = effect::get_label_object(d, a.e);
    effect::set_label_object(d, te, lo);
    // e:SetLabelObject(te)
    effect::set_label_object(d, a.e, Value::Effect(te));
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    let te = effect::get_label_object(d, a.e).as_effect().nil();
    // if te:GetHandler():IsRelateToEffect(e) then
    let th = effect::get_handler(d, te).nil();
    if card::is_relate_to_effect(d, th, a.e) {
        // e:SetLabel(te:GetLabel())
        let label = effect::get_labels(d, te);
        effect::set_label(d, a.e, &label);
        // e:SetLabelObject(te:GetLabelObject())
        let lo = effect::get_label_object(d, te);
        effect::set_label_object(d, a.e, lo);
        // local op=te:GetOperation()
        // if op then op(e,tp,eg,ep,ev,re,r,rp) end
        if let Some(opf) = effect::get_operation(d, te) {
            opf.call(d, &call_args(a));
        }
        // te:SetLabel(e:GetLabel())
        let label = effect::get_labels(d, a.e);
        effect::set_label(d, te, &label);
        // te:SetLabelObject(e:GetLabelObject())
        let lo = effect::get_label_object(d, a.e);
        effect::set_label_object(d, te, lo);
    }
}
