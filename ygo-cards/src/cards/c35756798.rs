//! Final Cross
use super::prelude::*;

pub const ID: u32 = 35756798;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WARRIOR, SET_SYNCHRON, SET_STARDUST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Grant to a Synchro Monster the ability to attack twice
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Check for Synchro Monsters sent to the GY
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_TO_GRAVE)
        effect::set_code(d, ge1, EVENT_TO_GRAVE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // for tc in eg:Iter() do
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        // if tc:IsType(TYPE_SYNCHRO) then
        if card::is_type(d, tc, TYPE_SYNCHRO) {
            // Duel.RegisterFlagEffect(tc:GetControler(),id,RESET_PHASE|PHASE_END,0,1)
            let p = card::get_controler(d, tc);
            duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(tp,id)>0 and Duel.IsTurnPlayer(tp) and (Duel.IsAbleToEnterBP() or Duel.IsBattlePhase())
    duel::get_flag_effect(d, a.tp, ID) > 0
        && duel::is_turn_player(d, a.tp)
        && (duel::is_able_to_enter_bp(d) || duel::is_battle_phase(d, None))
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_SYNCHRO) and c:CanAttack() and not c:IsHasEffect(EFFECT_EXTRA_ATTACK)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::can_attack(d, c)
        && card::is_has_effect(d, c, EFFECT_EXTRA_ATTACK, None).is_empty()
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tg=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let tg = duel::select_target(d, a.tp, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_ATKCHANGE,tg,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, tg, 1, a.tp, 0);
    true
}
/// `s.archetypetest(c)`
fn archetypetest(d: &mut Duel, c: CardId) -> bool {
    // return c:IsOriginalSetCard(SET_WARRIOR) or c:IsOriginalSetCard(SET_SYNCHRON) or c:IsOriginalSetCard(SET_STARDUST)
    card::is_original_set_card(d, c, &[SET_WARRIOR as u16])
        || card::is_original_set_card(d, c, &[SET_SYNCHRON as u16])
        || card::is_original_set_card(d, c, &[SET_STARDUST as u16])
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_SYNCHRO) and c:GetAttack()>0
    card::is_type(d, c, TYPE_SYNCHRO) && card::get_attack(d, c) > 0
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Can make a second attack
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3201)
        effect::set_description(d, e1, 3201);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // if not s.archetypetest(tc) then return end
        if !archetypetest(d, tc) {
            return;
        }
        // if Duel.IsExistingMatchingCard(s.atkfilter,tp,LOCATION_GRAVE,0,1,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if duel::is_existing_matching_card(d, Some(&atkfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SELECT as u64);
            // local atkc=Duel.SelectMatchingCard(tp,s.atkfilter,tp,LOCATION_GRAVE,0,1,1,nil):GetFirst()
            let g = duel::select_matching_card(d, a.tp, Some(&atkfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            let atkc = group::get_first(d, &g).nil();
            // Duel.HintSelection(atkc,true)
            duel::hint_selection(d, CardSet::from_card(atkc), true);
            // Gain ATK equal to the ATK of the Synchro monster
            // Duel.BreakEffect()
            duel::break_effect(d);
            // tc:UpdateAttack(atkc:GetAttack(),RESET_EVENT|RESETS_STANDARD,c)
            let atk = card::get_attack(d, atkc) as i64;
            card::update_attack(d, tc, atk, Some(RESET_EVENT | RESETS_STANDARD), Some(c), None);
        }
    }
}
