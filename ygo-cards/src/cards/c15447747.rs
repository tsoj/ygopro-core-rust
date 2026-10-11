//! Prank-Kids Plan
use super::prelude::*;

pub const ID: u32 = 15447747;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PRANK_KIDS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // link
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e2:SetCondition(s.lkcon)
    effect::set_condition(d, e2, Some(condition(lkcon)));
    // e2:SetCost(s.lkcost)
    effect::set_cost(d, e2, Some(cost_target(lkcost)));
    // e2:SetTarget(s.lktg)
    effect::set_target(d, e2, Some(cost_target(lktg)));
    // e2:SetOperation(s.lkop)
    effect::set_operation(d, e2, Some(operation(lkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // shuffle
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TODECK+CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_TODECK + CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.lkcon(e,tp,eg,ep,ev,re,r,rp)
fn lkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
// function s.lkcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn lkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    true
}
// function s.lkfilter(c,mg)
fn lkfilter(d: &mut Duel, c: CardId, mg: &CardSet) -> bool {
    // return c:IsSetCard(SET_PRANK_KIDS) and c:IsLinkSummonable(nil,mg)
    card::is_set_card(d, c, &[SET_PRANK_KIDS as u16]) && card::is_link_summonable(d, c, None, Some(mg.clone()), 0, 0)
}
// function s.lktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn lktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local mg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_PRANK_KIDS),tp,LOCATION_MZONE,0,nil)
        let mg = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_PRANK_KIDS as u16]))), tp, LOCATION_MZONE, 0, NO_EX);
        // return Duel.IsExistingMatchingCard(s.lkfilter,tp,LOCATION_EXTRA,0,1,nil,mg)
        return duel::is_existing_matching_card(d, Some(&|d, c| lkfilter(d, c, &mg)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.lkop(e,tp,eg,ep,ev,re,r,rp)
fn lkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local mg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_PRANK_KIDS),tp,LOCATION_MZONE,0,nil)
    let mg = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_PRANK_KIDS as u16]))), tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local xg=Duel.SelectMatchingCard(tp,s.lkfilter,tp,LOCATION_EXTRA,0,1,1,nil,mg)
    let xg = duel::select_matching_card(d, tp, Some(&|d, c| lkfilter(d, c, &mg)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local tc=xg:GetFirst()
    let tc = group::get_first(d, &xg);
    // if tc then
    if let Some(tc) = tc {
        // Duel.LinkSummon(tp,tc,nil,mg)
        duel::link_summon(d, tp, tc, None, Some(mg), 0, 0);
    }
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp)
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.tp)
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PRANK_KIDS) and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_PRANK_KIDS as u16]) && card::is_able_to_deck(d, c)
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d).nil();
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local ct=math.min(#g,math.floor(tc:GetAttack()/100))
    let atk = card::get_attack(d, tc) as i64;
    let ct = (g.len() as i64).min(atk.div_euclid(100));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local sg=g:Select(tp,1,ct,nil)
    let sg = group::select(d, &g, tp, 1, ct as u16, false, NO_EX).nil();
    // if Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and tc:IsFaceup() and tc:IsRelateToBattle() then
    if duel::send_to_deck(d, &sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_faceup(d, tc)
        && card::is_relate_to_battle(d, tc)
    {
        // local oc=#(Duel.GetOperatedGroup())
        let oc = duel::get_operated_group(d).len() as i64;
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(oc*-100)
        effect::set_value(d, e1, oc * -100);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
