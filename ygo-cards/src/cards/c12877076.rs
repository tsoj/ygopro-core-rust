//! Awakening of the Crystal Ultimates
use super::prelude::*;

pub const ID: u32 = 12877076;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CRYSTAL_BEAST, SET_ULTIMATE_CRYSTAL, SET_RAINBOW_BRIDGE],
    listed_names: &[40854824],
    ..ScriptInfo::DEFAULT
};

/// `LOCATION_HAND_DECK_GRAVE_SZONE` (a local of the Lua script)
const LOCATION_HAND_DECK_GRAVE_SZONE: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE | LOCATION_SZONE;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE as u32));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ULTIMATE_CRYSTAL) and c:IsMonster() and not c:IsPublic()
    card::is_set_card(d, c, &[SET_ULTIMATE_CRYSTAL as u16]) && card::is_monster(d, c) && !card::is_public(d, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local reveal=Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil)
    let reveal = duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    // local control=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_ULTIMATE_CRYSTAL),tp,LOCATION_MZONE,0,1,nil)
    let control = duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_ULTIMATE_CRYSTAL as u16]))),
        tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    );
    // if chk==0 then return reveal or control end
    if a.chk == Some(0) {
        return reveal || control;
    }
    // if reveal and (not control or Duel.SelectYesNo(tp,aux.Stringid(id,0))) then
    if reveal && (!control || duel::select_yes_no(d, tp, aux::stringid(ID, 0))) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // e:SetLabel(1)
        effect::set_label(d, e, &[1]);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
    }
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_RAINBOW_BRIDGE) or c:IsCode(40854824)) and (c:IsAbleToHand() or c:IsAbleToGrave())
    (card::is_set_card(d, c, &[SET_RAINBOW_BRIDGE as u16]) || card::is_code(d, c, &[40854824]))
        && (card::is_able_to_hand(d, c, None) || card::is_able_to_grave(d, c, None))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsOriginalType(TYPE_MONSTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        && card::is_original_type(d, c, TYPE_MONSTER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local b1=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND_DECK_GRAVE_SZONE,0,1,nil,e,tp)
    let b2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND_DECK_GRAVE_SZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b1 and b2 and e:GetLabel()==0,aux.Stringid(id,3)})
    let lbl = effect::get_label(d, e);
    let op = duel::select_effect(
        d,
        tp,
        &[
            (b1, aux::stringid(ID, 1)),
            (b2, aux::stringid(ID, 2)),
            (b1 && b2 && lbl == 0, aux::stringid(ID, 3)),
        ],
    )
    .map_or(0, |x| x as i64);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE)
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND_DECK_GRAVE_SZONE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND_DECK_GRAVE_SZONE as i32);
    // elseif op==3 then
    } else if op == 3 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND_DECK_GRAVE_SZONE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND_DECK_GRAVE_SZONE as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    }
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local opt=e:GetLabel()
    let opt = effect::get_label(d, e);
    // local b1=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND_DECK_GRAVE_SZONE,0,1,nil,e,tp)
    let b2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(
            d,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))),
            tp,
            LOCATION_HAND_DECK_GRAVE_SZONE,
            0,
            1,
            NO_EX,
        );
    // local break_chk=false
    let mut break_chk = false;
    // if b1 and (opt==1 or opt==3) then
    if b1 && (opt == 1 || opt == 3) {
        // break_chk=true
        break_chk = true;
        // Take 1 "Rainbow Bridge" card or 1 "Rainbow Refraction" and either add it to your hand or send it to the GY
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,4))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 4));
        // local tc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &g);
        // if tc then
        if let Some(tc) = tc {
            // aux.ToHandOrElse(tc,tp)
            aux::to_hand_or_else(d, tc, tp, None, None, None);
        }
    }
    // if b2 and (opt==2 or opt==3) then
    if b2 && (opt == 2 || opt == 3) {
        // Special Summon 1 "Crystal Beast" Monster Card
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND_DECK_GRAVE_SZONE,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))),
            tp,
            LOCATION_HAND_DECK_GRAVE_SZONE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // if break_chk then Duel.BreakEffect() end
            if break_chk {
                duel::break_effect(d);
            }
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
