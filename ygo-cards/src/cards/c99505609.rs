//! Bingo Card
use super::prelude::*;

pub const ID: u32 = 99505609;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Choose 1 monster in a column with all zones occupied and destroy as many cards in that column as possible, then each player draws 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY, then activate 1 of these effects;
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.columnfilter(c)`: `c:GetColumnGroupCount()==(c:IsSequence(0,2,4) and 3 or 4)`
fn columnfilter(d: &mut Duel, c: CardId) -> bool {
    let want = if card::is_sequence(d, c, &[0, 2, 4]) { 3 } else { 4 };
    card::get_column_group_count(d, c, 0, 0) == want
}
/// `s.destg`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.columnfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     and Duel.IsPlayerCanDraw(tp,1) and Duel.IsPlayerCanDraw(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&columnfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_player_can_draw(d, a.tp, 1)
            && duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,4,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 4, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    true
}
/// `s.desop`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.columnfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil):GetFirst()
    let sg = duel::select_matching_card(d, a.tp, Some(&columnfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let sc = match group::get_first(d, &sg) {
        Some(sc) => sc,
        None => return,
    };
    // Duel.HintSelection(sc)
    duel::hint_selection(d, sc, true);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=sc:GetColumnGroup():AddCard(sc)
    let mut g = card::get_column_group(d, sc, 0, 0);
    group::add_card(&mut g, &CardSet::from_card(sc));
    // if c:IsRelateToEffect(e) then g:RemoveCard(c) end
    if card::is_relate_to_effect(d, c, a.e) {
        group::remove_card(&mut g, &CardSet::from_card(c));
    }
    // if Duel.Destroy(g,REASON_EFFECT)>0 then
    if duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        // local turn_player=Duel.GetTurnPlayer()
        let turn_player = duel::get_turn_player(d);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(turn_player,1,REASON_EFFECT)
        duel::draw(d, turn_player, 1, REASON_EFFECT);
        // Duel.Draw(1-turn_player,1,REASON_EFFECT)
        duel::draw(d, 1 - turn_player, 1, REASON_EFFECT);
    }
}
/// `s.efftg`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local own_mmzone=Duel.GetFieldGroup(tp,LOCATION_MMZONE,0)
    let own_mmzone = duel::get_field_group(d, a.tp, LOCATION_MMZONE, 0);
    // local own_stzone=Duel.GetFieldGroup(tp,LOCATION_STZONE,0)
    let own_stzone = duel::get_field_group(d, a.tp, LOCATION_STZONE, 0);
    // local b1=(#own_mmzone==5 or #own_stzone==5) and Duel.IsPlayerCanDraw(tp,2)
    let b1 = (own_mmzone.len() == 5 || own_stzone.len() == 5) && duel::is_player_can_draw(d, a.tp, 2);
    // local opp=1-tp
    let opp = 1 - a.tp;
    // local opp_mmzone=Duel.GetFieldGroup(opp,LOCATION_MMZONE,0)
    let opp_mmzone = duel::get_field_group(d, opp, LOCATION_MMZONE, 0);
    // local opp_stzone=Duel.GetFieldGroup(opp,LOCATION_STZONE,0)
    let opp_stzone = duel::get_field_group(d, opp, LOCATION_STZONE, 0);
    // local b2=(#opp_mmzone==5 or #opp_stzone==5) and Duel.IsPlayerCanDraw(opp,2)
    let b2 = (opp_mmzone.len() == 5 || opp_stzone.len() == 5) && duel::is_player_can_draw(d, opp, 2);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local cd=e:GetChainData()
    // cd.choice=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let choice = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, choice_value(choice));
    // if cd.choice==1 then
    if choice == Some(1) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,own_mmzone+own_stzone,5,tp,0)
        let mut g = own_mmzone.clone();
        group::add_card(&mut g, &own_stzone);
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 5, a.tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,opp_mmzone+opp_stzone,5,tp,0)
        let mut g = opp_mmzone.clone();
        group::add_card(&mut g, &opp_stzone);
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 5, a.tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,opp,2)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, opp, 2);
    }
    true
}
/// The Lua number stored in `cd.choice` (`nil` when the selection gave none).
fn choice_value(choice: Option<usize>) -> Value {
    match choice {
        Some(x) => Value::Int(x as i64),
        None => Value::Nil,
    }
}
/// `s.effop`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local player=e:GetChainData().choice==1 and tp or 1-tp
    let choice = chain::effect_data(d, a.e, None, chain::field::CARD);
    let player = if choice.int() == 1 { a.tp } else { 1 - a.tp };
    // local mmzone=Duel.GetFieldGroup(player,LOCATION_MMZONE,0)
    let mmzone = duel::get_field_group(d, player, LOCATION_MMZONE, 0);
    // local stzone=Duel.GetFieldGroup(player,LOCATION_STZONE,0)
    let stzone = duel::get_field_group(d, player, LOCATION_STZONE, 0);
    // local op=nil
    // local b1=#mmzone==5
    let b1 = mmzone.len() == 5;
    // local b2=#stzone==5
    let b2 = stzone.len() == 5;
    let op: Option<usize> = if b1 && b2 {
        // op=Duel.SelectEffect(tp,{true,aux.Stringid(id,4)},{true,aux.Stringid(id,5)})
        duel::select_effect(d, a.tp, &[(true, aux::stringid(ID, 4)), (true, aux::stringid(ID, 5))])
    } else {
        // op=(b1 and 1) or (b2 and 2) or nil
        if b1 {
            Some(1)
        } else if b2 {
            Some(2)
        } else {
            None
        }
    };
    // if op and Duel.Destroy(op==1 and mmzone or stzone,REASON_EFFECT)==5 and Duel.IsPlayerCanDraw(player) then
    if let Some(op) = op {
        let targets = if op == 1 { mmzone } else { stzone };
        if duel::destroy(d, targets, REASON_EFFECT, None, None) == 5 && duel::is_player_can_draw(d, player, 0) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(player,2,REASON_EFFECT)
            duel::draw(d, player, 2, REASON_EFFECT);
        }
    }
}
