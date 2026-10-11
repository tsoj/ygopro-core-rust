//! Morphtronic Remoten
use super::prelude::*;

pub const ID: u32 = 57108202;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MORPHTRONIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.cona)
    effect::set_condition(d, e1, Some(condition(cona)));
    // e1:SetTarget(s.tga)
    effect::set_target(d, e1, Some(cost_target(tga)));
    // e1:SetOperation(s.opa)
    effect::set_operation(d, e1, Some(operation(opa)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // salvage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.cond)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.tgd)
    effect::set_target(d, e2, Some(cost_target(tgd)));
    // e2:SetOperation(s.opd)
    effect::set_operation(d, e2, Some(operation(opd)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,lv)`
fn filter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsSetCard(SET_MORPHTRONIC) and c:GetLevel()==lv and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && card::get_level(d, c) == lv && card::is_able_to_hand(d, c, None)
}
/// `s.cona(e,tp,eg,ep,ev,re,r,rp)`
fn cona(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsDisabled() and e:GetHandler():IsAttackPos()
    let h = effect::get_handler(d, a.e).nil();
    !card::is_disabled(d, h) && card::is_attack_pos(d, h)
}
/// `s.filtera(c,tp)`
fn filtera(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return c:IsSetCard(SET_MORPHTRONIC) and lv>0 and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,lv)
    card::is_set_card(d, c, &[SET_MORPHTRONIC as u16])
        && lv > 0
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.tga(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tga(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.filtera(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filtera(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filtera,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filtera(d, c, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.filtera,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filtera(d, c, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.opa(e,tp,eg,ep,ev,re,r,rp)`
fn opa(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)~=0 then
    let Some(tc) = tc else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, a.e) || duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,tc:GetLevel())
    let lv = card::get_level(d, tc);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter(d, x, lv)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.cond(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsDisabled() and e:GetHandler():IsDefensePos()
    let h = effect::get_handler(d, a.e).nil();
    !card::is_disabled(d, h) && card::is_defense_pos(d, h)
}
/// `s.filterd(c,tp)`
fn filterd(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return c:IsSetCard(SET_MORPHTRONIC) and lv>0 and c:IsAbleToGrave()
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil,lv)
    card::is_set_card(d, c, &[SET_MORPHTRONIC as u16])
        && lv > 0
        && card::is_able_to_grave(d, c, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, lv)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.tgd(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgd(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filterd,tp,LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filterd(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.opd(e,tp,eg,ep,ev,re,r,rp)`
fn opd(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filterd,tp,LOCATION_HAND,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filterd(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if not tc then return end
    let Some(tc) = tc else {
        return;
    };
    // Duel.SendtoGrave(tc,REASON_EFFECT)
    duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    // if not tc:IsLocation(LOCATION_GRAVE) then return end
    if !card::is_location(d, tc, LOCATION_GRAVE) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,tc,tc:GetLevel())
    let lv = card::get_level(d, tc);
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, x| filter(d, x, lv)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, tc.into()).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
