//! Goyo King
use super::prelude::*;

pub const ID: u32 = 84305651;

pub const INFO: ScriptInfo = ScriptInfo {
    synchro_nt_required: Some(1),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsType,TYPE_SYNCHRO),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_type_ctx(d, x, TYPE_SYNCHRO, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCondition(aux.bdogcon)
    effect::set_condition(d, e2, Some(aux::bdogcon()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetAttackTarget()~=nil
    duel::get_attack_target(d).is_some()
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local ct=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_MZONE,0,nil)
        let ct = duel::get_matching_group_count(d, Some(&filter), a.tp, LOCATION_MZONE, 0, NO_EX) as i64;
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(ct*400)
        effect::set_value(d, e1, ct * 400);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_PHASE | PHASE_DAMAGE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_WARRIOR) and c:IsAttribute(ATTRIBUTE_EARTH) and c:IsType(TYPE_SYNCHRO)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_WARRIOR) && card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_type(d, c, TYPE_SYNCHRO)
}
/// `s.ctfilter(c)`
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsControlerCanBeChanged()
    card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and bc:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let b1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, bc.nil(), a.e, 0, a.tp, false, false, None, None, None);
    // local b2=Duel.IsExistingMatchingCard(s.ctfilter,tp,0,LOCATION_MZONE,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&ctfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=0
    let op: i32;
    // if b1 and b2 then
    if b1 && b2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
        op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    } else if b1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1))
        op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]);
    } else {
        // op=Duel.SelectOption(tp,aux.Stringid(id,2))+1
        op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]) + 1;
    }
    // if op==0 then
    if op == 0 {
        // Duel.SetTargetCard(bc)
        duel::set_target_card(d, bc.nil());
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,bc,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(bc.nil())), 1, 0, 0);
    } else {
        // e:SetCategory(CATEGORY_CONTROL)
        effect::set_category(d, a.e, CATEGORY_CONTROL);
    }
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
        // local g=Duel.SelectMatchingCard(tp,s.ctfilter,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&ctfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // local tc=g:GetFirst()
        // if tc then
        if let Some(tc) = group::get_first(d, &g) {
            // Duel.GetControl(tc,tp)
            duel::get_control(d, tc, a.tp, 0, 0, None, None);
        }
    }
}
