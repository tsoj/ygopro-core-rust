//! Cherubini, Ebon Angel of the Burning Abyss
use super::prelude::*;

pub const ID: u32 = 58699500;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BURNING_ABYSS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunction(Card.IsLevel,3),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, _sc, _st, _tp| card::is_level(d, c, &[3]))), 2, Some(2), None, None, None);
    // indestructable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.indtg)
    effect::set_target(d, e1, Some(effect_card_fn(indtg)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy replace
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetTarget(s.desreptg)
    effect::set_target(d, e2, Some(cost_target(desreptg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // boost
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetLabel(0)
    effect::set_label(d, e3, &[0]);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(s.atkcost)
    effect::set_cost(d, e3, Some(cost_target(atkcost)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.indtg(e,c)`
fn indtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandler():GetLinkedGroup():IsContains(c)
    let h = effect::get_handler(d, e).nil();
    let g = card::get_linked_group(d, h);
    group::is_contains(&g, c)
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and (c:IsReason(REASON_BATTLE) or rp~=tp)
    //     and Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_ONFIELD,0,1,c,e) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE)
            && (card::is_reason(d, c, REASON_BATTLE) || a.rp != tp)
            && duel::is_existing_matching_card(d, Some(&|d, cc| desfilter(d, cc, e)), tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,1,c,e)
        let g = duel::select_matching_card(d, tp, Some(&|d, cc| desfilter(d, cc, e)), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_REPLACE)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_REPLACE, None, None);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.desfilter(c,e)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return not c:IsImmuneToEffect(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED)
    !card::is_immune_to_effect(d, c, e) && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(3) and (c:GetBaseAttack()>0 or c:GetBaseDefense()>0) and c:IsAbleToGraveAsCost()
    card::is_level(d, c, &[3]) && (card::get_base_attack(d, c) > 0 || card::get_base_defense(d, c) > 0) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_BURNING_ABYSS)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BURNING_ABYSS as u16])
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=1 then return false end
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_DECK,0,1,nil)
        //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
    // e:SetLabelObject(g:GetFirst())
    let sc = group::get_first(d, &g);
    effect::set_label_object(d, a.e, sc.map_or(Value::Nil, Value::Card));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local sc=e:GetLabelObject()
    let sc = effect::get_label_object(d, a.e);
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and sc then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        if let Value::Card(sc) = sc {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(sc:GetAttack())
            let atk = card::get_attack(d, sc);
            effect::set_value(d, e1, atk as i64);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_UPDATE_DEFENSE)
            effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
            // e2:SetValue(sc:GetDefense())
            let def = card::get_defense(d, sc);
            effect::set_value(d, e2, def as i64);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
