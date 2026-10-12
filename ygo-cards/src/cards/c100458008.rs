//! A Journey Back to the Light - "Zenet" (--光帰の旅－『セネト』)
use super::prelude::*;

pub const ID: u32 = 100458008;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ZENET], ..ScriptInfo::DEFAULT };

/// `LOCATIONS_HAND_DECK_ONFIELD_GRAVE`
const LOCATIONS_HAND_DECK_ONFIELD_GRAVE: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD | LOCATION_GRAVE;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Show 2 or 3 Normal Monster Cards in your hand, Deck, face-up field, and/or GY, then apply the appropriate effect based on the number shown, then if you showed a card(s) in the hand, you can draw 1 card
    // --● 2: Special Summon 1 Level 8 or lower non-Effect Fusion Monster from your Extra Deck as a Normal Monster Card
    // --● 3: Special Summon 1 "Zenet" monster from your Deck or 1 Normal Monster from your GY
    // --You can only activate 1 "A Journey Back to the Light - "Zenet"" per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DRAW);
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
    // --Prevent the non-Effect Fusion Monster Special Summoned as a Normal Monster Card from being returned to the hand or Main Deck as cost
    // aux.GlobalCheck(s,function() ... end)
    // (the redefinition of Card.IsAbleToHandAsCost / Card.IsAbleToDeckAsCost lives in lua/card.rs, AS_COST_GUARDS)
    aux::global_check(d, ID, |_| {});
}
// function s.showfilter(c)
//     return c:IsOriginalType(TYPE_NORMAL) and (c:IsFaceup() or not c:IsOnField())
// end
fn showfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_original_type(d, c, TYPE_NORMAL) && (card::is_faceup(d, c) || !card::is_on_field(d, c))
}
// function s.exspfilter(c,e,tp)
//     return c:IsLevelBelow(8) and c:IsNonEffectMonster() and c:IsFusionMonster() and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
//         and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
// end
fn exspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 8)
        && card::is_non_effect_monster(d, c)
        && card::is_fusion_monster(d, c)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.deckgyspfilter(c,e,tp)
