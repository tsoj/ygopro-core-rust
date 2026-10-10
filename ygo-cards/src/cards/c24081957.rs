//! Sinful Spoils of Subversion - Snake-Eye
use super::prelude::*;

pub const ID: u32 = 24081957;

pub fn initial_effect(d: &mut Duel, c: CardId) {
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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,tp,hand_chk)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8, hand_chk: bool) -> bool {
    // if c:IsFacedown() then return false end
    if card::is_facedown(d, c) {
        return false;
    }
    // local owner=c:GetOwner()
    let owner = card::get_owner(d, c);
    // local ft=Duel.GetLocationCount(owner,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, owner, LOCATION_SZONE as u8, None, None, None).0;
    // if owner==tp and hand_chk then ft=ft-1 end
    if owner == tp && hand_chk {
        ft -= 1;
    }
    // return ft>0
    ft > 0
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local hand_chk=e:GetHandler():IsLocation(LOCATION_HAND)
    let h = effect::get_handler(d, a.e).nil();
    let hand_chk = card::is_location(d, h, LOCATION_HAND);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,tp,hand_chk) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, a.tp, hand_chk);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,tp,hand_chk) end
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, tp, hand_chk)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,tp,hand_chk)
    duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, tp, hand_chk)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e)) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e)) {
        return;
    }
    // if Duel.GetLocationCount(tc:GetOwner(),LOCATION_SZONE)==0 then
    let owner = card::get_owner(d, tc);
    if duel::get_location_count(d, owner, LOCATION_SZONE as u8, None, None, None).0 == 0 {
        // Duel.SendtoGrave(tc,REASON_RULE,nil,PLAYER_NONE)
        duel::send_to_grave(d, CardSet::from_card(tc), REASON_RULE, None, Some(PLAYER_NONE as u8));
    // elseif Duel.MoveToField(tc,tp,tc:GetOwner(),LOCATION_SZONE,POS_FACEUP,tc:IsMonsterCard()) then
    } else {
        let is_monster = card::is_monster(d, tc);
        if duel::move_to_field(d, tc, a.tp, owner, LOCATION_SZONE as u16, POS_FACEUP as u8, is_monster, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
            // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
