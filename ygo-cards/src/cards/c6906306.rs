//! Rubysapphirus, the Adamant Jewel
use super::prelude::*;

pub const ID: u32 = 6906306;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Xyz Summon procedure: 2+ Level 9 monsters
    // Xyz.AddProcedure(c,nil,9,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(9), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // --If this card would be destroyed, you can detach 1 material from this card instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.reptg)
    effect::set_target(d, e1, Some(cost_target(reptg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Double this card's ATK until the end of the Damage Step
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Negate opponent's effects that targets the card targetted by this effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e3, CATEGORY_DISABLE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e3:SetCondition(s.negcon)
    effect::set_condition(d, e3, Some(condition(negcon)));
    // e3:SetTarget(s.negtg)
    effect::set_target(d, e3, Some(cost_target(negtg)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && card::check_remove_overlay_card(d, c, tp, 1, REASON_EFFECT);
    }
    // return Duel.SelectEffectYesNo(tp,c,96) and c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)>0
    duel::select_effect_yes_no(d, tp, c, Some(96)) && card::remove_overlay_card(d, c, tp, 1, 1, REASON_EFFECT) > 0
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:GetOverlayGroup():IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_FIRE) then return false end
    let og = card::get_overlay_group(d, c);
    if !group::is_exists(d, &og, &|d, x| card::is_attribute(d, x, ATTRIBUTE_FIRE as u32), 1, NO_EX) {
        return false;
    }
    // local bc=Duel.GetAttackTarget()
    // return bc and bc:IsFaceup() and bc:GetAttack()>c:GetAttack()
    match duel::get_attack_target(d) {
        Some(bc) => card::is_faceup(d, bc) && card::get_attack(d, bc) > card::get_attack(d, c),
        None => false,
    }
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToBattle() then
    if card::is_faceup(d, c) && card::is_relate_to_battle(d, c) {
        // --Double this card's ATK
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(c:GetAttack()*2)
        let atk = card::get_attack(d, c);
        effect::set_value(d, e1, (atk * 2) as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and e:GetHandler():GetOverlayGroup():IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_WATER)
    if !duel::is_turn_player(d, 1 - a.tp) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, h);
    group::is_exists(d, &og, &|d, x| card::is_attribute(d, x, ATTRIBUTE_WATER as u32), 1, NO_EX)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,0,1,1,nil)
    duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
        // --Negate opponent's activated effects that target this card
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAIN_SOLVING)
        effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
        // e1:SetCondition(s.discon)
        effect::set_condition(d, e1, Some(condition(discon)));
        // e1:SetOperation(s.disop)
        effect::set_operation(d, e1, Some(operation(disop)));
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not (rp==1-tp and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET)) then return false end
    if !(a.rp == 1 - a.tp && effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0)) {
        return false;
    }
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    // return tc and tg and tg:IsContains(tc) and tc:HasFlagEffect(id)
    match (tc, tg) {
        (Some(tc), Some(tg)) => tg.contains(tc) && card::has_flag_effect(d, tc, ID, None),
        _ => false,
    }
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
