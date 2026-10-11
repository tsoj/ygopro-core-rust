//! World Legacy Pawns
//! 星遺物の傀儡
use super::prelude::*;

pub const ID: u32 = 89320376;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KRAWLER], ..ScriptInfo::DEFAULT };

/// `e` with `chk` set and no chain-target card (the Lua calls `s.cost(e,tp,eg,ep,ev,re,r,rp,0)`).
fn with_chk(a: &EffArgs, chk: i64) -> EffArgs {
    EffArgs { chk: Some(chk), extra: Vec::new(), ..a.clone() }
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetHintTiming(TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_END_PHASE, None);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // pos (face-down)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_POSITION);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, TIMING_END_PHASE, None);
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.postg1)
    effect::set_target(d, e2, Some(cost_target(postg1)));
    // e2:SetOperation(s.posop1)
    effect::set_operation(d, e2, Some(operation(posop1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // pos (face-up)
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_POSITION + CATEGORY_SET);
    // e3:SetCost(s.poscost)
    effect::set_cost(d, e3, Some(cost_target(poscost)));
    // e3:SetTarget(s.postg2)
    effect::set_target(d, e3, Some(cost_target(postg2)));
    // e3:SetOperation(s.posop2)
    effect::set_operation(d, e3, Some(operation(posop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then
    if let Some(_chkc) = a.chkc() {
        // if e:GetLabel()==0 then
        if effect::get_label(d, a.e) == 0 {
            // return s.postg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
            return postg1(d, a);
        } else {
            // return s.postg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
            return postg2(d, a);
        }
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local b1=s.cost(e,tp,eg,ep,ev,re,r,rp,0) and s.postg1(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = cost_fn(d, &with_chk(a, 0)) && postg1(d, &with_chk(a, 0));
    // local b2=s.poscost(e,tp,eg,ep,ev,re,r,rp,0) and s.postg2(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = poscost(d, &with_chk(a, 0)) && postg2(d, &with_chk(a, 0));
    // if (b1 or b2) and Duel.SelectYesNo(tp,94) then
    if (b1 || b2) && duel::select_yes_no(d, tp, 94) {
        // local op=0
        let op: i64;
        // if b1 and b2 then
        if b1 && b2 {
            // op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
            op = duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]) as i64;
        } else if b1 {
            // op=Duel.SelectOption(tp,aux.Stringid(id,0))
            op = duel::select_option(d, tp, true, &[aux::stringid(ID, 0)]) as i64;
        } else {
            // op=Duel.SelectOption(tp,aux.Stringid(id,1))+1
            op = duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]) as i64 + 1;
        }
        // e:SetLabel(op)
        effect::set_label(d, a.e, &[op]);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // if op==0 then
        if op == 0 {
            // s.cost(e,tp,eg,ep,ev,re,r,rp,1)
            cost_fn(d, &with_chk(a, 1));
            // s.postg1(e,tp,eg,ep,ev,re,r,rp,1)
            postg1(d, &with_chk(a, 1));
            // e:SetOperation(s.posop1)
            effect::set_operation(d, a.e, Some(operation(posop1)));
        } else {
            // s.poscost(e,tp,eg,ep,ev,re,r,rp,1)
            poscost(d, &with_chk(a, 1));
            // s.postg2(e,tp,eg,ep,ev,re,r,rp,1)
            postg2(d, &with_chk(a, 1));
            // e:SetOperation(s.posop2)
            effect::set_operation(d, a.e, Some(operation(posop2)));
        }
    } else {
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // e:SetOperation(nil)
        effect::set_operation(d, a.e, None);
    }
    true
}
/// `s.postg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn postg1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFacedown() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && card::is_facedown(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFacedown,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFacedown,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, 0, 0);
    true
}
/// `s.posop1(e,tp,eg,ep,ev,re,r,rp)`
fn posop1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFacedown() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_facedown(d, tc) {
            // local pos1=0
            let mut pos1: u32 = 0;
            // if not tc:IsPosition(POS_FACEUP_ATTACK) then pos1=pos1+POS_FACEUP_ATTACK end
            if !card::is_position(d, tc, POS_FACEUP_ATTACK) {
                pos1 += POS_FACEUP_ATTACK;
            }
            // if not tc:IsPosition(POS_FACEUP_DEFENSE) then pos1=pos1+POS_FACEUP_DEFENSE end
            if !card::is_position(d, tc, POS_FACEUP_DEFENSE) {
                pos1 += POS_FACEUP_DEFENSE;
            }
            // local pos2=Duel.SelectPosition(tp,tc,pos1)
            let pos2 = duel::select_position(d, tp, tc, pos1 as u8);
            // Duel.ChangePosition(tc,pos2)
            duel::change_position(d, tc, pos2 as u8, None, None, None, false);
        }
    }
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_KRAWLER) and c:IsMonster() and c:IsAbleToDeckAsCost()
    card::is_set_card(d, c, &[SET_KRAWLER as u16]) && card::is_monster(d, c) && card::is_able_to_deck_as_cost(d, c)
}
/// `s.poscost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn poscost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return s.cost(e,tp,eg,ep,ev,re,r,rp,0) and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return cost_fn(d, &with_chk(a, 0)) && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    // s.cost(e,tp,eg,ep,ev,re,r,rp,1)
    cost_fn(d, &with_chk(a, 1));
    true
}
/// `s.posfilter(c)`
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCanTurnSet()
    card::is_faceup(d, c) && card::is_can_turn_set(d, c)
}
/// `s.postg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn postg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.posfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && posfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.posfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&posfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectTarget(tp,s.posfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&posfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop2(e,tp,eg,ep,ev,re,r,rp)`
fn posop2(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // Duel.ChangePosition(tc,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
