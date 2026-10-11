//! Hazy Pillar
use super::prelude::*;

pub const ID: u32 = 83108603;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HAZY_FLAME], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // decrease tribute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_DECREASE_TRIBUTE)
    effect::set_code(d, e2, EFFECT_DECREASE_TRIBUTE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_HAZY_FLAME))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_HAZY_FLAME as u16]))));
    // e2:SetValue(0x1)
    effect::set_value(d, e2, 0x1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // material
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.mattg)
    effect::set_target(d, e3, Some(cost_target(mattg)));
    // e3:SetOperation(s.matop)
    effect::set_operation(d, e3, Some(operation(matop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.xyzfilter(c,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ)
    //   and Duel.IsExistingMatchingCard(s.matfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,c)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_XYZ)
        && duel::is_existing_matching_card(d, Some(&matfilter), tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, c.into())
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_HAND) or c:IsFaceup()) and c:IsSetCard(SET_HAZY_FLAME) and c:IsMonster() and not c:IsType(TYPE_TOKEN)
    (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_set_card(d, c, &[SET_HAZY_FLAME as u16])
        && card::is_monster(d, c)
        && !card::is_type(d, c, TYPE_TOKEN)
}
/// `s.mattg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn mattg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.xyzfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && xyzfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| xyzfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, tp, Some(&|d, c| xyzfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.matop(e,tp,eg,ep,ev,re,r,rp)`
fn matop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
        // local g=Duel.SelectMatchingCard(tp,s.matfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,1,tc)
        let g = duel::select_matching_card(d, tp, Some(&matfilter), tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, 1, false, tc.into()).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Overlay(tc,g,true)
            duel::overlay(d, tc, g, true);
        }
    }
}
