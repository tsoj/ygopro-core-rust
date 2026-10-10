//! Red Supernova Dragon
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 99585850;

pub const INFO: ScriptInfo = ScriptInfo { synchro_nt_required: Some(1), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon Procedure
    // Synchro.AddProcedure(c,nil,3,3,Synchro.NonTuner(Card.IsType,TYPE_SYNCHRO),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        3,
        3,
        Some(synchro::non_tuner(Some(std::sync::Arc::new(|d, x| card::is_type(d, x, TYPE_SYNCHRO))))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Must first be synchro summoned
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(aux.synlimit)
    effect::set_value_fn(d, e0, aux::synlimit());
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Gains 500 ATK per Tuner monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by opponent's effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetValue(aux.indoval)
    effect::set_value_fn(d, e2, aux::indoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish itself an all cards the opponent controls
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e3, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e3:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e3, Some(condition(cond3)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetCondition(s.rmcon2)
    effect::set_condition(d, e4, Some(condition(rmcon2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Special summon itself during your next End Phase
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e5:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e5, EVENT_PHASE + PHASE_END);
    // e5:SetRange(LOCATION_REMOVED)
    effect::set_range(d, e5, LOCATION_REMOVED);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetCondition(s.spcon)
    effect::set_condition(d, e5, Some(condition(spcon)));
    // e5:SetTarget(s.sptg)
    effect::set_target(d, e5, Some(cost_target(sptg)));
    // e5:SetOperation(s.spop)
    effect::set_operation(d, e5, Some(operation(spop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // e3:SetLabelObject(e5)
    effect::set_label_object(d, e3, Value::Effect(e5));
    // e4:SetLabelObject(e5)
    effect::set_label_object(d, e4, Value::Effect(e5));
    // Double tuner
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e6, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e6:SetCode(EFFECT_MULTIPLE_TUNERS)
    effect::set_code(d, e6, EFFECT_MULTIPLE_TUNERS);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsType,c:GetControler(),LOCATION_GRAVE,0,nil,TYPE_TUNER)*500
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&|d, x| card::is_type(d, x, TYPE_TUNER)), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 500
}
/// `function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end`
fn cond3(d: &mut Duel, a: &EffArgs) -> bool {
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.tp)
}
/// `s.rmcon2(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect()
    a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil())
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if c:IsRelateToEffect(e) and c:IsAbleToRemove() then g:AddCard(c) end
    if card::is_relate_to_effect(d, c, a.e) && card::is_able_to_remove(d, c, None, None, None) {
        group::add_card(&mut g, &CardSet::from_card(c));
    }
    // if #g>0 and Duel.Remove(g,POS_FACEUP,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // if og:IsContains(c) then
        if group::is_contains(&og, c) {
            // local ct=1
            let mut ct: u16 = 1;
            // e:GetLabelObject() (the effect e5 that s.spcon reads)
            let e5 = effect::get_label_object(d, a.e).as_effect().nil();
            // if Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_END) then
            if duel::is_turn_player(d, a.tp) && duel::is_phase(d, PHASE_END) {
                // ct=2
                ct = 2;
                // e:GetLabelObject():SetLabel(Duel.GetTurnCount())
                let turn = duel::get_turn_count(d, None) as i64;
                effect::set_label(d, e5, &[turn]);
            } else {
                // e:GetLabelObject():SetLabel(0)
                effect::set_label(d, e5, &[0]);
            }
            // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END|RESET_SELF_TURN,0,ct)
            card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END | RESET_SELF_TURN, 0, ct, 0, 0);
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():HasFlagEffect(id) and e:GetLabel()~=Duel.GetTurnCount() and Duel.IsTurnPlayer(tp)
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) > 0
        && effect::get_label(d, a.e) != duel::get_turn_count(d, None) as i64
        && duel::is_turn_player(d, a.tp)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
