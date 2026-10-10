//! Gokipole
//! ゴキポール
use super::prelude::*;

pub const ID: u32 = 17535764;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 1 Level 4 Insect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(4) and c:IsRace(RACE_INSECT) and c:IsAbleToHand()
    card::is_level(d, c, &[4]) && card::is_race(d, c, RACE_INSECT) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
    // if #g==0 or Duel.SendtoHand(g,nil,REASON_EFFECT)==0 then return end
    let g = match g {
        Some(g) if !g.is_empty() => g,
        _ => return,
    };
    if duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) == 0 {
        return;
    }
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - tp, tc);
    // if tc:IsType(TYPE_NORMAL) and tc:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if card::is_type(d, tc, TYPE_NORMAL)
        && card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)==0 then return end
        if duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
            return;
        }
        // local dg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttackAbove,tc:GetAttack()),tp,LOCATION_MZONE,LOCATION_MZONE,nil)
        let atk = card::get_attack(d, tc);
        let dg = duel::get_matching_group(d, Some(&aux::faceup_filter(move |d, c| card::is_attack_above(d, c, atk))), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // if #dg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !dg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local dc=dg:Select(tp,1,1,nil)
            let dc = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
            // if #dc>0 then
            if !dc.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.HintSelection(dc,true)
                duel::hint_selection(d, dc.clone(), true);
                // Duel.Destroy(dc,REASON_EFFECT)
                duel::destroy(d, dc, REASON_EFFECT, None, None);
            }
        }
    }
}
