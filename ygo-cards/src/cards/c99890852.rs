//! Trickstar Bouquet
use super::prelude::*;

pub const ID: u32 = 99890852;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRICKSTAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP as u32, None);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_TRICKSTAR) and not c:IsType(TYPE_FUSION|TYPE_SYNCHRO|TYPE_TOKEN|TYPE_XYZ|TYPE_LINK)
    //     and c:IsAbleToHand() and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,c)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_TRICKSTAR as u16])
        && !card::is_type(d, c, TYPE_FUSION | TYPE_SYNCHRO | TYPE_TOKEN | TYPE_XYZ | TYPE_LINK)
        && card::is_able_to_hand(d, c, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    effect::set_label_object(d, a.e, Value::card(first));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,g)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, (&g).into());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    // if #g~=2 then return end
    let g = duel::get_target_cards(d, a.e).nil();
    if g.len() != 2 {
        return;
    }
    // local hc=g:GetFirst()
    // local tc=g:GetNext()
    let members = group::iter(d, &g);
    let (mut hc, mut tc) = (members[0], members[1]);
    // if hc~=e:GetLabelObject() then tc,hc=hc,tc end
    if Some(hc) != effect::get_label_object(d, a.e).as_card() {
        std::mem::swap(&mut hc, &mut tc);
    }
    // if hc:IsFaceup() and tc:IsFaceup() and Duel.SendtoHand(hc,nil,REASON_EFFECT)~=0 then
    if card::is_faceup(d, hc) && card::is_faceup(d, tc) && duel::send_to_hand(d, hc, None, REASON_EFFECT, None) != 0 {
        // local sg=Duel.GetOperatedGroup()
        let sg = duel::get_operated_group(d);
        // if not sg:IsExists(Card.IsLocation,1,nil,LOCATION_HAND) then return end
        if !group::is_exists(d, &sg, &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX) {
            return;
        }
        // local atk=hc:GetBaseAttack()
        let atk = card::get_base_attack(d, hc);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
