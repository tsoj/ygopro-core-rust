//! Materiactor Critical
use super::prelude::*;

pub const ID: u32 = 64767757;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MATERIACTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 card attached to 1 "Materiactor" Xyz Monster you control to your hand and negate the activation
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_NEGATE+CATEGORY_TODECK+CATEGORY_ATKCHANGE+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_NEGATE + CATEGORY_TODECK + CATEGORY_ATKCHANGE + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.thnegcon)
    effect::set_condition(d, e1, Some(condition(thnegcon)));
    // e1:SetTarget(s.thnegtg)
    effect::set_target(d, e1, Some(cost_target(thnegtg)));
    // e1:SetOperation(s.thnegop)
    effect::set_operation(d, e1, Some(operation(thnegop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thnegcon(e,tp,eg,ep,ev,re,r,rp)`
fn thnegcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE)) and Duel.IsChainNegatable(ev)
    let re = a.re.nil();
    (effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.materiactorxyzfilter(c)`
fn materiactorxyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MATERIACTOR) and c:IsType(TYPE_XYZ) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_MATERIACTOR as u16]) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c)
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsOwner(tp) and c:IsAbleToHand()
    card::is_owner(d, c, tp) && card::is_able_to_hand(d, c, None)
}
/// `s.xyzthfilter(c,tp)`
fn xyzthfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return s.materiactorxyzfilter(c) and c:GetOverlayGroup():IsExists(s.thfilter,1,nil,tp)
    if !materiactorxyzfilter(d, c) {
        return false;
    }
    let og = card::get_overlay_group(d, c);
    group::is_exists(d, &og, &|d, cc| thfilter(d, cc, tp), 1, NO_EX)
}
/// `s.thnegtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thnegtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.xyzthfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, c| xyzthfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_OVERLAY)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_OVERLAY as i32);
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.thnegop(e,tp,eg,ep,ev,re,r,rp)`
fn thnegop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local xyzg=Duel.GetMatchingGroup(s.xyzthfilter,tp,LOCATION_MZONE,0,nil,tp)
    let xyzg = duel::get_matching_group(d, Some(&|d, c| xyzthfilter(d, c, tp)), tp, LOCATION_MZONE, 0, NO_EX);
    // if #xyzg==0 then return end
    if xyzg.is_empty() {
        return;
    }
    // local og=Duel.GetOverlayGroup(tp,1,0,xyzg)
    let og = duel::get_overlay_group(d, tp, 1, 0, Some(&xyzg));
    // if #og==0 then return end
    if og.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=og:FilterSelect(tp,s.thfilter,1,1,nil,tp):GetFirst()
    let sg = group::filter_select(d, &og, tp, &|d, c| thfilter(d, c, tp), 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sg);
    // if sc and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) then
    let Some(sc) = sc else { return };
    if duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) {
        // Duel.ConfirmCards(1-tp,sc)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // if not Duel.NegateActivation(ev) then return end
        if !duel::negate_activation(d, a.ev as u8) {
            return;
        }
        // if sc:IsMonster() then
        if card::is_monster(d, sc) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
            // local dg=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,1,nil)
            let dg = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_deck(d, c)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            // if #dg>0 then
            if !dg.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoDeck(dg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
                duel::send_to_deck(d, dg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
            }
        } else if card::is_spell(d, sc) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local atkg=Duel.GetMatchingGroup(s.materiactorxyzfilter,tp,LOCATION_MZONE,0,nil)
            let atkg = duel::get_matching_group(d, Some(&|d, c| materiactorxyzfilter(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
            // for tc in atkg:Iter() do tc:UpdateAttack(1000) end
            for tc in group::iter(d, &atkg) {
                card::update_attack(d, tc, 1000, None, None, None);
            }
        } else if card::is_trap(d, sc) && card::is_s_setable(d, sc, false, None) && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SSet(tp,sc)
            duel::s_set(d, tp, CardSet::from_card(sc), None, true);
        }
    }
}
