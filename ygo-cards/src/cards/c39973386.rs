//! Duality
use super::prelude::*;

pub const ID: u32 = 39973386;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 monster from Hand or Extra Deck with the same original Type and Level
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
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle 1 LIGHT and 1 DARK monster into the Deck and draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK|ATTRIBUTE_LIGHT) and c:HasLevel()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_EXTRA,0,1,nil,e,tp,c:GetOriginalRace(),c:GetOriginalLevel(),c:GetOriginalAttribute(),c)
    card::is_attribute(d, c, ATTRIBUTE_DARK | ATTRIBUTE_LIGHT) && card::has_level(d, c) && {
        let race = card::get_original_race(d, c);
        let lvl = card::get_original_level(d, c);
        let att = card::get_original_attribute(d, c);
        duel::is_existing_matching_card(
            d,
            Some(&|d, x| spfilter(d, x, e, tp, race, lvl, att, Some(c))),
            tp,
            LOCATION_HAND | LOCATION_EXTRA,
            0,
            1,
            NO_EX,
        )
    }
}
/// `s.spfilter(c,e,tp,race,lvl,att,cost_card)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, race: u64, lvl: u32, att: u32, cost_card: Option<CardId>) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK|ATTRIBUTE_LIGHT) and c:IsOriginalRace(race) and c:GetOriginalLevel()==lvl and not c:IsOriginalAttribute(att)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and ((c:IsLocation(LOCATION_HAND) and Duel.GetMZoneCount(tp,cost_card)>0) or (c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,cost_card,c)>0))
    card::is_attribute(d, c, ATTRIBUTE_DARK | ATTRIBUTE_LIGHT)
        && card::is_original_race(d, c, race)
        && card::get_original_level(d, c) == lvl
        && !card::is_original_attribute(d, c, att)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && ((card::is_location(d, c, LOCATION_HAND) && m_zone_count(d, tp, cost_card) > 0)
            || (card::is_location(d, c, LOCATION_EXTRA) && location_count_from_ex(d, tp, cost_card, c) > 0))
}
/// `Duel.GetMZoneCount(tp,cost_card)`
fn m_zone_count(d: &mut Duel, tp: u8, cost_card: Option<CardId>) -> i32 {
    match cost_card {
        Some(x) => duel::get_m_zone_count(d, tp, x.into(), None, None, None).0,
        None => duel::get_m_zone_count(d, tp, NO_EX, None, None, None).0,
    }
}
/// `Duel.GetLocationCountFromEx(tp,tp,cost_card,c)`
fn location_count_from_ex(d: &mut Duel, tp: u8, cost_card: Option<CardId>, c: CardId) -> i32 {
    let ex = match cost_card {
        Some(x) => x.into(),
        None => NO_EX,
    };
    duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0
}
/// `s.cost(e,tp,...,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,e,tp)
        return duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, a.e, tp)), 1, 1, false, None, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local rc=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,e,tp):GetFirst()
    let sel = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, a.e, tp)), 1, 1, false, None, NO_EX);
    let rc = group::get_first(d, &sel).nil();
    // e:SetLabel(rc:GetOriginalRace(),rc:GetOriginalLevel(),rc:GetOriginalAttribute())
    let race = card::get_original_race(d, rc) as i64;
    let lvl = card::get_original_level(d, rc) as i64;
    let att = card::get_original_attribute(d, rc) as i64;
    effect::set_label(d, a.e, &[race, lvl, att]);
    // Duel.Release(rc,REASON_COST)
    duel::release(d, CardSet::from_card(rc), REASON_COST, None);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_EXTRA) as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local race,lvl,att=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let race = labels.first().copied().unwrap_or(0) as u64;
    let lvl = labels.get(1).copied().unwrap_or(0) as u32;
    let att = labels.get(2).copied().unwrap_or(0) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_EXTRA,0,1,1,nil,e,tp,race,lvl,att)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| spfilter(d, c, a.e, tp, race, lvl, att, None)),
        tp,
        LOCATION_HAND | LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tdfilter(c,e)`
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsMonster() and c:IsAttribute(ATTRIBUTE_DARK|ATTRIBUTE_LIGHT) and c:IsAbleToDeck() and c:IsCanBeEffectTarget(e)
    card::is_monster(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_DARK | ATTRIBUTE_LIGHT)
        && card::is_able_to_deck(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_DARK)>0 and sg:FilterCount(Card.IsAttribute,nil,ATTRIBUTE_LIGHT)>0
    let dark = group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_DARK), NO_EX) > 0;
    let light = group::filter_count(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_LIGHT), NO_EX) > 0;
    (dark && light, false)
}
/// `s.drtg(e,tp,...,chk,chkc)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local rg=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE,0,nil,e)
    let rg = duel::get_matching_group(d, Some(&|d, c| tdfilter(d, c, e)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and #rg>1
    //     and aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1)
            && rg.len() > 1
            && matches!(
                aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_TODECK)
    let g = aux::select_unselect_group(
        d,
        &rg,
        Some(e),
        tp,
        Some(2),
        Some(2),
        Some(&rescon),
        1,
        tp,
        Some(HINTMSG_TODECK as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.drop(e,tp,...)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg~=2 or Duel.SendtoDeck(tg,nil,SEQ_DECKTOP,REASON_EFFECT)~=2 then return end
    if tg.len() != 2 || duel::send_to_deck(d, tg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) != 2 {
        return;
    }
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // if g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, tp);
    }
    // local ct=g:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let ct = group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if ct==2 and Duel.IsPlayerCanDraw(tp) then
    if ct == 2 && duel::is_player_can_draw(d, tp, 0) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
