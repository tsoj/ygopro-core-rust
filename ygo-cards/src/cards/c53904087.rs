//! 神書の使いラハムゥ
//! Rahamu, Envoy of the Sacred Tome
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 53904087;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 Effect Monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2)
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_type_ctx(d, x, TYPE_EFFECT, sc, st, Some(tp)))), 2, None, None, None, None);
    // Once while this Link Summoned card is face-up on the field, you can Normal Summon 1 Level 5 or higher monster without Tributing
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_NO_TURN_RESET)
    effect::set_property(d, e1, EFFECT_FLAG_NO_TURN_RESET, 0);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e1, LOCATION_HAND, 0);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.ntcon)
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    // e1:SetTarget(aux.FieldSummonProcTg(function(e,c) return c:IsLevelAbove(5) end))
    effect::set_target(d, e1, Some(aux::field_summon_proc_tg(Some(effect_card_fn(|d, _e, x| card::is_level_above(d, x, 5))), None)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During the Main Phase, you can (Quick Effect): Immediately after this effect resolves, Normal Summon 1 Level 5 or higher DARK monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e2, CATEGORY_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(lua_fn(|d, _a: Args| duel::is_main_phase(d, None))));
    // e2:SetTarget(s.nstg)
    effect::set_target(d, e2, Some(cost_target(nstg)));
    // e2:SetOperation(s.nsop)
    effect::set_operation(d, e2, Some(operation(nsop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // During your End Phase: You can reveal any number of monsters in your hand and place them on the bottom of the Deck in any order, then draw the same number of cards
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e3, Some(lua_fn(|d, a: Args| duel::is_turn_player(d, a.p(1)))));
    // e3:SetTarget(s.drtg)
    effect::set_target(d, e3, Some(cost_target(drtg)));
    // e3:SetOperation(s.drop)
    effect::set_operation(d, e3, Some(operation(drop_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return e:GetHandler():IsLinkSummoned() and minc==0 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_link_summoned(d, h)
        && a.int(2) == 0
        && duel::get_location_count(d, card::get_controler(d, c), LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.nsfilter(c)`
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSummonable(true,nil) and c:IsLevelAbove(5) and c:IsAttribute(ATTRIBUTE_DARK)
    card::is_summonable(d, c, true, None, 0, None) && card::is_level_above(d, c, 5) && card::is_attribute(d, c, ATTRIBUTE_DARK as u32)
}
/// `s.nstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&nsfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    true
}
/// `s.nsop(e,tp,eg,ep,ev,re,r,rp)`
fn nsop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if sc then
    if let Some(sc) = group::get_first(d, &g) {
        // Duel.Summon(tp,sc,true,nil)
        duel::summon(d, tp, sc, true, None, 0, None);
    }
}
/// `aux.AND(Card.IsMonster,Card.IsAbleToDeck)`
fn monster_deck(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(aux.AND(Card.IsMonster,Card.IsAbleToDeck),tp,LOCATION_HAND,0,1,nil) and Duel.IsPlayerCanDraw(tp)
        return duel::is_existing_matching_card(d, Some(&monster_deck), tp, LOCATION_HAND, 0, 1, NO_EX) && duel::is_player_can_draw(d, tp, 0);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local target_player=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let target_player = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER as u32]).remove(0).int() as u8;
    // local max_count=Duel.GetMatchingGroupCount(aux.AND(Card.IsMonster,Card.IsAbleToDeck),target_player,LOCATION_HAND,0,nil)
    let max_count = duel::get_matching_group_count(d, Some(&monster_deck), target_player, LOCATION_HAND, 0, NO_EX);
    // if max_count==0 then return end
    if max_count == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,target_player,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, target_player, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(target_player,aux.AND(Card.IsMonster,Card.IsAbleToDeck),target_player,LOCATION_HAND,0,1,max_count,nil)
    let g = duel::select_matching_card(d, target_player, Some(&monster_deck), target_player, LOCATION_HAND, 0, 1, max_count as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(1-target_player,g)
        duel::confirm_cards(d, 1 - target_player, g.clone());
        // local draw_count=Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        let draw_count = duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        // if draw_count>0 then
        if draw_count > 0 {
            // if draw_count>1 then Duel.SortDeckbottom(target_player,target_player,draw_count) end
            if draw_count > 1 {
                duel::sort_deckbottom(d, target_player, target_player, draw_count as u16);
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(target_player,draw_count,REASON_EFFECT)
            duel::draw(d, target_player, draw_count as u32, REASON_EFFECT);
        }
    }
}
