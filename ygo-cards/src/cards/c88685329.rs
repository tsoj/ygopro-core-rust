//! Dark Eye Nightmare
use super::prelude::*;

pub const ID: u32 = 88685329;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.drcost)
    effect::set_cost(d, e1, Some(cost_target(drcost)));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destruction replacement
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsAbleToRemoveAsCost()
    card::is_trap(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.drcost(e,tp,...,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1) && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local mct=2
    // if Duel.IsPlayerCanDraw(tp,2) then mct=3 end
    let mct: u16 = if duel::is_player_can_draw(d, tp, 2) { 3 } else { 2 };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_GRAVE,0,1,mct,nil)
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_GRAVE, 0, 1, mct, false, NO_EX).nil();
    // e:SetLabel(Duel.Remove(g,POS_FACEUP,REASON_COST))
    let n = duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    effect::set_label(d, a.e, &[n as i64]);
    true
}
/// `s.drtg(e,tp,...,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // if e:GetLabel()==3 then
    if effect::get_label(d, a.e) == 3 {
        // e:SetCategory(CATEGORY_DRAW+CATEGORY_HANDES)
        effect::set_category(d, a.e, CATEGORY_DRAW + CATEGORY_HANDES);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
        // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
        // e:SetCategory(CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DRAW);
    }
    true
}
/// `s.drop(e,tp,...)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local lbl=e:GetLabel()
    let lbl = effect::get_label(d, a.e);
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local d=1
    // if lbl==3 then d=2 end
    let dn: u32 = if lbl == 3 { 2 } else { 1 };
    // if Duel.Draw(p,d,REASON_EFFECT)==d and lbl~=2 then
    if duel::draw(d, p, dn, REASON_EFFECT) == dn as i32 && lbl != 2 {
        // Duel.ShuffleHand(p)
        duel::shuffle_hand(d, p);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if lbl==1 then
        if lbl == 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,1,nil)
            let g = duel::select_matching_card(d, a.tp, Some(&card::is_able_to_deck), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT)
                duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
            }
        } else if lbl == 3 {
            // Duel.DiscardHand(p,nil,1,1,REASON_EFFECT|REASON_DISCARD)
            duel::discard_hand(d, p, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
        }
    }
}
/// `s.reptg(e,tp,...,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsReason(REASON_BATTLE) and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_reason(d, h, REASON_BATTLE) && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT|REASON_REPLACE)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
        // return true
        return true;
    }
    // else return false end
    false
}
