//! Kashtira Overlap
use super::prelude::*;

pub const ID: u32 = 8953369;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KASHTIRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 monster and increase ATK of the target by 1500
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate Effects of a target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.rmvfilter(c)
fn rmvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAttack(1500) and c:IsDefense(2100) and c:IsAbleToRemove()
    //   and (c:IsLocation(LOCATION_HAND) or c:IsFaceup())
    card::is_monster(d, c)
        && card::is_attack(d, c, &[1500])
        && card::is_defense(d, c, &[2100])
        && card::is_able_to_remove(d, c, None, None, None)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
}
// function s.tgfilter(c,tp)
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(s.rmvfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE,LOCATION_MZONE,1,c)
    card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&rmvfilter), tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // local tc=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,tp)
    let tc = duel::select_target(d, a.tp, Some(&|d, x| tgfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,tc,1,0,1500)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, tc, 1, 0, 1500);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,PLAYER_EITHER,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_EITHER as u8, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE) as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local exclude=tc and tc:IsRelateToEffect(e) or nil
    // In Lua this is `true` or nil, a boolean. Duel.GetMatchingGroup reads its exclusion with
    // lua_get<card*>/lua_get<group*>, which give nullptr for a boolean: nothing is excluded.
    let exclude = NO_EX;
    // local g=Duel.GetMatchingGroup(s.rmvfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE,LOCATION_MZONE,exclude)
    let g = duel::get_matching_group(d, Some(&rmvfilter), a.tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE, LOCATION_MZONE, exclude);
    // local rg=Group.CreateGroup()
    let mut rg = CardSet::new();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // rg=g:Select(tp,1,1,nil)
        rg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    }
    // elseif tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsAttack(1500) and tc:IsDefense(2100) and tc:IsAbleToRemove() then
    else {
        let tc_c = tc.nil();
        if card::is_relate_to_effect(d, tc_c, a.e)
            && card::is_faceup(d, tc_c)
            && card::is_attack(d, tc_c, &[1500])
            && card::is_defense(d, tc_c, &[2100])
            && card::is_able_to_remove(d, tc_c, None, None, None)
        {
            // rg=Group.FromCards(tc)
            rg = CardSet::from_card(tc_c);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
            // rg=rg:Select(tp,1,1,nil)
            rg = group::select(d, &rg, a.tp, 1, 1, false, NO_EX).nil();
        }
    }
    // if #rg==0 then return end
    if rg.is_empty() {
        return;
    }
    // if Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)>0 and rg:GetFirst():IsLocation(LOCATION_REMOVED) and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        let first = group::get_first(d, &rg).nil();
        if card::is_location(d, first, LOCATION_REMOVED) {
            let tc_c = tc.nil();
            if card::is_relate_to_effect(d, tc_c, a.e) && card::is_faceup(d, tc_c) {
                // Increase ATK
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_UPDATE_ATTACK)
                effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // e1:SetValue(1500)
                effect::set_value(d, e1, 1500);
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc_c, e1, false);
            }
        }
    }
}
// function s.disfilter(c)
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_EFFECT) and c:IsNegatableMonster()
    card::is_type(d, c, TYPE_EFFECT) && card::is_negatable_monster(d, c)
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.disfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && disfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_KASHTIRA),tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingTarget(s.disfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_KASHTIRA as u16]))), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    // local tc=Duel.SelectTarget(tp,s.disfilter,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
    let g = duel::select_target(d, a.tp, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsDisabled() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && !card::is_disabled(d, tc) {
        // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // Negate the effects of the target
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
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
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
