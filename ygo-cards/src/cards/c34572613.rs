//! Myutant Evolution Lab
use super::prelude::*;

pub const ID: u32 = 34572613;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MYUTANT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // Your "Myutant" monsters gains ATK
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
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MYUTANT))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MYUTANT as u16]))));
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);

    // Place 1 card from hand to deck, draw 1
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.drtg)
    effect::set_target(d, e3, Some(cost_target(drtg)));
    // e3:SetOperation(s.drop)
    effect::set_operation(d, e3, Some(operation(drop_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MYUTANT) and c:GetLevel()<5 and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MYUTANT as u16])
        && card::get_level(d, c) < 5
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_REMOVED,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_HAND | LOCATION_REMOVED, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MYUTANT)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MYUTANT as u16])
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=Duel.GetMatchingGroup(s.atkfilter,e:GetHandlerPlayer(),LOCATION_REMOVED,0,nil)
    let tp = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&atkfilter), tp, LOCATION_REMOVED, 0, NO_EX);
    // return g:GetClassCount(Card.GetCode)*100
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i64 * 100
}
/// `s.drfilter(c)`
fn drfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MYUTANT) and c:IsMonster() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_MYUTANT as u16]) && card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1)
    //     and Duel.IsExistingMatchingCard(s.drfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1) && duel::is_existing_matching_card(d, Some(&drfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.drfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&drfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // if Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)>0 then
    if duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) > 0 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
