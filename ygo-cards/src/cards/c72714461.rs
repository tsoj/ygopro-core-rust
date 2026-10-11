//! Wisdom-Eye Magician
use super::prelude::*;

pub const ID: u32 = 72714461;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGICIAN, SET_PERFORMAPAL], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // pendulum set
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCondition(s.pencon)
    effect::set_condition(d, e2, Some(condition(pencon)));
    // e2:SetTarget(s.pentg)
    effect::set_target(d, e2, Some(cost_target(pentg)));
    // e2:SetOperation(s.penop)
    effect::set_operation(d, e2, Some(operation(penop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // scale
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e3, Some(cost::self_discard()));
    // e3:SetTarget(s.sctg)
    effect::set_target(d, e3, Some(cost_target(sctg)));
    // e3:SetOperation(s.scop)
    effect::set_operation(d, e3, Some(operation(scop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAGICIAN) or c:IsSetCard(SET_PERFORMAPAL)
    card::is_set_card(d, c, &[SET_MAGICIAN as u16]) || card::is_set_card(d, c, &[SET_PERFORMAPAL as u16])
}
/// `s.pencon(e,tp,...)`
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_PZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_PZONE, 0, 1, h.into())
}
/// `s.penfilter(c)`
fn penfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAGICIAN) and c:IsType(TYPE_PENDULUM) and not c:IsCode(id) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_MAGICIAN as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && !card::is_code(d, c, &[ID])
        && !card::is_forbidden(d, c)
}
/// `s.pentg(e,tp,...,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.penfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&penfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.penop(e,tp,...)`
fn penop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // if Duel.Destroy(e:GetHandler(),REASON_EFFECT)~=0 then
    if duel::destroy(d, CardSet::from_card(h), REASON_EFFECT, None, None) != 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
        // local g=Duel.SelectMatchingCard(tp,s.penfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&penfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g);
        // if tc then
        if let Some(tc) = tc {
            // Duel.MoveToField(tc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
            duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
        }
    }
}
/// `s.scfilter(c)`
fn scfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetLeftScale()~=c:GetOriginalLeftScale()
    let cur = card::get_left_scale(d, c);
    cur != card::get_original_left_scale(d, c)
}
/// `s.sctg(e,tp,...,chk,chkc)`
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_PZONE) and chkc:IsControler(tp) and s.scfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_PZONE) && card::is_controler(d, chkc, a.tp) && scfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.scfilter,tp,LOCATION_PZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&scfilter), a.tp, LOCATION_PZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.scfilter,tp,LOCATION_PZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&scfilter), a.tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.scop(e,tp,...)`
fn scop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            let h = effect::get_handler(d, a.e).nil();
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LSCALE)
            effect::set_code(d, e1, EFFECT_CHANGE_LSCALE);
            // e1:SetValue(tc:GetOriginalLeftScale())
            let lv = card::get_original_left_scale(d, tc);
            effect::set_value(d, e1, lv as i64);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_CHANGE_RSCALE)
            effect::set_code(d, e2, EFFECT_CHANGE_RSCALE);
            // e2:SetValue(tc:GetOriginalRightScale())
            let rv = card::get_original_right_scale(d, tc);
            effect::set_value(d, e2, rv as i64);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
