//! True Draco Apocalypse
use super::prelude::*;

pub const ID: u32 = 61529473;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRUE_DRACO_KING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Destroy and halve ATK/DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP as u32, Some((TIMING_DAMAGE_STEP | TIMING_END_PHASE) as u32));
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(lua_fn(stat_change_cond)));
    // e1:SetCost(s.opccost)
    effect::set_cost(d, e1, Some(cost_target(opccost)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Tribute Summon 1 "True Draco" or "True King" monster face-up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e2, CATEGORY_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END as u32));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) and Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(lua_fn(sum_cond)));
    // e2:SetCost(s.opccost)
    effect::set_cost(d, e2, Some(cost_target(opccost)));
    // e2:SetTarget(s.sumtg)
    effect::set_target(d, e2, Some(cost_target(sumtg)));
    // e2:SetOperation(s.sumop)
    effect::set_operation(d, e2, Some(operation(sumop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 monster on the field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_SZONE) end)
    effect::set_condition(d, e3, Some(lua_fn(prev_szone_cond)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `aux.StatChangeDamageStepCondition()`
fn stat_change_cond(d: &mut Duel, _a: Args) -> bool {
    aux::stat_change_damage_step_condition(d)
}
/// `function(_,tp) return Duel.IsTurnPlayer(1-tp) and Duel.IsMainPhase() end`
fn sum_cond(d: &mut Duel, a: Args) -> bool {
    duel::is_turn_player(d, 1 - a.p(1)) && duel::is_main_phase(d, None)
}
/// `function(e) return e:GetHandler():IsPreviousLocation(LOCATION_SZONE) end`
fn prev_szone_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_previous_location(d, h, LOCATION_SZONE)
}
/// `s.opccost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn opccost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_CHAIN as u32, 0, 1, 0);
    true
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and chkc~=c and chkc:IsFaceup() and chkc:IsSetCard(SET_TRUE_DRACO_KING) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc)
            && card::is_controler(d, chkc, a.tp)
            && chkc != c
            && card::is_faceup(d, chkc)
            && card::is_set_card(d, chkc, &[SET_TRUE_DRACO_KING as u16]);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSetCard,SET_TRUE_DRACO_KING),tp,LOCATION_ONFIELD,0,1,c)
    //     and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&aux::faceup_filter(|d, cc| card::is_set_card(d, cc, &[SET_TRUE_DRACO_KING as u16]))),
            a.tp,
            LOCATION_ONFIELD,
            0,
            1,
            c.into(),
        ) && duel::is_existing_matching_card(d, Some(&|d, cc| card::is_faceup(d, cc)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsSetCard,SET_TRUE_DRACO_KING),tp,LOCATION_ONFIELD,0,1,1,c)
    let g = duel::select_target(
        d,
        a.tp,
        Some(&aux::faceup_filter(|d, cc| card::is_set_card(d, cc, &[SET_TRUE_DRACO_KING as u16]))),
        a.tp,
        LOCATION_ONFIELD,
        0,
        1,
        1,
        false,
        c.into(),
    )
    .nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
        // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&|d, cc| card::is_faceup(d, cc)), a.tp, 0, LOCATION_MZONE, NO_EX);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // for oc in g:Iter() do
        for oc in group::iter(d, &g) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(math.ceil(oc:GetAttack()/2))
            let atk = card::get_attack(d, oc);
            effect::set_value(d, e1, (atk as f64 / 2.0).ceil() as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // oc:RegisterEffect(e1)
            card::register_effect(d, oc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
            // e2:SetValue(math.ceil(oc:GetDefense()/2))
            let def = card::get_defense(d, oc);
            effect::set_value(d, e2, (def as f64 / 2.0).ceil() as i64);
            // oc:RegisterEffect(e2)
            card::register_effect(d, oc, e2, false);
        }
    }
}
/// `s.sumfilter(c)`
fn sumfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TRUE_DRACO_KING) and c:IsSummonable(true,nil,1)
    card::is_set_card(d, c, &[SET_TRUE_DRACO_KING as u16]) && card::is_summonable(d, c, true, None, 1, None)
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.sumfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&sumfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp)`
fn sumop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.sumfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&sumfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &g) {
        // Duel.Summon(tp,tc,true,nil,1)
        duel::summon(d, a.tp, tc, true, None, 1, None);
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
