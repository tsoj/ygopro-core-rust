//! Millennium-Eyes Illusionist
use super::prelude::*;

pub const ID: u32 = 89785779;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EYES_RESTRICT], listed_names: &[64631466], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and c:IsAbleToChangeControler()
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_MZONE,0,1,nil,c,tp)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_EFFECT)
        && card::is_able_to_change_controler(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, cc| eqfilter(d, cc, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.eqfilter(c,ec,tp)`
fn eqfilter(d: &mut Duel, c: CardId, ec: CardId, tp: u8) -> bool {
    // local eff={c:GetCardEffect(id)}
    let eff = card::get_card_effect(d, c, ID);
    // if c:IsFacedown() or ((not c:IsSetCard(SET_EYES_RESTRICT) or not c:IsType(TYPE_FUSION)) and not c:IsCode(64631466)) then return false end
    if card::is_facedown(d, c)
        || ((!card::is_set_card(d, c, &[SET_EYES_RESTRICT as u16]) || !card::is_type(d, c, TYPE_FUSION)) && !card::is_code(d, c, &[64631466]))
    {
        return false;
    }
    // for _,te in ipairs(eff) do
    for te in eff {
        // if te:GetValue()(ec,c,tp) then return true end
        if call_value(d, te, &[Value::Card(ec), Value::Card(c), Value::Int(tp as i64)]).truthy() {
            return true;
        }
    }
    // return false
    false
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and s.filter(chkc,tp) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_on_field(d, chkc) && filter(d, chkc, a.tp) && chkc != h;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and Duel.IsExistingTarget(s.filter,tp,0,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.tp)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.tp)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, 0, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local tc1=Duel.GetFirstTarget()
    let tc1 = duel::first_target(d);
    // if not tc1 then return end
    let Some(tc1) = tc1 else {
        return;
    };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_MZONE,0,1,1,nil,tc1,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| eqfilter(d, c, tc1, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local tc2=g:GetFirst()
    let tc2 = group::get_first(d, &g);
    // if not tc2 then return end
    let Some(tc2) = tc2 else {
        return;
    };
    // local te=tc2:GetCardEffect(id)
    let te = card::get_card_effect(d, tc2, ID).first().copied().nil();
    // if tc1:IsFaceup() and tc1:IsRelateToEffect(e) and tc1:IsControler(1-tp) then
    if card::is_faceup(d, tc1) && card::is_relate_to_effect(d, tc1, a.e) && card::is_controler(d, tc1, 1 - a.tp) {
        // te:GetOperation()(tc2,te:GetLabelObject(),tp,tc1)
        let label_object = effect::get_label_object(d, te);
        call_operation(d, te, &[Value::Card(tc2), label_object, Value::Int(a.tp as i64), Value::Card(tc1)]);
    }
}
/// `s.thfilter(c,e)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_EYES_RESTRICT) and c:IsType(TYPE_FUSION)) or c:IsCode(64631466)
    (card::is_set_card(d, c, &[SET_EYES_RESTRICT as u16]) && card::is_type(d, c, TYPE_FUSION)) || card::is_code(d, c, &[64631466])
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.thfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &thfilter, 1, NO_EX)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.SendtoHand(e:GetHandler(),nil,REASON_EFFECT)
        duel::send_to_hand(d, h, None, REASON_EFFECT, None);
    }
}
/// `te:GetValue()(...)` (a number value is not callable)
fn call_value(d: &mut Duel, te: EffectId, args: &[Value]) -> Value {
    match effect::get_value(d, te) {
        Value::Fn(f) => f.call(d, args),
        _ => crate::lua_error("attempt to call a number value (local 'val')"),
    }
}
/// `te:GetOperation()(...)` (nil is not callable)
fn call_operation(d: &mut Duel, te: EffectId, args: &[Value]) -> Value {
    match effect::get_operation(d, te) {
        Some(f) => f.call(d, args),
        None => crate::lua_error("attempt to call a nil value (local 'op')"),
    }
}
