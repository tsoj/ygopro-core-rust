//! Two-for-One Team
use super::prelude::*;

pub const ID: u32 = 89928517;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsPublic()
    !card::is_public(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_HAND,0,nil)*Duel.GetMatchingGroupCount(s.filter,tp,0,LOCATION_HAND,nil)>0 end
    if a.chk == Some(0) {
        return duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_HAND, 0, NO_EX)
            * duel::get_matching_group_count(d, Some(&filter), a.tp, 0, LOCATION_HAND, NO_EX)
            > 0;
    }
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local hg1=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND,0,nil)
    let hg1 = duel::get_matching_group(d, Some(&filter), tp, LOCATION_HAND, 0, NO_EX);
    // local hg2=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_HAND,nil)
    let hg2 = duel::get_matching_group(d, Some(&filter), tp, 0, LOCATION_HAND, NO_EX);
    // if #hg1==0 or #hg2==0 then return end
    if hg1.is_empty() || hg2.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local tc1=hg1:Select(tp,1,1,nil):GetFirst()
    let sel1 = group::select(d, &hg1, tp, 1, 1, false, NO_EX).nil();
    let tc1 = group::get_first(d, &sel1).nil();
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_CONFIRM as u64);
    // local tc2=hg2:Select(1-tp,1,1,nil):GetFirst()
    let sel2 = group::select(d, &hg2, 1 - tp, 1, 1, false, NO_EX).nil();
    let tc2 = group::get_first(d, &sel2).nil();
    // Duel.ConfirmCards(1-tp,tc1)
    duel::confirm_cards(d, 1 - tp, tc1);
    // Duel.ConfirmCards(tp,tc2)
    duel::confirm_cards(d, tp, tc2);
    // if tc1:IsMonster() and tc2:IsMonster() then
    if card::is_monster(d, tc1) && card::is_monster(d, tc2) {
        // local ask1=Duel.SelectYesNo(tp,aux.Stringid(id,1))
        let ask1 = duel::select_yes_no(d, tp, aux::stringid(ID, 1));
        // local ask2=Duel.SelectYesNo(1-tp,aux.Stringid(id,1))
        let ask2 = duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 1));
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and tc1:IsCanBeSpecialSummoned(e,0,tp,false,false) and ask1 then
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, tc1, e, 0, tp, false, false, None, None, None)
            && ask1
        {
            // Duel.SpecialSummonStep(tc1,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, tc1, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
        // if Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and tc2:IsCanBeSpecialSummoned(e,0,1-tp,false,false) and ask2 then
        if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, tc2, e, 0, 1 - tp, false, false, None, None, None)
            && ask2
        {
            // Duel.SpecialSummonStep(tc2,0,1-tp,1-tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, tc2, 0, 1 - tp, 1 - tp, false, false, POS_FACEUP as u8, None);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    } else if card::is_spell(d, tc1)
        && card::is_spell(d, tc2)
        && duel::is_player_can_draw(d, tp, 2)
        && duel::is_player_can_draw(d, 1 - tp, 2)
    {
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, tp, 2, REASON_EFFECT);
        // Duel.Draw(1-tp,2,REASON_EFFECT)
        duel::draw(d, 1 - tp, 2, REASON_EFFECT);
    } else if card::is_trap(d, tc1)
        && card::is_trap(d, tc2)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_DECK, 0, 2, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_DECK, 2, NO_EX)
    {
        // for p=0,1 do
        for p in 0u8..=1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
            // local g=Duel.SelectMatchingCard(p,Card.IsAbleToGrave,p,LOCATION_DECK,0,2,2,nil)
            let g = duel::select_matching_card(d, p, Some(&|d, c| card::is_able_to_grave(d, c, None)), p, LOCATION_DECK, 0, 2, 2, false, NO_EX).nil();
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, &g, REASON_EFFECT, None, None);
        }
    }
}
