//! Lifeless Leaffish
//! 揺海魚デッドリーフ
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 89617515;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send to grave
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.gytg)
    effect::set_target(d, e1, Some(cost_target(gytg)));
    // e1:SetOperation(s.gyop)
    effect::set_operation(d, e1, Some(operation(gyop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Shuffle Fish monsters into the Deck and draw 1 card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e4, CATEGORY_TODECK + CATEGORY_DRAW);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e4, LOCATION_GRAVE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e4, Some(cost::self_banish()));
    // e4:SetTarget(s.drtg)
    effect::set_target(d, e4, Some(cost_target(drtg)));
    // e4:SetOperation(s.drop)
    effect::set_operation(d, e4, Some(operation(drop_op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.gyfilter(c)
fn gyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FISH) and not c:IsCode(id) and c:IsAbleToGrave()
    card::is_race(d, c, RACE_FISH) && !card::is_code(d, c, &[ID]) && card::is_able_to_grave(d, c, None)
}
// function s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.gyfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&gyfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.gyop(e,tp,eg,ep,ev,re,r,rp)
fn gyop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.gyfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&gyfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsRace(RACE_FISH) and c:IsAbleToDeck()
    card::is_monster(d, c) && card::is_race(d, c, RACE_FISH) && card::is_able_to_deck(d, c)
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc);
    }
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1)
    //     and Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,3,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1) && duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 3, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,3,3,e:GetHandler())
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 3, 3, false, h.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g.clone()), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg<=0 then return end
    if tg.is_empty() {
        return;
    }
    // Duel.SendtoDeck(tg,nil,SEQ_DECKTOP,REASON_EFFECT)
    duel::send_to_deck(d, tg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // if g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, tp);
    }
    // local ct=g:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let ct = group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if ct>0 then
    if ct > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
