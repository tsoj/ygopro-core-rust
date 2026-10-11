//! Dangerous Frightfur Nightmary
use super::prelude::*;

pub const ID: u32 = 58468105;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EDGE_IMP, SET_FLUFFAL, SET_FRIGHTFUR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_EDGE_IMP),1,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_FLUFFAL),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![
            (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_EDGE_IMP as u16], Some(fc), st, Some(tp)))), 1),
            (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_FLUFFAL as u16], Some(fc), st, Some(tp)))), 2),
        ],
    );
    // atk up
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    card::register_effect(d, c, e1, false);
    // mill
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_condition(d, e2, Some(condition(gycon)));
    effect::set_target(d, e2, Some(cost_target(gytg)));
    effect::set_operation(d, e2, Some(operation(gyop)));
    card::register_effect(d, c, e2, false);
    // negate
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_DISABLE);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(condition(discon)));
    effect::set_cost(d, e3, Some(cost_target(discost)));
    effect::set_target(d, e3, Some(cost_target(distg)));
    effect::set_operation(d, e3, Some(operation(disop)));
    card::register_effect(d, c, e3, false);
}
// function s.atkcon(e)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnPlayer()==e:GetHandlerPlayer()
    duel::get_turn_player(d) == effect::get_handler_player(d, a.e)
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsRace,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil,RACE_FAIRY|RACE_FIEND)*300
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&|d, c| card::is_race(d, c, RACE_FAIRY | RACE_FIEND)), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 300
}
// function s.gycon(e,tp,eg,ep,ev,re,r,rp)
fn gycon(d: &mut Duel, a: &EffArgs) -> bool {
    // return aux.bdcon(e,tp,eg,ep,ev,re,r,rp) and e:GetHandler():IsFusionSummoned()
    aux::bdcon().call(d, &[Value::Effect(a.e)]).truthy() && {
        let c = effect::get_handler(d, a.e).nil();
        card::is_fusion_summoned(d, c)
    }
}
// function s.gyfilter(c)
fn gyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToGrave() and c:IsMonster() and (c:IsSetCard(SET_EDGE_IMP) or c:IsSetCard(SET_FLUFFAL) or c:IsSetCard(SET_FRIGHTFUR))
    card::is_able_to_grave(d, c, None)
        && card::is_monster(d, c)
        && (card::is_set_card(d, c, &[SET_EDGE_IMP as u16]) || card::is_set_card(d, c, &[SET_FLUFFAL as u16]) || card::is_set_card(d, c, &[SET_FRIGHTFUR as u16]))
}
// function s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // local lv=e:GetHandler():GetBattleTarget():GetLevel()
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c).nil();
    let lv = card::get_level(d, bc);
    if a.chk == Some(0) {
        // return lv>0 and Duel.IsExistingMatchingCard(s.gyfilter,tp,LOCATION_DECK,0,lv,nil)
        return lv > 0 && duel::is_existing_matching_card(d, Some(&gyfilter), a.tp, LOCATION_DECK, 0, lv, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,lv,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, lv as u8, a.tp, LOCATION_DECK as i32);
    true
}
// function s.gyop(e,tp,eg,ep,ev,re,r,rp)
fn gyop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c).nil();
    let lv = card::get_level(d, bc);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&gyfilter), a.tp, LOCATION_DECK, 0, lv as u16, lv as u16, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_grave(d, &g, REASON_EFFECT, None, None);
    }
}
// function s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.rp == a.tp || card::is_status(d, c, STATUS_BATTLE_DESTROYED) || !effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local gp=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let gp = info[0].as_group();
    // return gp and gp:IsContains(c) and Duel.IsChainDisablable(ev)
    gp.is_some_and(|g| group::is_contains(g, c)) && duel::is_chain_disablable(d, a.ev as u8)
}
// function s.disfilter(c)
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FRIGHTFUR) and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_FRIGHTFUR as u16]) && card::is_able_to_remove_as_cost(d, c, None)
}
// function s.discost(e,tp,eg,ep,ev,re,r,rp,chk)
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&disfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&disfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
