//! Altergeist Memorygant
//! OCG updates from the anime version by Naim
use super::prelude::*;

pub const ID: u32 = 23790299;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ALTERGEIST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon Procedure
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ALTERGEIST),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_ALTERGEIST as u16], sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // Increase its ATK by the ATK of a monster tributed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 monster the opponent controls and make a second attack in a row
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destruction replacement
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetTarget(s.desreptg)
    effect::set_target(d, e3, Some(cost_target(desreptg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase() and aux.StatChangeDamageStepCondition()
    duel::is_battle_phase(d, None) && aux::stat_change_damage_step_condition(d)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsAttackAbove,1,false,nil,e:GetHandler(),1) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::check_release_group_cost(d, tp, Some(&|d, c| card::is_attack_above(d, c, 1)), 1, 1, false, None, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TRIBUTE as u64);
    // local rg=Duel.SelectReleaseGroupCost(tp,Card.IsAttackAbove,1,1,false,nil,e:GetHandler(),1)
    let h = effect::get_handler(d, e).nil();
    let rg = duel::select_release_group_cost(d, tp, Some(&|d, c| card::is_attack_above(d, c, 1)), 1, 1, false, None, h.into());
    // e:SetLabel(rg:GetFirst():GetAttack())
    let tc = group::get_first(d, &rg).nil();
    let atk = card::get_attack(d, tc);
    effect::set_label(d, e, &[atk as i64]);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(e:GetLabel())
    let label = effect::get_label(d, a.e);
    effect::set_value(d, e1, label);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.repfilter(c)`
fn repfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE)
    //     and Duel.IsExistingMatchingCard(s.repfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && duel::is_existing_matching_card(d, Some(&repfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into());
    }
    // if Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.repfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,c)
        let g = duel::select_matching_card(d, tp, Some(&repfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
        // Duel.Remove(g:GetFirst(),POS_FACEUP,REASON_EFFECT)
        let tc = group::get_first(d, &g).nil();
        duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsRelateToBattle() and c:GetBattleTarget():IsReason(REASON_BATTLE)
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    let bt = card::get_battle_target(d, c).nil();
    card::is_reason(d, bt, REASON_BATTLE)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let _p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and c:CanChainAttack() and Duel.Destroy(tc,REASON_EFFECT)~=0 then
    //     Duel.ChainAttack()
    // end
    if let Some(tc) = tc {
        if card::can_chain_attack(d, c, None, false) && duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) != 0 {
            duel::chain_attack(d, None);
        }
    }
}
