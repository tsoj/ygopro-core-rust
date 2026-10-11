//! Goddess Verdande's Guidance
use super::prelude::*;

pub const ID: u32 = 64961254;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[91969909], listed_series: &[SET_VALKYRIE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: ... you can add 1 "Goddess Urd's Verdict" from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
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
    // Reveal the top card of your opponent's Deck ...
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.revealtg)
    effect::set_target(d, e2, Some(cost_target(revealtg)));
    // e2:SetOperation(s.revealop)
    effect::set_operation(d, e2, Some(operation(revealop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(91969909) and c:IsAbleToHand()
    card::is_code(d, c, &[91969909]) && card::is_able_to_hand(d, c, None)
}
// aux.NOT(aux.FaceupFilter(Card.IsSetCard,SET_VALKYRIE))
fn not_faceup_valkyrie(d: &mut Duel, c: CardId) -> bool {
    !(card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VALKYRIE as u16]))
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)>0
    //   and not Duel.IsExistingMatchingCard(aux.NOT(aux.FaceupFilter(Card.IsSetCard,SET_VALKYRIE)),tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty()
        && duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) > 0
        && !duel::is_existing_matching_card(d, Some(&not_faceup_valkyrie), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
// function s.revealtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn revealtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, 1 - a.tp, 1);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CARDTYPE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CARDTYPE as u64);
    // Duel.SetTargetParam(Duel.SelectOption(tp,DECLTYPE_MONSTER,DECLTYPE_SPELL,DECLTYPE_TRAP))
    let opt = duel::select_option(d, a.tp, true, &[DECLTYPE_MONSTER as u64, DECLTYPE_SPELL as u64, DECLTYPE_TRAP as u64]);
    duel::set_target_param(d, opt as u32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.revealop(e,tp,eg,ep,ev,re,r,rp)
fn revealop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(1-tp,0,LOCATION_DECK)<=0 then return end
    if duel::get_field_group_count(d, 1 - a.tp, 0, LOCATION_DECK) <= 0 {
        return;
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.ConfirmDecktop(1-tp,1)
    duel::confirm_decktop(d, 1 - a.tp, 1);
    // local tc=Duel.GetDecktopGroup(1-tp,1):GetFirst()
    let top = duel::get_decktop_group(d, 1 - a.tp, 1);
    let tc = group::get_first(d, &top);
    // if not tc then return end
    let Some(tc) = tc else {
        return;
    };
    // local opt=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let opt = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if (opt==0 and tc:IsMonster() and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //   and tc:IsCanBeSpecialSummoned(e,0,1-tp,false,false,POS_FACEDOWN_DEFENSE,1-tp)) then
    let summon_ok = opt == 0 && card::is_monster(d, tc) && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(
        d,
        tc,
        a.e,
        0,
        1 - a.tp,
        false,
        false,
        Some(POS_FACEDOWN_DEFENSE as u8),
        Some(1 - a.tp),
        None,
    );
    if summon_ok {
        // Duel.SpecialSummon(tc,0,1-tp,1-tp,false,false,POS_FACEDOWN_DEFENSE)
        duel::special_summon(d, tc, 0, 1 - a.tp, 1 - a.tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
        // Duel.ConfirmCards(tp,tc)
        duel::confirm_cards(d, a.tp, tc);
    } else if ((opt == 1 && card::is_spell(d, tc)) || (opt == 2 && card::is_trap(d, tc))) && card::is_s_setable(d, tc, false, Some(1 - a.tp)) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SSet(1-tp,tc)
        duel::s_set(d, 1 - a.tp, tc, None, true);
    } else {
        // Duel.SendtoHand(tc,1-tp,REASON_EFFECT)
        duel::send_to_hand(d, tc, Some(1 - a.tp), REASON_EFFECT, None);
        // Duel.ConfirmCards(tp,tc)
        duel::confirm_cards(d, a.tp, tc);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - a.tp);
    }
}
