//! Shateki
use super::prelude::*;

pub const ID: u32 = 5574510;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Roll a six-sided die and apply an effect based on the result
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DICE+CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_DICE + CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(function(e,tp) return not Duel.HasFlagEffect(tp,id) end)
    effect::set_condition(d, e1, Some(condition(|d, a| !duel::has_flag_effect(d, a.tp, ID, None))));
    // e1:SetCost(Cost.Discard())
    effect::set_cost(d, e1, Some(cost::discard(None, false, None, None, None)));
    // e1:SetTarget(s.dietg)
    effect::set_target(d, e1, Some(cost_target(dietg)));
    // e1:SetOperation(s.dieop)
    effect::set_operation(d, e1, Some(operation(dieop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.dietg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn dietg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() and chkc:HasLevel() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - a.tp)
            && card::is_faceup(d, chkc)
            && card::has_level(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.HasLevel),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::has_level(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.HasLevel),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c) && card::has_level(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c,lv)`
fn thfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsOriginalLevel(lv) and c:IsAbleToHand()
    card::is_original_level(d, c, lv) && card::is_able_to_hand(d, c, None)
}
/// `s.dieop(e,tp,eg,ep,ev,re,r,rp)`
fn dieop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:HasLevel()) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::has_level(d, tc)) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local dc=Duel.TossDice(tp,1)
    let dc = duel::toss_dice(d, a.tp, 1, 0)[0] as i64;
    // if dc>tc:GetLevel() then
    if dc > card::get_level(d, tc) as i64 {
        // Destroy it, then you can add 1 monster with the same original Level from your Deck to your hand
        // if Duel.Destroy(tc,REASON_EFFECT)>0
        if duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
            // and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,tc:GetOriginalLevel())
            let lv = card::get_original_level(d, tc);
            if duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, lv)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
                // and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
                && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
            {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
                // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tc:GetOriginalLevel())
                let lv = card::get_original_level(d, tc);
                let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, lv)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
                // if #g>0 then
                if !g.is_empty() {
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.SendtoHand(g,nil,REASON_EFFECT)
                    duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
                    // Duel.ConfirmCards(1-tp,g)
                    duel::confirm_cards(d, 1 - a.tp, g);
                }
            }
        }
        // Cannot activate this effect of "Shateki" for the rest of this turn
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,2))
        aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    } else {
        // Reduce its Level by 1
        // if tc:IsLevelBelow(1) then return end
        if card::is_level_below(d, tc, 1) {
            return;
        }
        // tc:UpdateLevel(-1,RESET_EVENT|RESETS_STANDARD,c)
        card::update_level(d, tc, -1, Some(RESET_EVENT | RESETS_STANDARD), Some(c));
    }
}
