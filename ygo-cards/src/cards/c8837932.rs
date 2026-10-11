//! Cubic Mandala
use super::prelude::*;

pub const ID: u32 = 8837932;

const COUNTER_CUBIC: u16 = 0x1038;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x1038], listed_series: &[SET_CUBIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_COUNTER);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_END_PHASE,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_END_PHASE, Some(TIMING_END_PHASE));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_CHAIN_ACTIVATING)
    effect::set_code(d, e2, EVENT_CHAIN_ACTIVATING);
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_CUBIC),tp,LOCATION_MZONE,0,1,nil)
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CUBIC as u16]));
    duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,tid)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tid: i32) -> bool {
    // return c:IsReason(REASON_DESTROY) and c:IsMonster() and c:GetTurnID()==tid
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    //     and Duel.IsCanAddCounter(tp,COUNTER_CUBIC,1,c)
    card::is_reason(d, c, REASON_DESTROY)
        && card::is_monster(d, c)
        && card::get_turn_id(d, c) as i32 == tid
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)
        && duel::is_can_add_counter(d, tp, Some((COUNTER_CUBIC, 1, c)))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local tid=Duel.GetTurnCount()
    let tid = duel::get_turn_count(d, None);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(1-tp) and s.spfilter(chkc,e,tp,tid) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - tp) && spfilter(d, chkc, e, tp, tid);
    }
    // local ft=Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)
    let mut ft = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0;
    // if chk==0 then return ft>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,0,LOCATION_GRAVE,1,nil,e,tp,tid) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, e, tp, tid)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,0,LOCATION_GRAVE,1,ft,nil,e,tp,tid)
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, e, tp, tid)), tp, 0, LOCATION_GRAVE, 1, ft as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ft=Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)
    let ft = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0;
    // if ft<=0 or not c:IsRelateToEffect(e) then return end
    if ft <= 0 || !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local sg=Duel.GetTargetCards(e)
    let mut sg = duel::get_target_cards(d, e).nil();
    // if #sg>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if sg.len() > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if #sg>ft then
    if sg.len() as i32 > ft {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // sg=sg:Select(tp,ft,ft,nil)
        sg = group::select(d, &sg, tp, ft as u16, ft as u16, false, NO_EX).nil();
    }
    // for sc in sg:Iter() do
    for sc in group::iter(d, &sg) {
        // if Duel.SpecialSummonStep(sc,0,tp,1-tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, sc, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None) {
            // c:SetCardTarget(sc)
            card::set_card_target(d, c, sc);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(0)
            effect::set_value(d, e1, 0);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // for oc in og:Iter() do
    for oc in group::iter(d, &og) {
        // oc:AddCounter(COUNTER_CUBIC,1)
        card::add_counter(d, oc, COUNTER_CUBIC, 1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
        // e2:SetCondition(function(e) return e:GetHandler():GetCounter(COUNTER_CUBIC)>0 end)
        effect::set_condition(d, e2, Some(condition(has_counter)));
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // oc:RegisterEffect(e2)
        card::register_effect(d, oc, e2, false);
        // local e3=e2:Clone()
        let e3 = effect::clone(d, e2);
        // e3:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e3, EFFECT_DISABLE);
        // oc:RegisterEffect(e3)
        card::register_effect(d, oc, e3, false);
    }
}
/// `function(e) return e:GetHandler():GetCounter(COUNTER_CUBIC)>0 end`
fn has_counter(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, COUNTER_CUBIC) > 0
}
/// `s.dfilter(c,g)`
fn dfilter(_d: &mut Duel, c: CardId, g: &CardSet) -> bool {
    // return g:IsContains(c)
    g.contains(c)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=e:GetHandler():GetCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_card_target(d, h);
    // if re:IsMonsterEffect() and rp==1-tp
    //     and Duel.IsExistingMatchingCard(s.dfilter,tp,0,LOCATION_MZONE,1,nil,g) then
    let re = a.re.nil();
    if effect::is_monster_effect(d, re)
        && a.rp == 1 - tp
        && duel::is_existing_matching_card(d, Some(&|d, c| dfilter(d, c, &g)), tp, 0, LOCATION_MZONE, 1, NO_EX)
    {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=e:GetHandler():GetCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_card_target(d, h);
    // return eg:FilterCount(s.dfilter,nil,g)>0
    //     and not Duel.IsExistingMatchingCard(s.dfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil,g)
    let eg = a.eg.clone().nil();
    group::filter_count(d, &eg, &|d, c| dfilter(d, c, &g), NO_EX) > 0
        && !duel::is_existing_matching_card(d, Some(&|d, c| dfilter(d, c, &g)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
