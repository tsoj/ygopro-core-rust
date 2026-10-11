//! Digital Bug Scaradiator
use super::prelude::*;

pub const ID: u32 = 12615446;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,s.matfilter,3,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_INSECT, sc, st, Some(tp)) && card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, sc, st, Some(tp)))),
        Some(3),
        2,
        None,
        None,
        Some(xyz::INFINITE_MATS),
        None,
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Position+Negate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_DISABLE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(2))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(2), None, None)));
    // e1:SetTarget(s.postg)
    effect::set_target(d, e1, Some(cost_target(postg)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.xyzcon)
    effect::set_condition(d, e2, Some(condition(xyzcon)));
    // e2:SetTarget(s.xyztg)
    effect::set_target(d, e2, Some(cost_target(xyztg)));
    // e2:SetOperation(s.xyzop)
    effect::set_operation(d, e2, Some(operation(xyzop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `Card.IsCanChangePosition` as a filter (the Lua passes the C function)
fn can_change_position(d: &mut Duel, c: CardId) -> bool {
    card::is_can_change_position(d, c)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsCanChangePosition() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_can_change_position(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsCanChangePosition,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&can_change_position), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,Card.IsCanChangePosition,tp,0,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&can_change_position), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.ChangePosition(tc,POS_FACEUP_DEFENSE,POS_FACEDOWN_DEFENSE,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)>0
    //     and ((tc:IsFaceup() and not tc:IsDisabled()) or tc:IsType(TYPE_TRAPMONSTER)) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::change_position(
            d,
            CardSet::from_card(tc),
            POS_FACEUP_DEFENSE as u8,
            Some(POS_FACEDOWN_DEFENSE as u8),
            Some(POS_FACEUP_ATTACK as u8),
            Some(POS_FACEUP_ATTACK as u8),
            false,
        ) > 0
        && ((card::is_faceup(d, tc) && !card::is_disabled(d, tc)) || card::is_type(d, tc, TYPE_TRAPMONSTER))
    {
        // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // if tc:IsType(TYPE_TRAPMONSTER) then
        if card::is_type(d, tc, TYPE_TRAPMONSTER) {
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
            effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
            // e3:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc, e3, false);
        }
    }
}
/// `s.xyzcon(e,tp,eg,ep,ev,re,r,rp)`
fn xyzcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=c:GetBattleTarget()
    let tc = card::get_battle_target(d, c);
    // if not c:IsRelateToBattle() or c:IsFacedown() then return false end
    if !card::is_relate_to_battle(d, c) || card::is_facedown(d, c) {
        return false;
    }
    // e:SetLabelObject(tc)
    let label = match tc {
        Some(t) => Value::Card(t),
        None => Value::Nil,
    };
    effect::set_label_object(d, a.e, label);
    // return tc:IsLocation(LOCATION_GRAVE) and tc:IsMonster() and tc:IsReason(REASON_BATTLE)
    let tc = tc.nil();
    card::is_location(d, tc, LOCATION_GRAVE) && card::is_monster(d, tc) && card::is_reason(d, tc, REASON_BATTLE)
}
/// `s.xyztg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn xyztg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsType(TYPE_XYZ) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_type(d, h, TYPE_XYZ);
    }
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(tc)), 1, 0, 0);
    true
}
/// `s.xyzop(e,tp,eg,ep,ev,re,r,rp)`
fn xyzop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // Duel.Overlay(c,tc)
        duel::overlay(d, c, CardSet::from_card(tc.nil()), false);
    }
}
