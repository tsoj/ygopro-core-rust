//! Magikey Deity - Ashtartu
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 26988374;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGIKEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure
    // Xyz.AddProcedure(c,nil,8,2)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, None, None, false, None);
    // Inflict damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.damcon)
    effect::set_condition(d, e1, Some(condition(damcon)));
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Detach 1 material and banish
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) and Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(lua_fn(rm_cond)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc,bc=Duel.GetBattleMonster(tp)
    let (rc, bc) = duel::get_battle_monster(d, a.tp);
    // return rc and bc and bc:GetBaseAttack()>0 and (rc:IsType(TYPE_NORMAL) or rc:IsSetCard(SET_MAGIKEY))
    //     and rc:IsControler(tp) and bc:IsPreviousControler(1-tp)
    let (Some(rc), Some(bc)) = (rc, bc) else {
        return false;
    };
    card::get_base_attack(d, bc) > 0
        && (card::is_type(d, rc, TYPE_NORMAL) || card::is_set_card(d, rc, &[SET_MAGIKEY as u16]))
        && card::is_controler(d, rc, a.tp)
        && card::is_previous_controler(d, bc, 1 - a.tp)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local _,bc=Duel.GetBattleMonster(tp)
    let (_, bc) = duel::get_battle_monster(d, a.tp);
    // local dam=bc:GetBaseAttack()
    let dam = card::get_base_attack(d, bc.nil());
    // if chk==0 then return dam>0 end
    if a.chk == Some(0) {
        return dam > 0;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
/// `s.attrfilter(c)`
fn attrfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and (c:IsType(TYPE_NORMAL) or c:IsSetCard(SET_MAGIKEY)) and c:IsFaceup()
    card::is_monster(d, c) && (card::is_type(d, c, TYPE_NORMAL) || card::is_set_card(d, c, &[SET_MAGIKEY as u16])) && card::is_faceup(d, c)
}
/// `s.rmfilter(c,ag)`
fn rmfilter(d: &mut Duel, c: CardId, ag: &CardSet) -> bool {
    // if not (c:IsFaceup() and c:IsAbleToRemove()) then return false end
    if !(card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)) {
        return false;
    }
    // return ag:IsExists(Card.IsAttribute,1,nil,c:GetAttribute())
    let attr = card::get_attribute(d, c);
    group::is_exists(d, ag, &|d, x| card::is_attribute(d, x, attr), 1, NO_EX)
}
/// `function(_,tp) return Duel.IsTurnPlayer(1-tp) and Duel.IsMainPhase() end`
fn rm_cond(d: &mut Duel, a: Args) -> bool {
    duel::is_turn_player(d, 1 - a.p(1)) && duel::is_main_phase(d, None)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ag=Duel.GetMatchingGroup(s.attrfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let ag = duel::get_matching_group(d, Some(&|d, c| attrfilter(d, c)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.rmfilter(chkc,ag) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && rmfilter(d, chkc, &ag);
    }
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,0,LOCATION_MZONE,1,nil,ag)
    //     and e:GetHandler():CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rmfilter(d, c, &ag)), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
            && card::check_remove_overlay_card(d, h, a.tp, 1, REASON_EFFECT);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,0,LOCATION_MZONE,1,1,nil,ag)
    let g = duel::select_target(d, a.tp, Some(&|d, c| rmfilter(d, c, &ag)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = group::get_count(&g) as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)>0 and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e)
        && card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT) > 0
        && card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc.nil(), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
