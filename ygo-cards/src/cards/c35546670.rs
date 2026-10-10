//! World Legacy Scars
use super::prelude::*;

pub const ID: u32 = 35546670;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MEKK_KNIGHT, SET_WORLD_LEGACY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --atk/def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MEKK_KNIGHT))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MEKK_KNIGHT as u16]))));
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetValue(300)
    effect::set_value(d, e2, 300);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --draw
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e4, CATEGORY_DRAW);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCost(s.drcost)
    effect::set_cost(d, e4, Some(cost_target(drcost)));
    // e4:SetTarget(s.drtg)
    effect::set_target(d, e4, Some(cost_target(drtg)));
    // e4:SetOperation(s.drop)
    effect::set_operation(d, e4, Some(operation(drop_op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // --to grave
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e5, CATEGORY_TOGRAVE);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCost(s.tgcost)
    effect::set_cost(d, e5, Some(cost_target(tgcost)));
    // e5:SetTarget(s.tgtg)
    effect::set_target(d, e5, Some(cost_target(tgtg)));
    // e5:SetOperation(s.tgop)
    effect::set_operation(d, e5, Some(operation(tgop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.costfilter1(c)
fn costfilter1(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsSetCard(SET_MEKK_KNIGHT) and c:IsMonster()) or c:IsSetCard(SET_WORLD_LEGACY)) and c:IsDiscardable()
    ((card::is_set_card(d, c, &[SET_MEKK_KNIGHT as u16]) && card::is_monster(d, c)) || card::is_set_card(d, c, &[SET_WORLD_LEGACY as u16])) && card::is_discardable(d, c, None, None)
}
// function s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter1,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter1), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.DiscardHand(tp,s.costfilter1,1,1,REASON_DISCARD|REASON_COST)
    duel::discard_hand(d, a.tp, Some(&costfilter1), 1, 1, REASON_DISCARD | REASON_COST, NO_EX);
    true
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let count = info[1].int() as u32;
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, p, count, REASON_EFFECT);
}
// function s.costfilter2(c)
fn costfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MEKK_KNIGHT) and c:IsMonster() and (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup()) and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_MEKK_KNIGHT as u16])
        && card::is_monster(d, c)
        && (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
        && card::is_able_to_remove_as_cost(d, c, None)
}
// function s.tgcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.costfilter2,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&costfilter2), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, NO_EX);
    let dncheck = aux::dncheck();
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,8,8,aux.dncheck,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(8), Some(8), Some(&dncheck), 0, a.tp, None, None, None, false), Err(true));
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,8,8,aux.dncheck,1,tp,HINTMSG_REMOVE)
    let rg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(8), Some(8), Some(&dncheck), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false);
    // if rg then
    if let Ok(rg) = rg {
        // Duel.Remove(rg,POS_FACEUP,REASON_COST)
        duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
    }
    true
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,0,LOCATION_HAND|LOCATION_EXTRA,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, 0, LOCATION_HAND | LOCATION_EXTRA, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_HAND|LOCATION_EXTRA,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, 0, LOCATION_HAND | LOCATION_EXTRA, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, 0, 0);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_HAND|LOCATION_EXTRA,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, 0, LOCATION_HAND | LOCATION_EXTRA, NO_EX);
    // Duel.SendtoGrave(g,REASON_EFFECT)
    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
}
