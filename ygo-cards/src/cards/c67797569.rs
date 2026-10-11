//! Lavalval Salamander
use super::prelude::*;

pub const ID: u32 = 67797569;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LAVAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner FIRE monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsAttribute,ATTRIBUTE_FIRE),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, cc, sc, st, tp| {
            card::is_attribute_ctx(d, cc, ATTRIBUTE_FIRE, sc, st, Some(tp))
        })),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Draw 2 cards, then, if you have a FIRE monster in your hand, send 2 cards from your hand to the GY including a FIRE monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(lua_fn(synchro_cond)));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change face-up monsters your opponent controls, up to the number of "Laval" monsters you control, to face-down Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.poscost)
    effect::set_cost(d, e2, Some(cost_target(poscost)));
    // e2:SetTarget(s.postg)
    effect::set_target(d, e2, Some(cost_target(postg)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsSynchroSummoned() end`
fn synchro_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_synchro_summoned(d, h)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(2)
    duel::set_target_param(d, 2);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_FIRE)
    (group::is_exists(d, sg, &|d, cc| card::is_attribute(d, cc, ATTRIBUTE_FIRE), 1, NO_EX), false)
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dn = info[1].int() as u32;
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, p, dn, REASON_EFFECT);
    // Duel.ShuffleHand(p)
    duel::shuffle_hand(d, p);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local cg=Duel.GetFieldGroup(p,LOCATION_HAND,0)
    let cg = duel::get_field_group(d, p, LOCATION_HAND, 0);
    // local og=aux.SelectUnselectGroup(cg,e,tp,2,2,s.rescon,1,tp,HINTMSG_TOGRAVE,s.rescon)
    let og = aux::select_unselect_group(
        d,
        &cg,
        Some(a.e),
        a.tp,
        Some(2),
        Some(2),
        Some(&rescon),
        1,
        a.tp,
        Some(HINTMSG_TOGRAVE as u64),
        Some(&rescon),
        None,
        false,
    )
    .unwrap_or_default();
    // if og and #og>=2 then
    if og.len() >= 2 {
        // local ct=Duel.SendtoGrave(og,REASON_EFFECT)
        let _ct = duel::send_to_grave(d, og, REASON_EFFECT, None, None);
    } else {
        // Duel.ConfirmCards(1-p,cg)
        duel::confirm_cards(d, 1 - p, cg.clone());
        // Duel.SendtoDeck(cg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, cg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        // Duel.ShuffleDeck(p)
        duel::shuffle_deck(d, p);
    }
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.poscost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn poscost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_LAVAL),tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(
        d,
        Some(&aux::faceup_filter(|d, cc| card::is_set_card(d, cc, &[SET_LAVAL as u16]))),
        a.tp,
        LOCATION_MZONE,
        0,
        NO_EX,
    );
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCanTurnSet),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, cc| card::is_can_turn_set(d, cc))), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return ct>0 and #g>0 end
    if a.chk == Some(0) {
        return ct > 0 && !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_LAVAL),tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(
        d,
        Some(&aux::faceup_filter(|d, cc| card::is_set_card(d, cc, &[SET_LAVAL as u16]))),
        a.tp,
        LOCATION_MZONE,
        0,
        NO_EX,
    );
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsCanTurnSet),tp,0,LOCATION_MZONE,1,ct,nil)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::faceup_filter(|d, cc| card::is_can_turn_set(d, cc))),
        a.tp,
        0,
        LOCATION_MZONE,
        1,
        ct as u16,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
