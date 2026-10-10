//! Summon Sorceress
use super::prelude::*;

pub const ID: u32 = 61665245;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon Procedure
    // Link.AddProcedure(c,aux.NOT(aux.FilterBoolFunctionEx(Card.IsType,TYPE_TOKEN)),2,3,s.lcheck)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| !card::is_type_ctx(d, c, TYPE_TOKEN, sc, st, Some(tp)))),
        2,
        Some(3),
        Some(group_check(|d, g, lc, st, tp| lcheck(d, g, lc, st, tp))),
        None,
        None,
    );
    // Special Summon 1 monster from your hand in Defense Position, to your opponent's zone this card points to
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:CheckSameProperty(Card.GetRace,lc,sumtype,tp)
    group::check_same_property(d, g, &|d, c| card::get_race_ctx(d, c, Some(lc), sumtype, Some(tp))).0
}
/// `function(e) return e:GetHandler():IsLinkSummoned() end` (e1)
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
/// `s.hspfilter(c,e,tp,zone)`
fn hspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE,1-tp,zone)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(1 - tp), Some(zone))
}
/// `s.dspfilter(c,e,tp,rac)`
fn dspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rac: u64) -> bool {
    // return c:IsRace(rac) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_race(d, c, rac) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local zone=e:GetHandler():GetLinkedZone(1-tp)
    let h = effect::get_handler(d, e).nil();
    let zone = card::get_linked_zone(d, h, Some(1 - tp));
    // if chk==0 then return Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)>0
    //     and Duel.IsExistingMatchingCard(s.hspfilter,tp,LOCATION_HAND,0,1,nil,e,tp,zone) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| hspfilter(d, c, e, tp, zone)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local zone=c:GetLinkedZone(1-tp)
    let zone = card::get_linked_zone(d, c, Some(1 - tp));
    // if Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)<=0 then return end
    if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local hsc=Duel.SelectMatchingCard(tp,s.hspfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp,zone):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| hspfilter(d, x, e, tp, zone)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let hsc = group::get_first(d, &sel);
    // if hsc and Duel.SpecialSummonStep(hsc,0,tp,1-tp,false,false,POS_FACEUP_DEFENSE,zone) then
    if let Some(hsc) = hsc {
        if duel::special_summon_step(d, hsc, 0, tp, 1 - tp, false, false, POS_FACEUP_DEFENSE as u8, Some(zone)) {
            // Negate its effects
            // hsc:NegateEffects(c)
            card::negate_effects(d, hsc, c, None, false, None);
        }
    }
    // if Duel.SpecialSummonComplete()==0 then return end
    if duel::special_summon_complete(d) == 0 {
        return;
    }
    // local rac=hsc:GetRace()
    let rac = card::get_race(d, hsc.nil());
    // if not (Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.dspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,rac)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1))) then return end
    if !(duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| dspfilter(d, x, e, tp, rac)), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1)))
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local dsc=Duel.SelectMatchingCard(tp,s.dspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,rac):GetFirst()
    let sel2 = duel::select_matching_card(d, tp, Some(&|d, x| dspfilter(d, x, e, tp, rac)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if not dsc then return end
    let Some(dsc) = group::get_first(d, &sel2) else {
        return;
    };
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.SpecialSummonStep(dsc,0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
    if duel::special_summon_step(d, dsc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
        // Negate its effects
        // dsc:NegateEffects(c)
        card::negate_effects(d, dsc, c, None, false, None);
    }
    // if Duel.SpecialSummonComplete()==0 then return end
    if duel::special_summon_complete(d) == 0 {
        return;
    }
    // Cannot Special Summon, except monsters with the same original Type as the monster Special Summoned from the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsOriginalRace(dsc:GetOriginalRace()) end)
    effect::set_target(
        d,
        e1,
        Some(effect_card_fn(move |d, _e, x| {
            let orace = card::get_original_race(d, dsc);
            !card::is_original_race(d, x, orace)
        })),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
