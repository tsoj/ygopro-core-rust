//! Trickstar Colchica
//! トリックスター・コルチカ
use super::prelude::*;

pub const ID: u32 = 298846;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRICKSTAR], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 1, Some(1), None, None, None);
    // You can only Special Summon "Trickstar Colchica(s)" once per turn
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Inflict damage equal to the ATK of a monster destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.damcond)
    effect::set_condition(d, e1, Some(condition(damcond)));
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_TRICKSTAR,lc,sumtype,tp) and not c:IsType(TYPE_LINK,lc,sumtype,tp)
    card::is_set_card_ctx(d, c, &[SET_TRICKSTAR as u16], lc, sumtype, Some(tp))
        && !card::is_type_ctx(d, c, TYPE_LINK, lc, sumtype, Some(tp))
}
/// `s.damconfilter(c,tp,ct)`
fn damconfilter(d: &mut Duel, c: CardId, tp: u8, ct: usize) -> bool {
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return (c:IsPreviousControler(tp) and c:IsPreviousSetCard(SET_TRICKSTAR))
    //   or (ct==1 and bc and bc:IsControler(tp) and bc:IsSetCard(SET_TRICKSTAR))
    (card::is_previous_controler(d, c, tp) && card::is_previous_set_card(d, c, &[SET_TRICKSTAR as u16]))
        || (ct == 1
            && match bc {
                Some(bc) => card::is_controler(d, bc, tp) && card::is_set_card(d, bc, &[SET_TRICKSTAR as u16]),
                None => false,
            })
}
/// `s.damcond(e,tp,eg,ep,ev,re,r,rp)`
fn damcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.damconfilter,1,nil,tp,#eg) and not eg:IsContains(e:GetHandler())
    let eg = a.eg.clone().nil();
    let n = eg.len();
    group::is_exists(d, &eg, &|d, c| damconfilter(d, c, a.tp, n), 1, NO_EX)
        && !group::is_contains(&eg, effect::get_handler(d, a.e).nil())
}
/// `s.damtgfilter(c,e)`
fn damtgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsCanBeEffectTarget(e) and c:GetBaseAttack()>0
    card::is_can_be_effect_target(d, c, Some(e)) && card::get_base_attack(d, c) > 0
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // if chkc then return eg:IsContains(chkc) and s.damtgfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&eg, chkc) && damtgfilter(d, chkc, a.e);
    }
    // if chk==0 then return eg:IsExists(s.damtgfilter,1,nil,e) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| damtgfilter(d, c, a.e), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tc=nil
    // if #eg>1 then
    let tc = if eg.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
        // tc=eg:FilterSelect(tp,s.damtgfilter,1,1,nil,e):GetFirst()
        let g = group::filter_select(d, &eg, a.tp, &|d, c| damtgfilter(d, c, a.e), 1, 1, false, NO_EX).nil();
        group::get_first(d, &g)
    } else {
        // tc=eg:GetFirst()
        group::get_first(d, &eg)
    };
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc.nil());
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,tc:GetBaseAttack())
    let atk = card::get_base_attack(d, tc.nil());
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:GetBaseAttack()>0 then
    if card::is_relate_to_effect(d, tc, a.e) && card::get_base_attack(d, tc) > 0 {
        // Duel.Damage(1-tp,tc:GetBaseAttack(),REASON_EFFECT)
        let atk = card::get_base_attack(d, tc);
        duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
    }
}
