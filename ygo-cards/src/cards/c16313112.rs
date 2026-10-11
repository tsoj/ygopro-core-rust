//! Salamangreat Emerald Eagle
use super::prelude::*;

pub const ID: u32 = 16313112;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_SALAMANGREAT}
    listed_series: &[SET_SALAMANGREAT],
    // s.listed_names={38784726,id}
    listed_names: &[38784726, ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // aux.EnableCheckReincarnation(c)
    aux::enable_check_reincarnation(d, c);
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.atcost)
    effect::set_cost(d, e2, Some(cost_target(atcost)));
    // e2:SetOperation(s.atop)
    effect::set_operation(d, e2, Some(operation(atop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c,fc,sumtype,tp)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLinkMonster() and c:IsSetCard(SET_SALAMANGREAT)
    card::is_link_monster(d, c) && card::is_set_card(d, c, &[SET_SALAMANGREAT as u16])
}
// function s.atcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn atcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil)
    //   and e:GetHandler():GetFlagEffect(id+1)==0 end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c)), 1, 1, false, None, NO_EX)
            && card::get_flag_effect(d, effect::get_handler(d, a.e).nil(), ID + 1) == 0;
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.atop(e,tp,eg,ep,ev,re,r,rp)
fn atop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) and c:GetFlagEffect(id)==0 then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && card::get_flag_effect(d, c, ID) == 0 {
        // c:RegisterFlagEffect(id+1,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, c, ID + 1, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
        // e1:SetCode(EVENT_BATTLE_START)
        effect::set_code(d, e1, EVENT_BATTLE_START);
        // e1:SetOwnerPlayer(tp)
        effect::set_owner_player(d, e1, tp);
        // e1:SetCondition(s.descon2)
        effect::set_condition(d, e1, Some(condition(descon2)));
        // e1:SetOperation(s.desop2)
        effect::set_operation(d, e1, Some(operation(desop2)));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
}
// function s.descon2(e,tp,eg,ep,ev,re,r,rp)
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h);
    // return tp==e:GetOwnerPlayer() and tc and tc:IsControler(1-tp)
    if a.tp != effect::get_owner_player(d, a.e) {
        return false;
    }
    match tc {
        Some(tc) => card::is_controler(d, tc, 1 - a.tp),
        None => false,
    }
}
// function s.desop2(e,tp,eg,ep,ev,re,r,rp)
fn desop2(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, h).nil();
    // if Duel.Destroy(tc,REASON_EFFECT)==0 then return end
    if duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) == 0 {
        return;
    }
    // Duel.Damage(1-tp,tc:GetBaseAttack(),REASON_EFFECT)
    let atk = card::get_base_attack(d, tc);
    duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReincarnationSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_reincarnation_summoned(d, h)
}
// function s.desfilter(c)
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpecialSummoned()
    card::is_special_summoned(d, c)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
