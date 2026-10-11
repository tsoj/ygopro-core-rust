//! Cipher Bit
use super::prelude::*;

pub const ID: u32 = 99397762;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GALAXY_EYES, SET_CIPHER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Attach this card to 1 "Galaxy-Eyes" or "Cipher" Xyz Monster you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,mc,tp)`
fn tgfilter(d: &mut Duel, c: CardId, mc: CardId, tp: u8) -> bool {
    // return c:IsSetCard({SET_GALAXY_EYES,SET_CIPHER}) and c:IsType(TYPE_XYZ) and c:IsFaceup()
    //     and mc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT)
    card::is_set_card(d, c, &[SET_GALAXY_EYES as u16, SET_CIPHER as u16])
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_faceup(d, c)
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), Some(REASON_EFFECT))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,c,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, c, tp);
    }
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE)
    //     and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,c,tp) end
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,c,tp)
    duel::select_target(d, tp, Some(&|d, x| tgfilter(d, x, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `function(e,re,r,rp) return r&(REASON_BATTLE|REASON_EFFECT)>0 end`
fn indval(_d: &mut Duel, a: Args) -> bool {
    let r = a.int(2) as u32;
    (r & (REASON_BATTLE | REASON_EFFECT)) > 0
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) and c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) && !card::is_immune_to_effect(d, tc, e) && card::is_relate_to_effect(d, c, e) {
        // c:CancelToGrave()
        card::cancel_to_grave(d, c, true);
        // Duel.Overlay(tc,c)
        duel::overlay(d, tc, c, false);
        // The next time it would be destroyed by battle or card effect this turn, it is not destroyed
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetValue(function(e,re,r,rp) return r&(REASON_BATTLE|REASON_EFFECT)>0 end)
        effect::set_value_fn(d, e1, lua_fn(indval));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
