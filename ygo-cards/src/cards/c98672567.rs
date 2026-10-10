//! Pot of Dichotomy
use super::prelude::*;

pub const ID: u32 = 98672567;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetActivityCount(tp,ACTIVITY_BATTLE_PHASE)==0 and not Duel.CheckPhaseActivity()
    duel::get_activity_count(d, a.tp, &[ACTIVITY_BATTLE_PHASE])[0] == 0 && !duel::check_phase_activity(d)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    true
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsMonster() and c:IsAbleToDeck() and c:IsCanBeEffectTarget(e)
    card::is_monster(d, c) && card::is_able_to_deck(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// The value `Card.GetRace` gives `aux.dpcheck`
fn race_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_race(d, c) as i64
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if not Duel.IsPlayerCanDraw(tp,2) then return false end
        if !duel::is_player_can_draw(d, a.tp, 2) {
            return false;
        }
        // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil,e)
        let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e)), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // return g:GetClassCount(Card.GetRace)>=3
        return group::get_class_count(d, &g, &|d, c| card::get_race(d, c) as i64) >= 3;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local tg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dpcheck(Card.GetRace),1,tp,HINTMSG_TODECK)
    let tg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(3),
        Some(3),
        Some(&aux::dpcheck(race_value)),
        1,
        a.tp,
        Some(HINTMSG_TODECK as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, &tg);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,3,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg), 3, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    // if not tg or tg:FilterCount(Card.IsRelateToEffect,nil,e)~=3 then return end
    let tg = match chain::get_target_cards(d, 0) {
        Value::Group(g) => g,
        _ => return,
    };
    if group::filter_count(d, &tg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX) != 3 {
        return;
    }
    // Duel.SendtoDeck(tg,nil,SEQ_DECKTOP,REASON_EFFECT)
    duel::send_to_deck(d, &tg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    // local g=Duel.GetOperatedGroup()
    let g = duel::get_operated_group(d);
    // if g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
    if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, a.tp);
    }
    // local ct=g:FilterCount(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let ct = group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if ct==3 then
    if ct == 3 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, a.tp, 2, REASON_EFFECT);
    }
}
