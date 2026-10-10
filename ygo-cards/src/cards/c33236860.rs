//! Blackwing - Silverwind the Ascendant
use super::prelude::*;

pub const ID: u32 = 33236860;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_BLACKWING],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro Summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_BLACKWING),1,1,Synchro.NonTuner(nil),2,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_BLACKWING as u16], sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        2,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Destroy up to 2 monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetCost(s.descost)
    effect::set_cost(d, e1, Some(cost_target(descost)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent the destruction of a "Blackwing" monster by battle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e) return Duel.IsTurnPlayer(1-e:GetHandlerPlayer()) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, 1 - effect::get_handler_player(d, a.e)))));
    // e2:SetTarget(function (e,c) return c:IsSetCard(SET_BLACKWING) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_BLACKWING as u16]))));
    // e2:SetValue(s.indesval)
    effect::set_value_fn(d, e2, lua_fn(indesval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetActivityCount(tp,ACTIVITY_BATTLE_PHASE)==0 end
    if a.chk == Some(0) {
        return duel::get_activity_count(d, a.tp, &[ACTIVITY_BATTLE_PHASE])[0] == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
/// `s.filter(c,atk)`
fn filter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:IsDefenseBelow(atk-1)
    card::is_faceup(d, c) && card::is_defense_below(d, c, atk - 1)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc,c:GetAttack()) end
    if let Some(chkc) = a.chkc() {
        let atk = card::get_attack(d, c);
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc, atk);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,c:GetAttack()) end
    if a.chk == Some(0) {
        let atk = card::get_attack(d, c);
        return duel::is_existing_target(d, Some(&|d, x| filter(d, x, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,2,nil,c:GetAttack())
    let atk = card::get_attack(d, c);
    let g = duel::select_target(d, a.tp, Some(&|d, x| filter(d, x, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desfilter(c,e,atk)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, atk: i32) -> bool {
    // return c:IsFaceup() and c:IsRelateToEffect(e) and c:IsDefenseBelow(atk-1)
    card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e) && card::is_defense_below(d, c, atk - 1)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    // local sg=g:Filter(s.desfilter,nil,e,c:GetAttack())
    let atk = card::get_attack(d, c);
    let sg = group::filter(d, &g, &|d, x| desfilter(d, x, a.e, atk), NO_EX);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
/// `s.indesval(e,re,r,rp)`
fn indesval(_d: &mut Duel, a: Args) -> bool {
    // return r&REASON_BATTLE>0
    (a.int(2) as u32 & REASON_BATTLE) > 0
}
