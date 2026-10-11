//! Springans Watch
use super::prelude::*;

pub const ID: u32 = 23499963;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPRINGANS], listed_names: &[60884672], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Vast Desert Gold Golgonda" from deck
    // Or add 1 "Sprigans" monster from deck, and if you do, send 1 "Sprigans" monster from deck to GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
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
// function s.fcfilter(c)
fn fcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(60884672) and c:IsAbleToHand()
    card::is_code(d, c, &[60884672]) && card::is_able_to_hand(d, c, None)
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_SPRINGANS) and c:IsAbleToGrave()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_SPRINGANS as u16]) && card::is_able_to_grave(d, c, None)
}
// function s.mcfilter(c,tp)
fn mcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_SPRINGANS) and c:IsAbleToHand()
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,c)
    card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_SPRINGANS as u16])
        && card::is_able_to_hand(d, c, None)
        && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, c.into())
}
// aux.FaceupFilter(Card.IsCode,60884672)
fn faceup_golgonda(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[60884672])
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.fcfilter,tp,LOCATION_DECK,0,1,nil)
    //     or (Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,60884672),tp,LOCATION_FZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.mcfilter,tp,LOCATION_DECK,0,1,nil,tp))
    // end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&fcfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            || (duel::is_existing_matching_card(d, Some(&faceup_golgonda), a.tp, LOCATION_FZONE, 0, 1, NO_EX)
                && duel::is_existing_matching_card(d, Some(&|d, c| mcfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.IsExistingMatchingCard(s.fcfilter,tp,LOCATION_DECK,0,1,nil)
    let fa = duel::is_existing_matching_card(d, Some(&fcfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,60884672),tp,LOCATION_FZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.mcfilter,tp,LOCATION_DECK,0,1,nil,tp)
    let fb = duel::is_existing_matching_card(d, Some(&faceup_golgonda), a.tp, LOCATION_FZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| mcfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // if not a and not b then return end
    if !fa && !fb {
        return;
    }
    // if (not a and b) or (b and Duel.SelectYesNo(tp,aux.Stringid(id,1))) then
    if (!fa && fb) || (fb && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.mcfilter,tp,LOCATION_DECK,0,1,1,nil,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| mcfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
        if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, &g);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
            // local cg=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
            let cg = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #cg>0 then Duel.SendtoGrave(cg,REASON_EFFECT) end
            if !cg.is_empty() {
                duel::send_to_grave(d, cg, REASON_EFFECT, None, None);
            }
        }
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.fcfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&fcfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, &g);
        }
    }
}
