//! Libromancer First Appearance
//! リブロマンサー・ライジング
use super::prelude::*;

pub const ID: u32 = 18890039;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIBROMANCER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
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
    // Ritual Summon 1 "Libromancer" monster
    // local e2=Ritual.CreateProc(c,RITPROC_GREATER,aux.FilterBoolFunction(Card.IsSetCard,SET_LIBROMANCER),nil,aux.Stringid(id,1))
    let e2 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            filter: Some(std::sync::Arc::new(|d, cc| card::is_set_card(d, cc, &[SET_LIBROMANCER as u16]))),
            desc: Some(aux::stringid(ID, 1)),
            ..Default::default()
        },
    );
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_LIBROMANCER) and c:IsAbleToHand()
    //     and not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,c:GetCode()),tp,LOCATION_ONFIELD,0,1,nil)
    if !(card::is_monster(d, c) && card::is_set_card(d, c, &[SET_LIBROMANCER as u16]) && card::is_able_to_hand(d, c, None)) {
        return false;
    }
    let codes = card::get_code(d, c);
    !duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(move |d, cc| card::is_code(d, cc, &codes))),
        tp,
        LOCATION_ONFIELD,
        0,
        1,
        NO_EX,
    )
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - tp, sg);
        }
    }
}
