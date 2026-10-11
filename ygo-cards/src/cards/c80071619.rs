//! Ｒ．Ｂ．シェパード・クルーク
//! R.B. Shepherd's Crook
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 80071619;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RB], material_setcode: &[SET_RB as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, including an "R.B." monster
    // Link.AddProcedure(c,nil,2,3,s.linkmatcheck)
    link::add_procedure(d, c, None, 2, Some(3), Some(group_check(|d, g, lc, st, tp| linkmatcheck(d, g, lc, st, tp))), None, None);
    // Gains 500 ATK for each monster you control, except this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return 500*Duel.GetMatchingGroupCount(nil,c:GetControler(),LOCATION_MZONE,0,e:GetHandler()) end)
    effect::set_value_fn(
        d,
        e1,
        effect_card_value(|d, e, x| {
            let h = effect::get_handler(d, e).nil();
            let p = card::get_controler(d, x);
            500 * duel::get_matching_group_count(d, None, p, LOCATION_MZONE, 0, h.into()) as i64
        }),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 1 "R.B." Spell/Trap from your Deck or GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_LEAVE_GRAVE+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_LEAVE_GRAVE + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Target 3 Level 3 or higher "R.B." monsters; place 2 on the bottom of the Deck in any order, Special Summon the third
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsMainPhase(1-tp) end)
    effect::set_condition(d, e3, Some(lua_fn(|d, a: Args| duel::is_main_phase(d, Some(1 - a.p(1))))));
    // e3:SetTarget(s.tdsptg)
    effect::set_target(d, e3, Some(cost_target(tdsptg)));
    // e3:SetOperation(s.tdspop)
    effect::set_operation(d, e3, Some(operation(tdspop)));
    // e3:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.linkmatcheck(g,lc,sumtype,tp)`
fn linkmatcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsSetCard,1,nil,SET_RB,lc,sumtype,tp)
    group::is_exists(d, g, &|d, x| card::is_set_card_ctx(d, x, &[SET_RB as u16], Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RB) and c:IsTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_RB as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
        // (the 6th argument is a number, not a card, so the core excludes nothing)
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.setfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let nv = aux::necro_valley_filter(|d, x| setfilter(d, x));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g)
        duel::s_set(d, tp, g, None, true);
    }
}
/// `s.tdspfilter(c,e,tp)`
fn tdspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelAbove(3) and c:IsSetCard(SET_RB) and c:IsCanBeEffectTarget(e)
    //     and (c:IsAbleToDeck() or c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE))
    card::is_level_above(d, c, 3)
        && card::is_set_card(d, c, &[SET_RB as u16])
        && card::is_can_be_effect_target(d, c, Some(e))
        && (card::is_able_to_deck(d, c) || card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(s.spcheck,1,nil,e,tp,sg)
    let e = e.nil();
    (group::is_exists(d, sg, &|d, x| spcheck(d, x, e, tp, sg), 1, NO_EX), false)
}
/// `s.spcheck(c,e,tp,sg)`
fn spcheck(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: &CardSet) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) and sg:IsExists(Card.IsAbleToDeck,2,c)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && group::is_exists(d, sg, &|d, x| card::is_able_to_deck(d, x), 2, c.into())
}
/// `s.tdsptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tdspfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| tdspfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g>=3 and aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,0)
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && g.len() >= 3
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_TARGET)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg.clone()), 2, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), 1, tp, 0);
    true
}
/// `s.tdrescon(sg,e,tp,mg)`
fn tdrescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return mg:IsExists(s.spcheck,1,sg,e,tp,sg)
    let e = e.nil();
    (group::is_exists(d, mg, &|d, x| spcheck(d, x, e, tp, sg), 1, sg.into()), false)
}
/// `s.tdspop(e,tp,eg,ep,ev,re,r,rp)`
fn tdspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, e).nil();
    // if #tg<2 then return end
    if tg.len() < 2 {
        return;
    }
    // if #tg==2 and Duel.SendtoDeck(tg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 then
    if tg.len() == 2 && duel::send_to_deck(d, tg.clone(), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0 {
        // local ct=tg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)
        let ct = group::filter_count(d, &tg, &|d, x| card::is_location(d, x, LOCATION_DECK), NO_EX);
        // if ct>0 then Duel.SortDeckbottom(tp,tp,ct) end
        if ct > 0 {
            duel::sort_deckbottom(d, tp, tp, ct as u16);
        }
        // return
        return;
    }
    // local dg=aux.SelectUnselectGroup(tg,e,tp,2,2,s.tdrescon,1,tp,HINTMSG_TODECK)
    let dg = aux::select_unselect_group(d, &tg, Some(e), tp, Some(2), Some(2), Some(&tdrescon), 1, tp, Some(HINTMSG_TODECK as u64), None, None, false)
        .unwrap_or_default();
    // if #dg~=2 then return end
    if dg.len() != 2 {
        return;
    }
    // Duel.HintSelection(dg)
    duel::hint_selection(d, dg.clone(), true);
    // if Duel.SendtoDeck(dg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 and dg:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)==2 then
    if duel::send_to_deck(d, dg.clone(), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0
        && group::filter_count(d, &dg, &|d, x| card::is_location(d, x, LOCATION_DECK | LOCATION_EXTRA), NO_EX) == 2
    {
        // local ct=dg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)
        let ct = group::filter_count(d, &dg, &|d, x| card::is_location(d, x, LOCATION_DECK), NO_EX);
        // if ct>0 then Duel.SortDeckbottom(tp,tp,ct) end
        if ct > 0 {
            duel::sort_deckbottom(d, tp, tp, ct as u16);
        }
        // local sg=tg-dg
        let mut sg = tg.clone();
        group::remove_card(&mut sg, &dg);
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
