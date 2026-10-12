//! Skyborne Blue Cularsaw
//! JP name
use super::prelude::*;

pub const ID: u32 = 101402087;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,s.lv8matfilter,aux.FilterBoolFunctionEx(Card.IsRace,RACE_WARRIOR))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| lv8matfilter(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_WARRIOR, Some(fc), st, Some(tp)))),
        ],
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP+CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_EQUIP + CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_fusion_summoned(d, h)
    })));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,nil,s.equipop,e1)
    // (equipval is nil in the Lua: calling it raises "attempt to call a nil value")
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|_d, _a: Args| -> Value { crate::lua_error("attempt to call a nil value") }),
        lua_fn(|d, a: Args| {
            equipop(d, a.c(0), a.e(1), a.p(2), a.c(3));
            Value::Nil
        }),
        e1,
        None,
        None,
        None,
    );
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e2, CATEGORY_NEGATE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_condition(d, e2, Some(condition(negate_condition)));
    // e2:SetCost(s.negcost)
    effect::set_cost(d, e2, Some(cost_target(negcost)));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e2, Some(cost_target(|d, a| {
        // if chk==0 then return true end
        if a.chk == Some(0) {
            return true;
        }
        // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
        true
    })));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateActivation(ev) end)
    effect::set_operation(d, e2, Some(operation(|d, a| {
        duel::negate_activation(d, a.ev as u8);
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// local LOCATIONS_GRAVE_REMOVED=LOCATION_GRAVE|LOCATION_REMOVED
const LOCATIONS_GRAVE_REMOVED: u32 = LOCATION_GRAVE | LOCATION_REMOVED;
// s.lv8matfilter(c,fc,sumtype,tp)
fn lv8matfilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsLevelAbove(8) and c:IsAttribute(ATTRIBUTE_DARK,fc,sumtype,tp)
    card::is_level_above(d, c, 8) && card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, Some(fc), sumtype, Some(tp))
}
// s.eqfilter(c,opp,tp)
fn eqfilter(d: &mut Duel, c: CardId, opp: u8, tp: u8) -> bool {
    // return (c:IsRace(RACE_WARRIOR) or (c:IsMonster() and c:IsControler(opp))) and c:IsFaceup()
    //     and c:CheckUniqueOnField(tp) and not c:IsForbidden()
    (card::is_race(d, c, RACE_WARRIOR) || (card::is_monster(d, c) && card::is_controler(d, c, opp)))
        && card::is_faceup(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
        && !card::is_forbidden(d, c)
}
// s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATIONS_GRAVE_REMOVED,LOCATIONS_GRAVE_REMOVED,1,nil,1-tp,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| eqfilter(d, c, 1 - tp, tp)), tp, LOCATIONS_GRAVE_REMOVED, LOCATIONS_GRAVE_REMOVED, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,PLAYER_EITHER,LOCATIONS_GRAVE_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, PLAYER_EITHER as u8, LOCATIONS_GRAVE_REMOVED as i32);
    true
}
// s.eqop(e,tp,eg,ep,ev,re,r,rp)
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.eqfilter),tp,LOCATIONS_GRAVE_REMOVED,LOCATIONS_GRAVE_REMOVED,1,1,nil,1-tp,tp):GetFirst()
        let sel = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, x| eqfilter(d, x, 1 - tp, tp))),
            tp,
            LOCATIONS_GRAVE_REMOVED,
            LOCATIONS_GRAVE_REMOVED,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let Some(sc) = group::get_first(d, &sel) else { return };
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // s.equipop(c,e,tp,sc)
        equipop(d, c, a.e, tp, sc);
    }
}
// s.equipop(c,e,tp,tc)
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // if c:EquipByEffectAndLimitRegister(e,tp,tc,nil,true) then
    if card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, Some(true)) {
        // local e1a=Effect.CreateEffect(tc)
        let e1a = effect::create_effect(d, tc);
        // e1a:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e1a, EFFECT_TYPE_EQUIP);
        // e1a:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
        // e1a:SetValue(500)
        effect::set_value(d, e1a, 500);
        // e1a:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1a, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1a)
        card::register_effect(d, tc, e1a, false);
        // local e1b=e1a:Clone()
        let e1b = effect::clone(d, e1a);
        // e1b:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
        // tc:RegisterEffect(e1b)
        card::register_effect(d, tc, e1b, false);
    }
}
// s.negcostfilter(c)
fn negcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsEquipCard() and c:IsFaceup() and c:IsAbleToGraveAsCost()
    card::is_equip_card(d, c) && card::is_faceup(d, c) && card::is_able_to_grave_as_cost(d, c)
}
// s.negcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.negcostfilter,tp,LOCATION_SZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| negcostfilter(d, c)), tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.negcostfilter,tp,LOCATION_SZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| negcostfilter(d, c)), tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// the condition of e2: function(e,tp,eg,ep,ev,re,r,rp)
fn negate_condition(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsSpellTrapEffect() and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)
    let h = effect::get_handler(d, a.e).nil();
    a.rp == 1 - a.tp && effect::is_spell_trap_effect(d, a.re.nil()) && !card::is_status(d, h, STATUS_BATTLE_DESTROYED) && duel::is_chain_negatable(d, a.ev as u8)
}
