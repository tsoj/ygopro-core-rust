//! Chimera Fusion
//! Scripted by Larry126
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 63136489;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={CARD_CHIMERA_MYTHICAL_BEAST,5818798,77207191}
    listed_names: &[CARD_CHIMERA_MYTHICAL_BEAST, 5818798, 77207191],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff(c,nil,nil,function() return nil,s.matcheck end)
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            extrafil: Some(Arc::new(|_d: &mut Duel, _e: EffectId, _tp: u8, _mg: &CardSet| (None, Some(lua_fn(matcheck))))),
            ..Default::default()
        },
    );
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.matcheck(tp,sg,fc)
fn matcheck(d: &mut Duel, a: Args) -> bool {
    let sg = a.g(1);
    // return sg:IsExists(Card.IsRace,1,nil,RACE_BEAST|RACE_FIEND,fc,SUMMON_TYPE_FUSION,tp)
    // (the extra arguments of Card.IsRace are ignored by the C function)
    group::is_exists(d, &sg, &|d, c| card::is_race(d, c, RACE_BEAST | RACE_FIEND), 1, NO_EX)
}
// function s.effcon(e,tp,eg,ep,ev,re,r,rp)
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_CHIMERA_MYTHICAL_BEAST),tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_CHIMERA_MYTHICAL_BEAST]))),
        a.tp,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        0,
        1,
        NO_EX,
    )
}
// function s.spfilter(c,e,tp,code1,code2)
/// `codes` are the `code1, code2` arguments; a nil one is left out (`IsCode` skips nil arguments)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, codes: &[u32]) -> bool {
    // return c:IsCode(code1,code2) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, codes) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local b1=c:IsAbleToHand()
    let b1 = card::is_able_to_hand(d, c, None);
    // local b2=c:IsAbleToRemove() and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp,5818798)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp,77207191)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    let b2 = card::is_able_to_remove(d, c, None, None, None)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
        && duel::is_existing_matching_card(
            d,
            Some(&|d, x| spfilter(d, x, a.e, tp, &[5818798])),
            tp,
            LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        )
        && duel::is_existing_matching_card(
            d,
            Some(&|d, x| spfilter(d, x, a.e, tp, &[77207191])),
            tp,
            LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        )
        && !duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some();
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).nil() as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, 0);
    } else {
        // e:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // Add this card to your hand
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,c)
        duel::confirm_cards(d, 1 - tp, c);
    } else if duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, c, LOCATION_REMOVED)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
        && !duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        // Banish this card and Special Summon 1 "Gazelle the King of Mythical Beasts" and 1 "Berfomet" from your Deck and/or GY
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp,5818798,77207191)
        let g = duel::get_matching_group(
            d,
            Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp, &[5818798, 77207191]))),
            tp,
            LOCATION_DECK | LOCATION_GRAVE,
            0,
            NO_EX,
        );
        // if g:GetClassCount(Card.GetCode)==2 then
        if group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64) == 2 {
            // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
            let rs = aux::dncheck();
            let sg = aux::select_unselect_group(
                d,
                &g,
                Some(a.e),
                tp,
                Some(2),
                Some(2),
                Some(&rs),
                1,
                tp,
                Some(HINTMSG_SPSUMMON as u64),
                None,
                None,
                false,
            )
            .unwrap_or_default();
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
