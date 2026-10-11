//! Time Dimension Hole
//! タイム・ディメンションホール
use super::prelude::*;

pub const ID: u32 = 90615639;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=2 end)
    effect::set_condition(d, e1, Some(lua_fn(cond1)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=2 end`
fn cond1(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    duel::get_field_group(d, tp, LOCATION_DECK, 0).len() >= 2
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsPlayerCanSpecialSummon(tp)
    //     and Duel.IsExistingMatchingCard(Card.IsSummonableCard,tp,LOCATION_DECK,0,1,nil)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_EHERO_BLAZEMAN) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon(d, tp, None)
            && duel::is_existing_matching_card(d, Some(&card::is_summonable_card), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_player_affected_by_effect(d, tp, CARD_EHERO_BLAZEMAN).is_none();
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<2 then return end
    if duel::get_field_group(d, tp, LOCATION_DECK, 0).len() < 2 {
        return;
    }
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, tp, 1);
    // local tc=Duel.GetDecktopGroup(tp,1):GetFirst()
    let tc = group::get_first(d, &duel::get_decktop_group(d, tp, 1)).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and tc:IsSummonableCard()
    //     and tc:IsCanBeSpecialSummoned(e,0,tp,false,false) then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_summonable_card(d, tc)
        && card::is_can_be_special_summoned(d, tc, a.e, 0, tp, false, false, None, None, None)
    {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    } else {
        // local ac=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
        let ac = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
        // if ac==1 then Duel.MoveSequence(tc,1) end
        if ac == 1 {
            duel::move_sequence(d, tc, 1, None);
        }
    }
}
