//! Jinzo - Jector
use super::prelude::*;

pub const ID: u32 = 51916032;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_JINZO], listed_names: &[ID, CARD_JINZO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // change name
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetValue(CARD_JINZO)
    effect::set_value(d, e1, CARD_JINZO as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // add to hand
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(activate)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_JINZO) and not c:IsCode(id) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_JINZO as u16]) && !card::is_code(d, c, &[ID]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.cffilter(c)
fn cffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:GetSequence()<5
    card::is_facedown(d, c) && card::get_sequence(d, c) < 5
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_JINZO) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_JINZO as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
    // if #g==0 or Duel.SendtoHand(g,nil,REASON_EFFECT)==0 then return end
    let Some(g) = g else {
        return;
    };
    if g.is_empty() || duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) == 0 {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // local cg=Duel.GetMatchingGroup(s.cffilter,tp,0,LOCATION_SZONE,nil)
    let cg = duel::get_matching_group(d, Some(&cffilter), a.tp, 0, LOCATION_SZONE, NO_EX);
    // if #cg>0 then
    if !cg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.ConfirmCards(tp,cg)
        duel::confirm_cards(d, a.tp, cg.clone());
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        // if ft<=0 then return end
        if ft <= 0 {
            return;
        }
        // local ct=cg:FilterCount(Card.IsTrap,nil)
        let mut ct = group::filter_count(d, &cg, &|d, c| card::is_trap(d, c), NO_EX) as i32;
        // if ct>ft then ct=ft end
        if ct > ft {
            ct = ft;
        }
        // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
        let e = a.e;
        let tp = a.tp;
        let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), a.tp, LOCATION_HAND, 0, NO_EX);
        // if ct>0 and #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if ct > 0 && !sg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // local tg=sg:Select(tp,1,ct,nil)
            let tg = group::select(d, &sg, a.tp, 1, ct as u16, false, NO_EX).nil();
            // Duel.SpecialSummon(tg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
