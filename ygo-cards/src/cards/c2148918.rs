//! Armor Ninjitsu Art of Rust Mist
use super::prelude::*;

pub const ID: u32 = 2148918;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NINJA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetHintTiming(TIMING_SPSUMMON)
    effect::set_hint_timing(d, e1, TIMING_SPSUMMON, None);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk change
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon_cond)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: Option<EffectId>, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and (not e or c:IsRelateToEffect(e))
    if !(card::is_faceup(d, c) && card::is_controler(d, c, tp)) {
        return false;
    }
    match e {
        None => true,
        Some(e) => card::is_relate_to_effect(d, c, e),
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, tp: u8, eg: &CardSet) -> bool {
    // return eg:IsExists(s.tgfilter,1,nil,nil,1-tp) and
    //     Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_NINJA),tp,LOCATION_MZONE,0,1,nil)
    group::is_exists(d, eg, &|d, c| tgfilter(d, c, None, 1 - tp), 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NINJA as u16])), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// The condition of the trigger `e2` (the event group is its `eg`).
fn spcon_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    spcon(d, a.tp, &eg)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)` with the group `eg`: `chk` 0 only checks, otherwise
/// the target is `eg`
fn sptg_eg(d: &mut Duel, eg: &CardSet, chk: i64) -> bool {
    // if chk==0 then return true end
    if chk == 0 {
        return true;
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    true
}
/// `s.sptg` as the target of the trigger `e2`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let eg = a.eg.clone().nil();
    sptg_eg(d, &eg, 1)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)` with the group `eg`
fn spop(d: &mut Duel, e: EffectId, tp: u8, eg: &CardSet) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local g=eg:Filter(s.tgfilter,nil,e,1-tp)
    let g = group::filter(d, eg, &|d, c| tgfilter(d, c, Some(e), 1 - tp), NO_EX);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(tc:GetAttack()/2)
        let atk = card::get_attack(d, tc);
        effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.spop` as the operation of the trigger `e2`
fn spop_op(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    spop(d, a.e, a.tp, &eg);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`: the activation chooses the effect of the Spell
/// for the last Special Summon
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local res,teg,tep,tev,tre,tr,trp=Duel.CheckEvent(EVENT_SPSUMMON_SUCCESS,true)
    let found = duel::check_event(d, EVENT_SPSUMMON_SUCCESS);
    let teg = found.as_ref().map(|ev| ev.event_cards.clone().nil());
    // if res and s.spcon(e,tp,teg,tep,tev,tre,tr,trp) and s.sptg(e,tp,teg,tep,tev,tre,tr,trp,0)
    //     and Duel.SelectYesNo(tp,94) then
    let ok = match &teg {
        Some(g) => spcon(d, a.tp, g) && sptg_eg(d, g, 0) && duel::select_yes_no(d, a.tp, 94),
        None => false,
    };
    if let (true, Some(teg)) = (ok, teg) {
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE);
        // e:SetOperation(s.spop2(e,tp,teg,tep,tev,tre,tr,trp))
        let (e, tp) = (a.e, a.tp);
        let op_eg = teg.clone();
        effect::set_operation(d, a.e, Some(operation(move |d, _a| spop(d, e, tp, &op_eg))));
        // s.sptg(e,tp,teg,tep,tev,tre,tr,trp,1)
        sptg_eg(d, &teg, 1);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetOperation(nil)
        effect::set_operation(d, a.e, None);
    }
    true
}
