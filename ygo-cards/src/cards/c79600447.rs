//! Mementotlan Cranium Burst
use super::prelude::*;

pub const ID: u32 = 79600447;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_MEMENTOAL_TECUHTLICA], listed_series: &[SET_MEMENTO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetHintTiming(TIMING_ATTACK,TIMINGS_CHECK_MONSTER_E|TIMING_ATTACK)
    effect::set_hint_timing(d, e0, TIMING_ATTACK, Some(TIMINGS_CHECK_MONSTER_E | TIMING_ATTACK));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // --Opponent's monsters must attack your "Memento" monster with the highest ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e1, EFFECT_MUST_ATTACK);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetCondition(s.atcon)
    effect::set_condition(d, e1, Some(condition(atcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    effect::set_code(d, e2, EFFECT_MUST_ATTACK_MONSTER);
    // e2:SetValue(s.atlimit)
    effect::set_value_fn(d, e2, effect_card_fn(atlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Negate a monster effect activated in your opponent's field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE+CATEGORY_DISABLE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE + CATEGORY_DISABLE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.negcon)
    effect::set_condition(d, e3, Some(condition(negcon)));
    // e3:SetTarget(s.negtg)
    effect::set_target(d, e3, Some(cost_target(negtg)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atcon(e)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_MEMENTO),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_MEMENTO as u16]));
    duel::is_existing_matching_card(d, Some(&f), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let tp = effect::get_handler_player(d, e);
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_MEMENTO),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil)
    let f = aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_MEMENTO as u16]));
    let g = duel::get_matching_group(d, Some(&f), tp, LOCATION_MZONE, 0, NO_EX);
    // return g:GetMaxGroup(Card.GetAttack):IsContains(c)
    match group::get_max_group(d, &g, &|d, x| card::get_attack(d, x) as i64) {
        Some((mg, _)) => mg.contains(c),
        None => false,
    }
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int() as u32;
    // return re:IsMonsterEffect() and (loc&LOCATION_MZONE)~=0 and ep==1-tp and Duel.IsChainDisablable(ev)
    let re = a.re.nil();
    effect::is_monster_effect(d, re)
        && (loc & LOCATION_MZONE) != 0
        && a.ep == 1 - tp
        && duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsCode(CARD_MEMENTOAL_TECUHTLICA) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && card::is_code(d, chkc, &[CARD_MEMENTOAL_TECUHTLICA]);
    }
    // if chk==0 then return not Duel.HasFlagEffect(tp,id)
    //   and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsCode,CARD_MEMENTOAL_TECUHTLICA),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        let f = aux::faceup_filter(|d, x| card::is_code(d, x, &[CARD_MEMENTOAL_TECUHTLICA]));
        return !duel::has_flag_effect(d, tp, ID, None) && duel::is_existing_target(d, Some(&f), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsCode,CARD_MEMENTOAL_TECUHTLICA),tp,LOCATION_MZONE,0,1,1,nil)
    let f = aux::faceup_filter(|d, x| card::is_code(d, x, &[CARD_MEMENTOAL_TECUHTLICA]));
    duel::select_target(d, tp, Some(&f), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, tp, ID, RESET_CHAIN, EFFECT_FLAG_OATH, 1, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup()
    //   and tc:UpdateAttack(-1000,RESET_EVENT|RESETS_STANDARD,c)==-1000
    //   and tc:UpdateDefense(-1000,RESET_EVENT|RESETS_STANDARD,c)==-1000 then
    if card::is_relate_to_effect(d, tc, e)
        && card::is_faceup(d, tc)
        && card::update_attack(d, tc, -1000, Some(RESET_EVENT | RESETS_STANDARD), Some(c), None) == -1000
        && card::update_defense(d, tc, -1000, Some(RESET_EVENT | RESETS_STANDARD), Some(c), None) == -1000
    {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
