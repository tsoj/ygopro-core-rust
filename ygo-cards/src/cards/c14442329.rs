//! JJ "Kewl Tune"
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 14442329;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[41069676],
    listed_series: &[SET_KEWL_TUNE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // You can Normal Summon 1 Tuner in addition to your Normal Summon/Set
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_HAND | LOCATION_MZONE, 0);
    // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsType,TYPE_TUNER))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_TUNER))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // "Kewl Tune Loudness War" you control gains 3300 ATK while your opponent has a Tuner in their field or GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsCode,41069676))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[41069676]))));
    // e2:SetValue(3300)
    effect::set_value(d, e2, 3300);
    // e2:SetCondition(function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_TUNER),e:GetHandlerPlayer(),0,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add to your hand, or Special Summon, 1 "Kewl Tune" monster from your Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(s.thspcost)
    effect::set_cost(d, e3, Some(cost_target(thspcost)));
    // e3:SetTarget(s.thsptg)
    effect::set_target(d, e3, Some(cost_target(thsptg)));
    // e3:SetOperation(s.thspop)
    effect::set_operation(d, e3, Some(operation(thspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.costfilter(c,e,tp)
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_TUNER) and Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,Duel.GetMZoneCount(tp,c)>0)
    if !card::is_type(d, c, TYPE_TUNER) {
        return false;
    }
    let mzone = duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0;
    duel::is_existing_matching_card(d, Some(&|d, cc| thspfilter(d, cc, e, tp, mzone)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
// function s.thspfilter(c,e,tp,mmz_chk)
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // return c:IsSetCard(SET_KEWL_TUNE) and c:IsMonster()
    //     and (c:IsAbleToHand() or (mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_set_card(d, c, &[SET_KEWL_TUNE as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None) || (mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
// function s.thspcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn thspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil,e,tp)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND as u64, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON as u64, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.thspop(e,tp,eg,ep,ev,re,r,rp)
fn thspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,mmz_chk):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| thspfilter(d, c, e, tp, mmz_chk)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc then
    if let Some(sc) = sc {
        // aux.ToHandOrElse(sc,tp,
        //     function() return mmz_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
        //     function() Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
        //     aux.Stringid(id,3))
        aux::to_hand_or_else(
            d,
            sc,
            tp,
            Some(&|d, _c| mmz_chk && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None)),
            Some(&|d, _g| {
                duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                0
            }),
            Some(aux::stringid(ID, 3)),
        );
    }
    // --You cannot Special Summon for the rest of this turn, except Tuners
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e1, aux::stringid(ID, 4));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function s.splimit(e,c)
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // if c:IsMonster() then
    if card::is_monster(d, c) {
        // return not c:IsType(TYPE_TUNER)
        !card::is_type(d, c, TYPE_TUNER)
    // elseif c:IsMonsterCard() then
    } else if card::is_monster_card(d, c) {
        // return not c:IsOriginalType(TYPE_TUNER)
        !card::is_original_type(d, c, TYPE_TUNER)
    } else {
        // end (nil: false)
        false
    }
}
// function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_TUNER),e:GetHandlerPlayer(),0,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_TUNER)),
        p,
        0,
        LOCATION_MZONE | LOCATION_GRAVE,
        1,
        NO_EX,
    )
}
