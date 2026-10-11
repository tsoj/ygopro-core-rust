//! Mythical Beast Medusa
use super::prelude::*;

pub const ID: u32 = 54965929;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // c:EnableCounterPermit(COUNTER_SPELL)
    // (a monster: the default location is LOCATION_MZONE)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, Some(LOCATION_MZONE), None);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetOperation(aux.chainreg)
    effect::set_operation(d, e2, Some(aux::chainreg()));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetOperation(s.acop)
    effect::set_operation(d, e3, Some(operation(acop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // to hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e4, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e4, TIMING_DAMAGE_STEP, None);
    // e4:SetCondition(s.atkcon)
    effect::set_condition(d, e4, Some(condition(atkcon)));
    // e4:SetCost(s.atkcost)
    effect::set_cost(d, e4, Some(cost_target(atkcost)));
    // e4:SetTarget(s.atktg)
    effect::set_target(d, e4, Some(cost_target(atktg)));
    // e4:SetOperation(s.atkop)
    effect::set_operation(d, e4, Some(operation(atkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_PZONE,0) == 1
    duel::get_field_group_count(d, a.tp, LOCATION_PZONE, 0) == 1
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanAddCounter(COUNTER_SPELL,1,false,LOCATION_MZONE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, LOCATION_MZONE)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return c:IsDestructable() and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return card::is_destructable(d, c, None)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, tc| spfilter(d, tc, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, tc| spfilter(d, tc, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.Destroy(c,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, c, e) && duel::destroy(d, c, REASON_EFFECT, None, None) != 0 {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)~=0
        //     and tc:IsCanAddCounter(COUNTER_SPELL,1) then
        if card::is_relate_to_effect(d, tc, e)
            && duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0
            && card::is_can_add_counter(d, tc, COUNTER_SPELL as u16, 1, false, 0)
        {
            // tc:AddCounter(COUNTER_SPELL,1)
            card::add_counter(d, tc, COUNTER_SPELL as u16, 1, false);
        }
    }
}
/// `s.acop(e,tp,eg,ep,ev,re,r,rp)`
fn acop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and e:GetHandler():GetFlagEffect(1)>0 then
    let re = a.re.nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) && card::get_flag_effect(d, c, 1) > 0 {
        // e:GetHandler():AddCounter(COUNTER_SPELL,1)
        card::add_counter(d, c, COUNTER_SPELL as u16, 1, false);
    }
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase() and aux.StatChangeDamageStepCondition()
    duel::is_battle_phase(d, None) && aux::stat_change_damage_step_condition(d)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,0,COUNTER_SPELL,2,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, tp, 1, 0, COUNTER_SPELL as u16, 2, REASON_COST);
    }
    // Duel.RemoveCounter(tp,1,0,COUNTER_SPELL,2,REASON_COST)
    duel::remove_counter(d, tp, 1, 0, COUNTER_SPELL as u16, 2, REASON_COST);
    true
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        let c = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(math.ceil(tc:GetAttack()/2))
        let atk = card::get_attack(d, tc);
        effect::set_value(d, e1, (atk as f64 / 2.0).ceil() as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // e2:SetValue(math.ceil(tc:GetDefense()/2))
        let def = card::get_defense(d, tc);
        effect::set_value(d, e2, (def as f64 / 2.0).ceil() as i64);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
