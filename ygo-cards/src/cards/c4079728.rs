//! Gung-Ho! Springans!
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 4079728;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_ALBAZ, 29601381],
    listed_series: &[SET_SPRINGANS, SET_THERION],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 4 "Springans" monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_SPRINGANS),4,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_SPRINGANS as u16], sc, st, Some(tp)))),
        Some(4),
        2,
        None,
        None,
        Some(xyz::INFINITE_MATS),
        None,
        false,
        None,
    );
    // Special Summon 1 "Springans Captain Sargas" from your Deck or GY
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1a, CATEGORY_SPECIAL_SUMMON);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1a, EVENT_TO_GRAVE);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetTarget(s.sptg)
    effect::set_target(d, e1a, Some(cost_target(sptg)));
    // e1a:SetOperation(s.spop)
    effect::set_operation(d, e1a, Some(operation(spop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // e1b:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e1b, Some(lua_fn(e1b_cond)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Add to your hand, or Special Summon, 1 "Springans" or "Therion" monster or "Fallen of Albaz" or monster that mentions it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(2))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(2), None, None)));
    // e2:SetTarget(s.thsptg)
    effect::set_target(d, e2, Some(cost_target(thsptg)));
    // e2:SetOperation(s.thspop)
    effect::set_operation(d, e2, Some(operation(thspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn e1b_cond(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsXyzSummoned()
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_xyz_summoned(d, h)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(29601381) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[29601381]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let necro = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp));
    let g = duel::select_matching_card(d, a.tp, Some(&necro), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // if #g>0 then
    if let Some(g) = g {
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // return (c:IsSetCard({SET_SPRINGANS,SET_THERION}) or (c:IsCode(CARD_ALBAZ) or c:ListsCode(CARD_ALBAZ))) and c:IsMonster()
    //     and (c:IsAbleToHand() or (mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    (card::is_set_card(d, c, &[SET_SPRINGANS as u16, SET_THERION as u16])
        || (card::is_code(d, c, &[CARD_ALBAZ]) || card::lists_code(d, c, &[CARD_ALBAZ])))
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None)
            || (mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0) end
    if a.chk == Some(0) {
        let mmz = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        return duel::is_existing_matching_card(d, Some(&|d, c| thspfilter(d, c, a.e, a.tp, mmz)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thspop(d: &mut Duel, a: &EffArgs) {
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,mmz_chk):GetFirst()
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| thspfilter(d, c, a.e, a.tp, mmz_chk)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
    let sc = sg.and_then(|g| group::get_first(d, &g));
    // if not sc then return end
    let Some(sc) = sc else { return };
    // aux.ToHandOrElse(sc,tp,
    //     function() return mmz_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
    //     function() return Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
    //     aux.Stringid(id,3))
    aux::to_hand_or_else(
        d,
        sc,
        a.tp,
        Some(&|d, _c| mmz_chk && card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, None, None, None)),
        Some(&|d, _g| duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None)),
        Some(aux::stringid(ID, 3)),
    );
}
