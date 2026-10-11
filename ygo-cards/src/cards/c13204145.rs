//! Mimighoul Maker
use super::prelude::*;

pub const ID: u32 = 13204145;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MIMIGHOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reveal 2 Flip monsters to Special Summon or add to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.revtg)
    effect::set_target(d, e1, Some(cost_target(revtg)));
    // e1:SetOperation(s.revop)
    effect::set_operation(d, e1, Some(operation(revop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change 1 face-down monster to face-up Attack or Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp,eg) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end)
    effect::set_condition(d, e2, Some(condition(summoned_by_opp_cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.postg)
    effect::set_target(d, e2, Some(cost_target(postg)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end`
fn summoned_by_opp_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX)
}
/// `s.revfilter(c,e,tp)`
fn revfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_FLIP) and not c:IsPublic() and c:IsAbleToHand()
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE,1-tp)
    card::is_type(d, c, TYPE_FLIP)
        && !card::is_public(d, c)
        && card::is_able_to_hand(d, c, None)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), Some(1 - tp), None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MIMIGHOUL) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MIMIGHOUL as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.revtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn revtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // if Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)==0 then return false end
        if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 == 0 {
            return false;
        }
        // local g=Duel.GetMatchingGroup(s.revfilter,tp,LOCATION_DECK,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| revfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,0)
        return matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&aux::dncheck()), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.revop(e,tp,eg,ep,ev,re,r,rp)`
fn revop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)==0 then return end
    if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.revfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| revfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_SELECT)
    let mut sg = aux::select_unselect_group(
        d,
        &g,
        Some(e),
        tp,
        Some(2),
        Some(2),
        Some(&aux::dncheck()),
        1,
        tp,
        Some(HINTMSG_SELECT as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #sg~=2 then return end
    if sg.len() != 2 {
        return;
    }
    // Duel.ConfirmCards(1-tp,sg)
    duel::confirm_cards(d, 1 - tp, sg.clone());
    // local sc=sg:RandomSelect(1-tp,1):GetFirst()
    let picked = group::random_select(d, &sg, 1 - tp, 1);
    let sc = group::get_first(d, &picked);
    // if not sc or Duel.SpecialSummon(sc,0,tp,1-tp,false,false,POS_FACEDOWN_DEFENSE)==0 then return end
    let Some(sc) = sc else {
        return;
    };
    if duel::special_summon(d, sc, 0, tp, 1 - tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) == 0 {
        return;
    }
    // Duel.ConfirmCards(tp,sc)
    duel::confirm_cards(d, tp, sc);
    // sg:RemoveCard(sc)
    group::remove_card(&mut sg, &CardSet::from_card(sc));
    // if Duel.SendtoHand(sg,nil,REASON_EFFECT)==0 then return end
    if duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None) == 0 {
        return;
    }
    // Duel.ConfirmCards(1-tp,sg)
    duel::confirm_cards(d, 1 - tp, sg);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local hg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let hg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, NO_EX);
    // if #hg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
    if hg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local hsg=hg:Select(tp,1,1,nil)
    let hsg = group::select(d, &hg, tp, 1, 1, false, NO_EX).nil();
    // if #hsg>0 then
    if !hsg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(hsg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, hsg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.posfilter`: `aux.AND(Card.IsFacedown,Card.IsCanChangePosition)`
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) && card::is_can_change_position(d, c)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFacedown() and chkc:IsCanChangePosition() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && posfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.AND(Card.IsFacedown,Card.IsCanChangePosition),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&posfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectTarget(tp,aux.AND(Card.IsFacedown,Card.IsCanChangePosition),tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&posfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), 1, tp, 0);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn posop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // local pos=(POS_FACEUP_ATTACK|POS_FACEUP_DEFENSE)&~tc:GetPosition()
        let pos = (POS_FACEUP_ATTACK | POS_FACEUP_DEFENSE) & !card::get_position(d, tc);
        // pos=Duel.SelectPosition(tp,tc,pos)
        let pos = duel::select_position(d, tp, tc, pos as u8);
        // Duel.ChangePosition(tc,pos)
        duel::change_position(d, tc, pos as u8, None, None, None, false);
    }
}
