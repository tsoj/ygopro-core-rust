//! Metaphys Horus Dragon
use super::prelude::*;

pub const ID: u32 = 36898537;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Check materials used for its synchro summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    // e1:SetValue(s.valcheck)
    // (a value function that sets a label and returns nothing: the core reads nil as 0)
    effect::set_value_fn(d, e1, effect_card_value(|d, e, c| {
        valcheck(d, e, c);
        0
    }));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate the following effects, depending on the monster type used
    // Normal monster: this card becomes unaffected by other card effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCondition(s.immcon)
    effect::set_condition(d, e2, Some(condition(immcon)));
    // e2:SetOperation(s.immop)
    effect::set_operation(d, e2, Some(operation(immop)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Effect monster: negate 1 other card on the field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e3, CATEGORY_DISABLE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCondition(s.negcon)
    effect::set_condition(d, e3, Some(condition(negcon)));
    // e3:SetTarget(s.negtg)
    effect::set_target(d, e3, Some(cost_target(negtg)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // e3:SetLabelObject(e1)
    effect::set_label_object(d, e3, Value::Effect(e1));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Pendulum monster: take control 1 of opponent's monsters
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e4, CATEGORY_CONTROL);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCondition(s.ctcon)
    effect::set_condition(d, e4, Some(condition(ctcon)));
    // e4:SetTarget(s.cttg)
    effect::set_target(d, e4, Some(cost_target(cttg)));
    // e4:SetOperation(s.ctop)
    effect::set_operation(d, e4, Some(operation(ctop)));
    // e4:SetLabelObject(e1)
    effect::set_label_object(d, e4, Value::Effect(e1));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local tpe=0
    let mut tpe: u32 = 0;
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // if not tc:IsType(TYPE_TUNER) then
        if !card::is_type(d, tc, TYPE_TUNER) {
            // tpe=(tpe|tc:GetType())
            tpe |= card::get_type(d, tc);
        }
    }
    // e:SetLabel(tpe)
    effect::set_label(d, e, &[tpe as i64]);
}
/// The label of the material check effect (`e:GetLabelObject():GetLabel()`)
fn material_label(d: &Duel, e: EffectId) -> i64 {
    let check = effect::get_label_object(d, e).as_effect().nil();
    effect::get_label(d, check)
}
/// `s.immcon(e,tp,eg,ep,ev,re,r,rp)`
fn immcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned() and (e:GetLabelObject():GetLabel()&TYPE_NORMAL)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h) && (material_label(d, a.e) & TYPE_NORMAL as i64) != 0
}
/// `s.immop(e,tp,eg,ep,ev,re,r,rp)`
fn immop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3100)
        effect::set_description(d, e1, 3100);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCode(EFFECT_IMMUNE_EFFECT)
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        // e1:SetValue(s.efilter)
        effect::set_value_fn(d, e1, lua_fn(|d, a| efilter(d, a.e(0), a.e(1))));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, e: EffectId, te: EffectId) -> bool {
    // return te:GetOwner()~=e:GetOwner()
    effect::get_owner(d, te) != effect::get_owner(d, e)
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned() and (e:GetLabelObject():GetLabel()&TYPE_EFFECT)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h) && (material_label(d, a.e) & TYPE_EFFECT as i64) != 0
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsNegatable() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_negatable(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,e:GetHandler())
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_negatable(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned() and (e:GetLabelObject():GetLabel()&TYPE_PENDULUM)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h) && (material_label(d, a.e) & TYPE_PENDULUM as i64) != 0
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, 0, 0);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectMatchingCard(1-tp,Card.IsControlerCanBeChanged,1-tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, 1 - a.tp, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), 1 - a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else { return };
    // if Duel.GetControl(tc,tp,nil,nil,nil,1-tp)~=0 then
    if duel::get_control(d, tc, a.tp, 0, 0, None, Some(1 - a.tp)) != 0 {
        // Cannot attack
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3206)
        effect::set_description(d, e1, 3206);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
    }
}