//     return ((c:IsSetCard(SET_ZENET) and c:IsLocation(LOCATION_DECK)) or (c:IsType(TYPE_NORMAL) and c:IsLocation(LOCATION_GRAVE)))
//         and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
// end
fn deckgyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    ((card::is_set_card(d, c, &[SET_ZENET as u16]) && card::is_location(d, c, LOCATION_DECK))
        || (card::is_type(d, c, TYPE_NORMAL) && card::is_location(d, c, LOCATION_GRAVE)))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local show_count=Duel.GetMatchingGroupCount(s.showfilter,tp,LOCATIONS_HAND_DECK_ONFIELD_GRAVE,0,nil)
        let show_count = duel::get_matching_group_count(d, Some(&showfilter), tp, LOCATIONS_HAND_DECK_ONFIELD_GRAVE, 0, NO_EX);
        // local option_1=show_count>=2 and Duel.IsExistingMatchingCard(s.exspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
        let option_1 = show_count >= 2 && duel::is_existing_matching_card(d, Some(&|d, c| exspfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
        // local option_2=show_count>=3 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.deckgyspfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp)
        let option_2 = show_count >= 3
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| deckgyspfilter(d, c, a.e, tp)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
        // return option_1 or option_2
        return option_1 || option_2;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_EXTRA | LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
// function s.rescon(option_1,option_2)
//     return function(sg,e,tp,mg)
//         return (#sg==2 and option_1) or (#sg==3 and option_2)
//     end
// end
fn rescon(option_1: bool, option_2: bool) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |_d, sg, _e, _tp, _mg, _c| ((sg.len() == 2 && option_1) || (sg.len() == 3 && option_2), false)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local show_group=Duel.GetMatchingGroup(s.showfilter,tp,LOCATIONS_HAND_DECK_ONFIELD_GRAVE,0,nil)
    let show_group = duel::get_matching_group(d, Some(&showfilter), tp, LOCATIONS_HAND_DECK_ONFIELD_GRAVE, 0, NO_EX);
    // if #show_group==0 then return end
    if show_group.is_empty() {
        return;
    }
    // local option_1=#show_group>=2 and Duel.IsExistingMatchingCard(s.exspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
    let option_1 = show_group.len() >= 2 && duel::is_existing_matching_card(d, Some(&|d, c| exspfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    // local option_2=#show_group>=3 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.deckgyspfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp)
    let option_2 = show_group.len() >= 3
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| deckgyspfilter(d, c, a.e, tp)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    // if not (option_1 or option_2) then return end
    if !(option_1 || option_2) {
        return;
    }
    // local rescon=s.rescon(option_1,option_2)
    let rescon_fn = rescon(option_1, option_2);
    let rs: aux::Rescon = &rescon_fn;
    // local sg=aux.SelectUnselectGroup(show_group,e,tp,2,3,rescon,1,tp,HINTMSG_CONFIRM,rescon)
    let sg = aux::select_unselect_group(d, &show_group, Some(a.e), tp, Some(2), Some(3), Some(rs), 1, tp, Some(HINTMSG_CONFIRM as u64), Some(rs), None, false).unwrap_or_default();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local hdg,fgg=sg:Split(Card.IsLocation,nil,LOCATION_HAND|LOCATION_DECK)
    let (hdg, fgg) = group::split(d, &sg, &|d, c| card::is_location(d, c, LOCATION_HAND | LOCATION_DECK), NO_EX);
    // if #fgg>0 then Duel.HintSelection(fgg) end
    if !fgg.is_empty() {
        duel::hint_selection(d, fgg.clone(), true);
    }
    // if #hdg>0 then Duel.ConfirmCards(1-tp,hdg) end
    if !hdg.is_empty() {
        duel::confirm_cards(d, 1 - tp, hdg.clone());
    }
    // if hdg:IsExists(Card.IsLocation,1,nil,LOCATION_HAND) then Duel.ShuffleHand(tp) end
    if group::is_exists(d, &hdg, &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX) {
        duel::shuffle_hand(d, tp);
    }
    // if hdg:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if group::is_exists(d, &hdg, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, tp);
    }
    // local deck_shuffle_chk=false
    let mut deck_shuffle_chk = false;
    // local summon_success_chk=false
    let mut summon_success_chk = false;
    // local hand_reveal_chk=sg:IsExists(Card.IsLocation,1,nil,LOCATION_HAND)
    let hand_reveal_chk = group::is_exists(d, &sg, &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX);
    // if #sg==2 then
    if sg.len() == 2 {
        // --● 2: Special Summon 1 Level 8 or lower non-Effect Fusion Monster from your Extra Deck as a Normal Monster Card
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.exspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| exspfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
        if let Some(sc) = sc {
            if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                // local c=e:GetHandler()
                let c = effect::get_handler(d, a.e).nil();
                // local original_type=sc:GetType()
                let original_type = card::get_type(d, sc);
                // sc:Type((original_type&~TYPE_FUSION)|TYPE_NORMAL)
                card::info_type(d, sc, Some((original_type & !TYPE_FUSION) | TYPE_NORMAL));
                // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,original_type,aux.Stringid(id,1))
                card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, original_type as i64, aux::stringid(ID, 1));
                // Reset its type back to the original when it stops being face-up in the Monster Zone
                restore_type_on_leave(d, c, sc, original_type, tp);
            }
        }
        // summon_success_chk=Duel.SpecialSummonComplete()>0
        summon_success_chk = duel::special_summon_complete(d) > 0;
    } else if sg.len() == 3 {
        // --● 3: Special Summon 1 "Zenet" monster from your Deck or 1 Normal Monster from your GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.deckgyspfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, c| deckgyspfilter(d, c, a.e, tp))),
            tp,
            LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let sc = group::get_first(d, &g);
        // if not sc then return end
        let Some(sc) = sc else {
            return;
        };
        // deck_shuffle_chk=sc:IsLocation(LOCATION_DECK)
        deck_shuffle_chk = card::is_location(d, sc, LOCATION_DECK);
        // summon_success_chk=Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0
        summon_success_chk = duel::special_summon(d, CardSet::from_card(sc), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0;
    }
    // if deck_shuffle_chk then Duel.ShuffleDeck(tp) end
    if deck_shuffle_chk {
        duel::shuffle_deck(d, tp);
    }
    // if hand_reveal_chk and summon_success_chk and Duel.IsPlayerCanDraw(tp,1)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if hand_reveal_chk && summon_success_chk && duel::is_player_can_draw(d, tp, 1) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
/// The type reset of a Normal Monster Card Special Summoned as a non-Effect Fusion Monster.
/// (`e0a` and `e0b` of the Lua, the same in Sennet Per Akh and Zenet journey)
fn restore_type_on_leave(d: &mut Duel, c: CardId, sc: CardId, original_type: u32, tp: u8) {
    // local e0a=Effect.CreateEffect(c)
    let e0a = effect::create_effect(d, c);
    // e0a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0a:SetCode(EVENT_ADJUST)
    effect::set_code(d, e0a, EVENT_ADJUST);
    // e0a:SetCondition(function() return not sc:HasFlagEffect(id) end)
    effect::set_condition(d, e0a, Some(lua_fn(move |d, _a: Args| !card::has_flag_effect(d, sc, ID, None))));
    // e0a:SetOperation(function() sc:Type(original_type) e0a:Reset() end)
    effect::set_operation(
        d,
        e0a,
        Some(operation(move |d, _a| {
            card::info_type(d, sc, Some(original_type));
            effect::reset(d, e0a);
        })),
    );
    // Duel.RegisterEffect(e0a,tp)
    duel::register_effect(d, e0a, tp);
    // local e0b=Effect.CreateEffect(c)
    let e0b = effect::create_effect(d, c);
    // e0b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e0b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0b:SetCode(EFFECT_SEND_REPLACE)
    effect::set_code(d, e0b, EFFECT_SEND_REPLACE);
    // e0b:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e0b, Some(cost_target(move |d, a| send_replace_target(d, a, e0b))));
    // e0b:SetValue(aux.FALSE)
    effect::set_value_fn(d, e0b, aux_false());
    // Duel.RegisterEffect(e0b,tp)
    duel::register_effect(d, e0b, tp);
}
// function(e,tp,eg,ep,ev,re,r,rp,chk) (the e0b target of restore_type_on_leave)
fn send_replace_target(d: &mut Duel, a: &EffArgs, e0b: EffectId) -> bool {
    // local g=eg:Filter(Card.HasFlagEffect,nil,id)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| card::has_flag_effect(d, c, ID, None), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // for tc in g:Iter() do tc:Type(tc:GetFlagEffectLabel(id)) end
    for tc in group::iter(d, &g) {
        let label = card::get_flag_effect_label(d, tc, ID).first().copied().unwrap_or(0);
        card::info_type(d, tc, Some(label as u32));
    }
    // e0b:Reset()
    effect::reset(d, e0b);
    // return true
    true
}
