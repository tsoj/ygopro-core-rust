//! Boompoline!!
//! (ドカンポリン)
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 14839621;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // return to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(1-tp,LOCATION_MZONE,nil,LOCATION_REASON_COUNT)+Duel.GetLocationCount(tp,LOCATION_MZONE,nil,LOCATION_REASON_COUNT)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, Some(LOCATION_REASON_COUNT), None).0
            + duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, Some(LOCATION_REASON_COUNT), None).0
            > 0;
    }
    // local zone=Duel.SelectDisableField(tp,1,LOCATION_MZONE,LOCATION_MZONE,0,true)
    let zone = duel::select_disable_field(d, a.tp, 1, LOCATION_MZONE, LOCATION_MZONE, Some(0), true);
    // Duel.Hint(HINT_ZONE,tp,zone)
    duel::hint(d, HINT_ZONE as u8, a.tp, zone as u64);
    // e:SetLabel(zone)
    effect::set_label(d, a.e, &[zone as i64]);
    true
}
/// `s.thfilter(c,tp,zone)`
fn thfilter(d: &mut Duel, c: CardId, tp: u8, zone: u32) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and aux.IsZone(c,zone,tp) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && aux::is_zone(d, c, zone, tp) && card::is_able_to_hand(d, c, None)
}
/// The zone stored on the activated effect (`e:GetLabelObject():GetLabel()` in `thcon`/`thtg`/`thop`)
fn stored_zone(d: &mut Duel, e: EffectId) -> u32 {
    let e1 = effect::get_label_object(d, e).as_effect().nil();
    effect::get_label(d, e1) as u32
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local zone=e:GetLabelObject():GetLabel()
    let zone = stored_zone(d, a.e);
    // return eg:IsExists(s.thfilter,1,nil,tp,zone)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| thfilter(d, c, a.tp, zone), 1, NO_EX)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local zone=e:GetLabelObject():GetLabel()
    let zone = stored_zone(d, a.e);
    // local g=eg:Filter(s.thfilter,nil,tp,zone)
    let eg = a.eg.clone().nil();
    let mut g = group::filter(d, &eg, &|d, c| thfilter(d, c, a.tp, zone), NO_EX);
    // g:AddCard(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    group::add_card(&mut g, &CardSet::from_card(h));
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 2, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zone=e:GetLabelObject():GetLabel()
    let zone = stored_zone(d, a.e);
    // local g=eg:Filter(s.thfilter,nil,tp,zone)
    let eg = a.eg.clone().nil();
    let mut g = group::filter(d, &eg, &|d, cc| thfilter(d, cc, a.tp, zone), NO_EX);
    // if c:IsRelateToEffect(e) and c:IsAbleToHand() and #g>0 then
    if card::is_relate_to_effect(d, c, a.e) && card::is_able_to_hand(d, c, None) && !g.is_empty() {
        // g:AddCard(c)
        group::add_card(&mut g, &CardSet::from_card(c));
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
