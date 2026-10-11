//! Onomatopickup
//! オノマト選択
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 85119159;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ONOMAT, SET_GAGAGA, SET_GOGOGO, SET_DODODO, SET_ZUBABA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // change level
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ONOMAT) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ONOMAT as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if group::get_count(&g) > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
// function s.lvfilter1(c,tp)
fn lvfilter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:HasLevel() and (c:IsSetCard(SET_GAGAGA) or c:IsSetCard(SET_GOGOGO) or c:IsSetCard(SET_DODODO) or c:IsSetCard(SET_ZUBABA))
    //   and Duel.IsExistingMatchingCard(s.lvfilter2,tp,LOCATION_MZONE,0,1,c,c:GetLevel())
    let set_ok = card::is_set_card(d, c, &[SET_GAGAGA as u16])
        || card::is_set_card(d, c, &[SET_GOGOGO as u16])
        || card::is_set_card(d, c, &[SET_DODODO as u16])
        || card::is_set_card(d, c, &[SET_ZUBABA as u16]);
    if !(card::is_faceup(d, c) && card::has_level(d, c) && set_ok) {
        return false;
    }
    let lv = card::get_level(d, c) as u32;
    duel::is_existing_matching_card(d, Some(&|d: &mut Duel, cc| lvfilter2(d, cc, lv)), tp, LOCATION_MZONE, 0, 1, c.into())
}
// function s.lvfilter2(c,lv)
fn lvfilter2(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsFaceup() and c:HasLevel() and not c:IsLevel(lv)
    card::is_faceup(d, c) && card::has_level(d, c) && !card::is_level(d, c, &[lv])
}
// function s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.lvfilter1(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && lvfilter1(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.lvfilter1,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d: &mut Duel, c| lvfilter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.lvfilter1,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, tp, Some(&|d: &mut Duel, c| lvfilter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.lvop(e,tp,eg,ep,ev,re,r,rp)
fn lvop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) {
        // local lv=tc:GetLevel()
        let lv = card::get_level(d, tc);
        // local g=Duel.GetMatchingGroup(s.lvfilter2,tp,LOCATION_MZONE,0,tc,lv)
        let g = duel::get_matching_group(d, Some(&|d: &mut Duel, c| lvfilter2(d, c, lv)), tp, LOCATION_MZONE, 0, tc.into());
        // local lc=g:GetFirst()
        let _ = group::get_first(d, &g);
        // for lc in aux.Next(g) do
        for lc in group::iter(d, &g) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h2 = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h2);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            // e1:SetValue(lv)
            effect::set_value(d, e1, lv as i64);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // lc:RegisterEffect(e1)
            card::register_effect(d, lc, e1, false);
        }
    }
}
