//! Dinowrestler Capaptera
use super::prelude::*;

pub const ID: u32 = 93507434;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOWRESTLER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.tgcon)
    effect::set_condition(d, e1, Some(condition(tgcon)));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCondition(s.ddcon)
    effect::set_condition(d, e2, Some(condition(ddcon)));
    // e2:SetTarget(s.ddtg)
    effect::set_target(d, e2, Some(cost_target(ddtg)));
    // e2:SetOperation(s.ddop)
    effect::set_operation(d, e2, Some(operation(ddop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tgcon(e,tp,...)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
    let opp = duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE);
    let own = duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0);
    opp > own
}
/// `s.tgtg(e,tp,...,chk,chkc)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsAbleToGrave() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_able_to_grave(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToGrave,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToGrave,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, 0, 0);
    true
}
/// `s.tgop(e,tp,...)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoGrave(tc,REASON_EFFECT)
        duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    }
}
/// `s.ddcon(e,tp,...)`
fn ddcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c);
    // return c:IsLocation(LOCATION_GRAVE) and r & REASON_LINK == REASON_LINK
    //     and rc:IsSetCard(SET_DINOWRESTLER) and rc:IsLinkMonster()
    card::is_location(d, c, LOCATION_GRAVE)
        && (a.r & REASON_LINK) == REASON_LINK
        && card::is_set_card(d, rc.nil(), &[SET_DINOWRESTLER as u16])
        && card::is_link_monster(d, rc.nil())
}
/// `s.ddtg(e,tp,...,chk)`
fn ddtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetCard(e:GetHandler():GetReasonCard())
    let c = effect::get_handler(d, a.e).nil();
    let rc = card::get_reason_card(d, c).nil();
    duel::set_target_card(d, rc);
    true
}
/// `s.ddop(e,tp,...)`
fn ddop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local sync=c:GetReasonCard()
    let sync = card::get_reason_card(d, c).nil();
    // if sync:IsRelateToEffect(e) and sync:IsFaceup() and sync:IsSetCard(SET_DINOWRESTLER) and sync:IsLinkMonster() then
    if card::is_relate_to_effect(d, sync, a.e)
        && card::is_faceup(d, sync)
        && card::is_set_card(d, sync, &[SET_DINOWRESTLER as u16])
        && card::is_link_monster(d, sync)
    {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // sync:RegisterEffect(e1)
        card::register_effect(d, sync, e1, false);
    }
}
