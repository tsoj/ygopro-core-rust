//! Evoltile Casinerio
use super::prelude::*;

pub const ID: u32 = 56240989;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(6) and c:IsRace(RACE_DINOSAUR) and c:IsAttribute(ATTRIBUTE_FIRE)
    //     and c:IsCanBeSpecialSummoned(e,154,tp,false,false)
    card::is_level_below(d, c, 6)
        && card::is_race(d, c, RACE_DINOSAUR)
        && card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_can_be_special_summoned(d, c, e, 154, tp, false, false, None, None, None)
}
/// `s.filter2(c,g)`
fn filter2(d: &mut Duel, c: CardId, g: &CardSet) -> bool {
    // return g:IsExists(Card.IsCode,1,c,c:GetCode())
    let codes = card::get_code(d, c);
    group::is_exists(d, g, &|d, x| card::is_code(d, x, &codes), 1, c.into())
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, x| filter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
        // return ft>1 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and g:IsExists(s.filter2,1,nil,g)
        return ft > 1
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && group::is_exists(d, &g, &|d, x| filter2(d, x, &g), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2
    {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| filter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    // local dg=g:Filter(s.filter2,nil,g)
    let dg = group::filter(d, &g, &|d, x| filter2(d, x, &g), NO_EX);
    // if #dg>=1 then
    if !dg.is_empty() {
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c) as i64;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=dg:Select(tp,1,1,nil)
        let sg = group::select(d, &dg, a.tp, 1, 1, false, NO_EX).nil();
        // local tc1=sg:GetFirst()
        let tc1 = group::get_first(d, &sg).nil();
        // local tc2=dg:Filter(Card.IsCode,tc1,tc1:GetCode()):GetFirst()
        let codes1 = card::get_code(d, tc1);
        let same_code = group::filter(d, &dg, &|d, x| card::is_code(d, x, &codes1), tc1.into());
        let tc2 = group::get_first(d, &same_code).nil();
        // Duel.SpecialSummonStep(tc1,154,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc1, 154, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // Duel.SpecialSummonStep(tc2,154,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc2, 154, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // tc1:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc1, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid, 0);
        // tc2:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc2, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid, 0);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc1:RegisterEffect(e1,true)
        card::register_effect(d, tc1, e1, true);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // tc2:RegisterEffect(e2,true)
        card::register_effect(d, tc2, e2, true);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // tc1:RegisterEffect(e3,true)
        card::register_effect(d, tc1, e3, true);
        // local e4=e3:Clone()
        let e4 = effect::clone(d, e3);
        // tc2:RegisterEffect(e4,true)
        card::register_effect(d, tc2, e4, true);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // sg:AddCard(tc2)
        let mut sg = sg;
        group::add_card(&mut sg, &CardSet::from_card(tc2));
        // sg:KeepAlive()  (groups are values in Rust: nothing to keep alive)
        // local e5=Effect.CreateEffect(c)
        let e5 = effect::create_effect(d, c);
        // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e5:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e5, EVENT_PHASE + PHASE_END);
        // e5:SetCountLimit(1)
        effect::set_count_limit(d, e5, 1, 0, 0, 0);
        // e5:SetLabel(fid)
        effect::set_label(d, e5, &[fid]);
        // e5:SetLabelObject(sg)
        effect::set_label_object(d, e5, Value::Group(sg));
        // e5:SetCondition(s.rmcon)
        effect::set_condition(d, e5, Some(condition(rmcon)));
        // e5:SetOperation(s.rmop)
        effect::set_operation(d, e5, Some(operation(rmop)));
        // Duel.RegisterEffect(e5,tp)
        duel::register_effect(d, e5, a.tp);
    }
}
/// `s.rmfilter(c,fid)`
fn rmfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    // if not g:IsExists(s.rmfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, x| rmfilter(d, x, fid), 1, NO_EX) {
        // g:DeleteGroup()  (nothing in Rust)
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    // local tg=g:Filter(s.rmfilter,nil,e:GetLabel())
    let tg = group::filter(d, &g, &|d, x| rmfilter(d, x, fid), NO_EX);
    // Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
