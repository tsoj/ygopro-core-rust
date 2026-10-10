//! Subterror Guru
use super::prelude::*;

pub const ID: u32 = 16428514;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SUBTERROR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.setcon1)
    effect::set_condition(d, e2, Some(condition(setcon1)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e3:SetCondition(s.setcon2)
    effect::set_condition(d, e3, Some(condition(setcon2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SUBTERROR) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SUBTERROR as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_SUBTERROR)`
fn faceup_subterror(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SUBTERROR as u16])
}
/// `s.setcon1(e,tp,eg,ep,ev,re,r,rp)`
fn setcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_SUBTERROR),tp,LOCATION_ONFIELD,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    !duel::is_existing_matching_card(d, Some(&|d, c| faceup_subterror(d, c)), a.tp, LOCATION_ONFIELD, 0, 1, h.into())
}
/// `s.setcon2(e,tp,eg,ep,ev,re,r,rp)`
fn setcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_SUBTERROR),tp,LOCATION_ONFIELD,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&|d, c| faceup_subterror(d, c)), a.tp, LOCATION_ONFIELD, 0, 1, h.into())
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCanTurnSet()
    card::is_faceup(d, c) && card::is_can_turn_set(d, c)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc~=c and s.setfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && chkc != c && setfilter(d, chkc);
    }
    // if chk==0 then return s.setfilter(c)
    //     and Duel.IsExistingTarget(s.setfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c) end
    if a.chk == Some(0) {
        return setfilter(d, c)
            && duel::is_existing_target(d, Some(&|d, x| setfilter(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectTarget(tp,s.setfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c)
    let mut g = duel::select_target(d, a.tp, Some(&|d, x| setfilter(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into()).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,2,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), 2, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local g=Group.FromCards(c,tc)
    let mut g = CardSet::from_card(c);
    group::add_card(&mut g, &CardSet::from_card(tc));
    // if g:IsExists(function(c,e) return not c:IsFaceup() or not c:IsRelateToEffect(e) end,1,nil,e) then return end
    if group::is_exists(d, &g, &|d, x| !card::is_faceup(d, x) || !card::is_relate_to_effect(d, x, a.e), 1, NO_EX) {
        return;
    }
    // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
    duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
}
