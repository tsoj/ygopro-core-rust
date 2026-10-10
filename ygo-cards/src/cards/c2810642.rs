//! PSY-Framegear Beta
use super::prelude::*;

pub const ID: u32 = 2810642;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_PSYFRAME_DRIVER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableUnsummonable()
    card::enable_unsummonable(d, c);
    // Must be special summoned by card effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.splimit)
    effect::set_value_fn(d, e1, lua_fn(splimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon itself and "PSY-Frame Driver", destroy attacking monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(condition_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.splimit(e,se,sp,st)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return se:IsHasType(EFFECT_TYPE_ACTIONS)
    let se = a.e(1);
    effect::is_has_type(d, se, EFFECT_TYPE_ACTIONS)
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():GetControler()~=tp and (Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    //   or Duel.IsPlayerAffectedByEffect(tp,CARD_PSYFRAME_LAMBDA))
    let atk = duel::get_attacker(d).nil();
    if card::get_controler(d, atk) == a.tp {
        return false;
    }
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
        || duel::is_player_affected_by_effect(d, a.tp, CARD_PSYFRAME_LAMBDA).is_some()
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_PSYFRAME_DRIVER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_PSYFRAME_DRIVER]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //   and Duel.GetAttacker():IsRelateToBattle()
    //   and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            return false;
        }
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 1 {
            return false;
        }
        let atk = duel::get_attacker(d).nil();
        if !card::is_relate_to_battle(d, atk) {
            return false;
        }
        let c = effect::get_handler(d, a.e).nil();
        if !card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None) {
            return false;
        }
        return duel::is_existing_matching_card(
            d,
            Some(&|d, x| spfilter(d, x, a.e, a.tp)),
            a.tp,
            LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetTargetCard(Duel.GetAttacker())
    let atk = duel::get_attacker(d).nil();
    duel::set_target_card(d, atk);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetAttacker(),1,0,0)
    let atk = duel::get_attacker(d).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(atk)), 1, 0, 0);
    true
}
// function s.rmfilter(c,fid)
fn rmfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 or not e:GetHandler():IsRelateToEffect(e) then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let mut g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, a.tp))),
        a.tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // local c=e:GetHandler()
    // local fid=c:GetFieldID()
    let fid = card::get_field_id(d, c) as i64;
    // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    // Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
    card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid, 0);
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid, 0);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // g:KeepAlive()  (groups are values: nothing to do)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetLabel(fid)
    effect::set_label(d, e1, &[fid]);
    // e1:SetLabelObject(g)
    effect::set_label_object(d, e1, Value::Group(g));
    // e1:SetCondition(s.rmcon)
    effect::set_condition(d, e1, Some(condition(rmcon)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local dc=Duel.GetFirstTarget()
    let dc = duel::first_target(d).nil();
    // if dc:IsRelateToEffect(e) and Duel.Destroy(dc,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, dc, a.e) && duel::destroy(d, dc, REASON_EFFECT, None, None) != 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SkipPhase(1-tp,PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
        duel::skip_phase(d, 1 - a.tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
    }
}
// function s.rmcon(e,tp,eg,ep,ev,re,r,rp)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let label = effect::get_label(d, a.e);
    // if not g:IsExists(s.rmfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, x| rmfilter(d, x, label), 1, NO_EX) {
        // g:DeleteGroup() (groups are values: nothing to do)
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // local tg=g:Filter(s.rmfilter,nil,e:GetLabel())
    let label = effect::get_label(d, a.e);
    let tg = group::filter(d, &g, &|d, x| rmfilter(d, x, label), NO_EX);
    // Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
