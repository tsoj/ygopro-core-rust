//! Cyber Raider
use super::prelude::*;

pub const ID: u32 = 39978267;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.desfilter(c)
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_EQUIP) and c:GetEquipTarget()
    card::is_type(d, c, TYPE_EQUIP) && card::get_equip_target(d, c).is_some()
}
// function s.eqfilter(c,ec)
fn eqfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsType(TYPE_EQUIP) and c:CheckEquipTarget(ec) and c:GetEquipTarget()
    card::is_type(d, c, TYPE_EQUIP) && card::check_equip_target(d, c, ec) && card::get_equip_target(d, c).is_some()
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local label=e:GetLabel()
    let label = effect::get_label(d, a.e);
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and chkc:IsType(TYPE_EQUIP) and chkc:GetEquipTarget()
    //   and (label==1 or (label==2 and chkc:CheckEquipTarget(c)))
    // end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE)
            && card::is_type(d, chkc, TYPE_EQUIP)
            && card::get_equip_target(d, chkc).is_some()
            && (label == 1 || (label == 2 && card::check_equip_target(d, chkc, c)));
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local b1=Duel.IsExistingTarget(s.desfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,nil)
    let b1 = duel::is_existing_target(d, Some(&desfilter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX);
    // local b2=Duel.IsExistingTarget(s.eqfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,nil,c)
    let b2 = duel::is_existing_target(d, Some(&|d, x| eqfilter(d, x, c)), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX);
    // local sel=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)})
    let sel = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(sel or 0)
    let sel_val = sel.map_or(0, |s| s as i64);
    effect::set_label(d, a.e, &[sel_val]);
    // if sel==1 then
    if sel_val == 1 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&desfilter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
        }
    // elseif sel==2 then
    } else if sel_val == 2 {
        // e:SetCategory(CATEGORY_EQUIP)
        effect::set_category(d, a.e, CATEGORY_EQUIP);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
        // local g=Duel.SelectTarget(tp,s.eqfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,nil,c)
        let g = duel::select_target(d, a.tp, Some(&|d, x| eqfilter(d, x, c)), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,tp,0)
            duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g), 1, a.tp, 0);
        }
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // local sel=e:GetLabel()
    let sel = effect::get_label(d, a.e);
    // if sel==0 then return end
    if sel == 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not (tc and tc:IsRelateToEffect(e)) then return end
    let Some(tc) = tc else { return };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if sel==1 and s.desfilter(tc) then
    if sel == 1 && desfilter(d, tc) {
        // Destroy 1 Equip Card on the field
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    // elseif sel==2 and c:IsRelateToEffect(e) and s.eqfilter(tc,c) then
    } else if sel == 2 && card::is_relate_to_effect(d, c, a.e) && eqfilter(d, tc, c) {
        // Equip 1 Equip Card on the field to this card
        // Duel.Equip(tp,tc,c)
        duel::equip(d, a.tp, tc, c, true, false);
    }
}
