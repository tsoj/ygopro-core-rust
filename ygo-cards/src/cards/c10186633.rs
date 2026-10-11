//! EN Shuffle
use super::prelude::*;

pub const ID: u32 = 10186633;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NEOS],
    listed_series: &[SET_ELEMENTAL_HERO, SET_NEO_SPACIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Elemental HERO" or "Neo-Spacian" monster from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle cards then draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
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
/// `s.tdfilter(c,e,tp)`
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard({SET_ELEMENTAL_HERO,SET_NEO_SPACIAN}) and c:IsFaceup()
    //     and c:IsAbleToDeck() and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetCode())
    if !(card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16, SET_NEO_SPACIAN as u16]) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)) {
        return false;
    }
    if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 <= 0 {
        return false;
    }
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsSetCard({SET_ELEMENTAL_HERO,SET_NEO_SPACIAN}) and not c:IsCode(code)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16, SET_NEO_SPACIAN as u16])
        && !card::is_code(d, c, &[code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| tdfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| tdfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if not tc then return end
    let Some(tc) = group::get_first(d, &sel) else {
        return;
    };
    // Duel.HintSelection(tc,true)
    duel::hint_selection(d, tc, true);
    // local code=tc:GetCode()
    let code = card::get_code(d, tc)[0];
    // if Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_DECK|LOCATION_EXTRA) then
    if duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,code)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tdfilter2(c)`
fn tdfilter2(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_ELEMENTAL_HERO) or c:IsSetCard(SET_NEO_SPACIAN)) and c:IsMonster() and c:IsAbleToDeck()
    (card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]) || card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16]))
        && card::is_monster(d, c)
        && card::is_able_to_deck(d, c)
}
/// `s.resfilter(c,sg)`
fn resfilter(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsSetCard(SET_ELEMENTAL_HERO) and sg:IsExists(Card.IsSetCard,1,c,SET_NEO_SPACIAN)
    card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]) && group::is_exists(d, sg, &|d, x| card::is_set_card(d, x, &[SET_NEO_SPACIAN as u16]), 1, c.into())
}
/// `s.rescon(sg,e,tp)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return (#sg==1 and sg:IsExists(Card.IsCode,1,nil,CARD_NEOS)) or sg:IsExists(s.resfilter,1,nil,sg)
    let one_neos = sg.len() == 1 && group::is_exists(d, sg, &|d, x| card::is_code(d, x, &[CARD_NEOS]), 1, NO_EX);
    let res = one_neos || group::is_exists(d, sg, &|d, x| resfilter(d, x, sg), 1, NO_EX);
    (res, false)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.tdfilter2,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter2), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and aux.SelectUnselectGroup(g,e,tp,1,2,s.rescon,0) end
    if a.chk == Some(0) {
        let rs: aux::Rescon = &rescon;
        return duel::is_player_can_draw(d, a.tp, 1)
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(2), Some(rs), 0, a.tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tdfilter2,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter2), tp, LOCATION_GRAVE, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,2,s.rescon,1,tp,HINTMSG_TODECK,s.rescon)
    let rs: aux::Rescon = &rescon;
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(2), Some(rs), 1, tp, Some(HINTMSG_TODECK as u64), Some(rs), None, false).unwrap_or_default();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.HintSelection(sg,true)
    duel::hint_selection(d, sg.clone(), true);
    let n = sg.len() as i32;
    // if Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==#sg then
    if duel::send_to_deck(d, sg.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == n {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // if og:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)~=#sg then return end
        if group::filter_count(d, &og, &|d, x| card::is_location(d, x, LOCATION_DECK | LOCATION_EXTRA), NO_EX) as i32 != n {
            return;
        }
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
