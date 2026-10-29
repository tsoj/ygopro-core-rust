//! Predaplant Verte Anaconda
use super::prelude::*;

pub const ID: u32 = 70369116;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FUSION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // The attribute of 1 face-up monster becomes DARK until the end of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.attg)
    effect::set_target(d, e1, Some(cost_target(attg)));
    // e1:SetOperation(s.atop)
    effect::set_operation(d, e1, Some(operation(atop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // This effect becomes the effect of 1 "Fusion" or "Polymerization" Normal or Quick-Play Spel when that card is activated
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atfilter(c)`
fn atfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not c:IsAttribute(ATTRIBUTE_DARK)
    card::is_faceup(d, c) && !card::is_attribute(d, c, ATTRIBUTE_DARK)
}
/// `s.attg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn attg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.atfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && atfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&atfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.atfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&atfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
        // e1:SetValue(ATTRIBUTE_DARK)
        effect::set_value(d, e1, ATTRIBUTE_DARK as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.copfilter(c)`
fn copfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToGraveAsCost() and c:IsSetCard(SET_FUSION) and (c:IsNormalSpell() or c:IsQuickPlaySpell())
    //     and c:CheckActivateEffect(true,true,false)~=nil
    card::is_able_to_grave_as_cost(d, c)
        && card::is_set_card(d, c, &[SET_FUSION as u16])
        && (card::is_normal_spell(d, c) || card::is_quick_play_spell(d, c))
        && card::check_activate_effect(d, c, true, true, false).is_some()
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckLPCost(tp,2000) and Duel.IsExistingMatchingCard(s.copfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, a.tp, 2000) && duel::is_existing_matching_card(d, Some(&copfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.PayLPCost(tp,2000)
    duel::pay_lp_cost(d, a.tp, 2000);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then
    if a.chkc().is_some() {
        // local te=e:GetLabelObject()
        let _te = effect::get_label_object(d, a.e);
        // return tg and tg(e,tp,eg,ep,ev,re,r,rp,0,chkc)
        // (tg is the global nil here, so the Lua returns nil)
        return false;
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.copfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&copfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.copfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&copfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if not Duel.SendtoGrave(g,REASON_COST) then return end
    // (SendtoGrave returns a number, which is always true in Lua, so the script goes on)
    duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
    // local te=g:GetFirst():CheckActivateEffect(true,true,false)
    let sc = group::get_first(d, &g).nil();
    let (te, _info) = card::check_activate_effect(d, sc, true, true, false).nil();
    // e:SetLabel(te:GetLabel())
    let label = effect::get_labels(d, te);
    effect::set_label(d, a.e, &label);
    // e:SetLabelObject(te:GetLabelObject())
    let lo = effect::get_label_object(d, te);
    effect::set_label_object(d, a.e, lo);
    // local tg=te:GetTarget()
    let tg = effect::get_target(d, te);
    // if tg then tg(e,tp,eg,ep,ev,re,r,rp,1) end
    if let Some(tg) = tg {
        tg.call(
            d,
            &[
                Value::Effect(a.e),
                Value::Int(a.tp as i64),
                a.eg.clone().map_or(Value::Nil, Value::Group),
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                Value::effect(a.re),
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
                Value::Int(1),
            ],
        );
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
    let te = effect::get_label_object(d, a.e);
    // if te then
    if let Some(te) = te.as_effect() {
        // e:SetLabel(te:GetLabel())
        let label = effect::get_labels(d, te);
        effect::set_label(d, a.e, &label);
        // e:SetLabelObject(te:GetLabelObject())
        let lo = effect::get_label_object(d, te);
        effect::set_label_object(d, a.e, lo);
        // local op=te:GetOperation()
        let op = effect::get_operation(d, te);
        // if op then op(e,tp,eg,ep,ev,re,r,rp) end
        if let Some(op) = op {
            op.call(
                d,
                &[
                    Value::Effect(a.e),
                    Value::Int(a.tp as i64),
                    a.eg.clone().map_or(Value::Nil, Value::Group),
                    Value::Int(a.ep as i64),
                    Value::Int(a.ev as i64),
                    Value::effect(a.re),
                    Value::Int(a.r as i64),
                    Value::Int(a.rp as i64),
                ],
            );
        }
        // te:SetLabel(e:GetLabel())
        let label = effect::get_labels(d, a.e);
        effect::set_label(d, te, &label);
        // te:SetLabelObject(e:GetLabelObject())
        let lo = effect::get_label_object(d, a.e);
        effect::set_label_object(d, te, lo);
    }
    // You cannot Special Summon monsters for the rest of this turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
