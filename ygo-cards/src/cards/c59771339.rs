//! Junk Berserker
use super::prelude::*;

pub const ID: u32 = 59771339;

pub const INFO: ScriptInfo = ScriptInfo {
    material: &[CARD_JUNK_SYNCHRON],
    listed_names: &[CARD_JUNK_SYNCHRON],
    material_setcode: &[SET_SYNCHRON],
    listed_series: &[SET_JUNK],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,s.tunerfilter,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| tunerfilter(d, c, sc, st, tp))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tunerfilter(c,lc,stype,tp)`: `return c:IsSummonCode(lc,stype,tp,CARD_JUNK_SYNCHRON) or c:IsHasEffect(20932152)`
fn tunerfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, stype: u64, tp: u8) -> bool {
    card::is_summon_code(d, c, lc, stype, tp, &[CARD_JUNK_SYNCHRON]) || !card::is_has_effect(d, c, 20932152, None).is_empty()
}
/// `s.atkcostfilter(c)`
fn atkcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_JUNK) and c:IsMonster() and c:GetAttack()>0 and c:IsAbleToRemoveAsCost()
    //     and aux.SpElimFilter(c,true)
    card::is_set_card(d, c, &[SET_JUNK as u16])
        && card::is_monster(d, c)
        && card::get_attack(d, c) > 0
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&atkcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.atkcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&atkcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g).nil();
    // e:SetLabel(sc:GetAttack())
    let atk = card::get_attack(d, sc);
    effect::set_label(d, a.e, &[atk as i64]);
    // Duel.Remove(sc,POS_FACEUP,REASON_COST)
    duel::remove(d, CardSet::from_card(sc), POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), 0, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsControler(1-tp) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_controler(d, tc, 1 - a.tp) {
        // tc:UpdateAttack(-e:GetLabel(),nil,e:GetHandler())
        let label = effect::get_label(d, a.e);
        let h = effect::get_handler(d, a.e).nil();
        card::update_attack(d, tc, -label, None, Some(h), None);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d);
    // return e:GetHandler()==Duel.GetAttacker() and bc and bc:IsDefensePos()
    let h = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(h) && bc.is_some_and(|bc| card::is_defense_pos(d, bc))
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetAttackTarget(),1,tp,0)
    let bc = duel::get_attack_target(d);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, bc.map(CardSet::from_card), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, _a: &EffArgs) {
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d);
    // if bc and bc:IsRelateToBattle() then Duel.Destroy(bc,REASON_EFFECT) end
    if let Some(bc) = bc {
        if card::is_relate_to_battle(d, bc) {
            duel::destroy(d, CardSet::from_card(bc), REASON_EFFECT, None, None);
        }
    }
}
