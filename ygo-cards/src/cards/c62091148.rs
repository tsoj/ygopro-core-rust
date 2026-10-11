//! Salamandra with Chain
//! scripted by Naim
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 62091148;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip this card to a FIRE monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP+CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_EQUIP + CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(aux.RemainFieldCost)
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    // e1:SetTarget(s.eqptg)
    effect::set_target(d, e1, Some(cost_target(eqptg)));
    // e1:SetOperation(s.eqpop)
    effect::set_operation(d, e1, Some(operation(eqpop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // Fusion Summon 1 FIRE Warrior or Dragon Fusion monster shuffling materials from your hand, field, and/or GY
    // local params = {fusfilter=s.fusmonfilter, matfilter=Card.IsAbleToDeck, extrafil=s.extramat,
    //   extraop=Fusion.ShuffleMaterial, extratg=s.extratg, stage2=s.stage2}
    let params = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| fusmonfilter(d, c))),
        matfilter: Some(Arc::new(|d, c, _e, _tp, _chk| card::is_able_to_deck(d, c))),
        extrafil: Some(Arc::new(|d, e, tp, _mg| (Some(extramat(d, e, tp)), None))),
        extraop: Some(fusion::shuffle_material()),
        extratg: Some(cost_target(extratg)),
        stage2: Some(Arc::new(|d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, sg: Option<&CardSet>, chk: i64| stage2(d, e, tc, tp, sg, chk))),
        ..Default::default()
    };
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(Fusion.SummonEffTG(params))
    let tg = fusion::summon_eff_tg(d, params.clone());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(params))
    let op = fusion::summon_eff_op(d, params);
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.eqptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn eqptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsAttribute(ATTRIBUTE_FIRE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && card::is_attribute(d, chkc, ATTRIBUTE_FIRE);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_FIRE),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&aux::faceup_filter(|d, c| card::is_attribute(d, c, ATTRIBUTE_FIRE))), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_FIRE),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&aux::faceup_filter(|d, c| card::is_attribute(d, c, ATTRIBUTE_FIRE))), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_POSITION,nil,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, None, 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
// function s.posfilter(c)
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and c:IsCanTurnSet()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && card::is_can_turn_set(d, c)
}
// function s.eqpop(e,tp,eg,ep,ev,re,r,rp)
fn eqpop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsStatus(STATUS_LEAVE_CONFIRMED) then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::get_first_target(d).first().copied().nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.Equip(tp,c,tc) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && duel::equip(d, tp, c, tc, true, false) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(700)
        effect::set_value(d, e1, 700);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
        // e2:SetValue(function(e,cc) return cc:IsAttribute(ATTRIBUTE_FIRE) end)
        effect::set_value_fn(d, e2, effect_card_fn(equip_limit));
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // Change 1 Effect Monster on the field to face-down Defense Position
        // local pg=Duel.GetMatchingGroup(s.posfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
        let pg = duel::get_matching_group(d, Some(&posfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // if #pg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if pg.len() > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
            // local tc=pg:Select(tp,1,1,nil)
            let tc2 = group::select(d, &pg, tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(tc,true)
            duel::hint_selection(d, tc2.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ChangePosition(tc,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, tc2, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    } else {
        // c:CancelToGrave(false)
        card::cancel_to_grave(d, c, false);
    }
}
// function s.equip_limit (value function of e2): cc:IsAttribute(ATTRIBUTE_FIRE)
fn equip_limit(d: &mut Duel, _e: EffectId, cc: CardId) -> bool {
    card::is_attribute(d, cc, ATTRIBUTE_FIRE)
}
// function s.fusmonfilter(c)
fn fusmonfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsRace(RACE_WARRIOR|RACE_DRAGON)
    card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_race(d, c, RACE_WARRIOR | RACE_DRAGON)
}
// function s.extramat(e,tp,mg,sumtype)
fn extramat(d: &mut Duel, _e: EffectId, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(aux.NecroValleyFilter(Fusion.IsMonsterFilter(Card.IsAbleToDeck)),tp,LOCATION_GRAVE,0,nil)
    duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| card::is_monster(d, c) && card::is_able_to_deck(d, c))), tp, LOCATION_GRAVE, 0, NO_EX)
}
// function s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,0,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, a.tp, (LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE) as i32);
    true
}
// function s.stage2(e,tc,tp,sg,chk)
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==0 then
    if chk == 0 {
        // aux.DelayedOperation(tc,PHASE_END,id,e,tp,function(cc) Duel.Destroy(cc,REASON_EFFECT) end,nil,0,1,aux.Stringid(id,3))
        // Group.FromCards(nil) is empty: aux.DelayedOperation returns at once.
        if let Some(tc) = tc {
            aux::delayed_operation(
                d,
                tc,
                PHASE_END,
                ID,
                e,
                tp,
                Some(std::sync::Arc::new(destroy_oper)),
                None,
                Some(0),
                Some(1),
                Some(aux::stringid(ID, 3)),
                None,
            );
        }
    }
}
// function(cc) Duel.Destroy(cc,REASON_EFFECT) end
fn destroy_oper(d: &mut Duel, cc: &CardSet, _a: &EffArgs) -> bool {
    duel::destroy(d, cc.clone(), REASON_EFFECT, None, None);
    true
}
