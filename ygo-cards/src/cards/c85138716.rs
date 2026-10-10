//! Rescue Rabbit
use super::prelude::*;

pub const ID: u32 = 85138716;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_DECK)
    effect::set_range(d, e1, LOCATION_DECK);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_NORMAL) && card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.filter2(c,g)`
fn filter2(d: &mut Duel, c: CardId, g: &CardSet) -> bool {
    // return g:IsExists(Card.IsCode,1,c,c:GetCode())
    let code = card::get_code(d, c)[0];
    group::is_exists(d, g, &|d, x| card::is_code(d, x, &[code]), 1, c.into())
}
/// `s.sptg(e,tp,...,chk)`
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
        let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
        // return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and ft>1 and g:IsExists(s.filter2,1,nil,g)
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none() && ft > 1 && {
            let g2 = g.clone();
            group::is_exists(d, &g, &|d, x| filter2(d, x, &g2), 1, NO_EX)
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    // local dg=g:Filter(s.filter2,nil,g)
    let mut dg = {
        let g2 = g.clone();
        group::filter(d, &g, &|d, x| filter2(d, x, &g2), NO_EX)
    };
    // if #dg>=1 then
    if !dg.is_empty() {
        // local fid=e:GetHandler():GetFieldID()
        let h = effect::get_handler(d, a.e).nil();
        let fid = card::get_field_id(d, h);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=dg:Select(tp,1,1,nil)
        let sg = group::select(d, &dg, a.tp, 1, 1, false, NO_EX).nil();
        // local tc1=sg:GetFirst()
        let tc1 = group::get_first(d, &sg).nil();
        // dg:RemoveCard(tc1)
        group::remove_card(&mut dg, &CardSet::from_card(tc1));
        // local tc2=dg:Filter(Card.IsCode,nil,tc1:GetCode()):GetFirst()
        let code1 = card::get_code(d, tc1)[0];
        let f2 = group::filter(d, &dg, &|d, x| card::is_code(d, x, &[code1]), NO_EX);
        let tc2 = group::get_first(d, &f2).nil();
        // Duel.SpecialSummonStep(tc1,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc1, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // Duel.SpecialSummonStep(tc2,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc2, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // tc1:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc1, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
        // tc2:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc2, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // sg:AddCard(tc2)
        let mut sg = sg;
        group::add_card(&mut sg, &CardSet::from_card(tc2));
        // sg:KeepAlive()  (groups are values in Rust)
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetLabel(fid)
        effect::set_label(d, e1, &[fid as i64]);
        // e1:SetLabelObject(sg)
        effect::set_label_object(d, e1, Value::Group(sg));
        // e1:SetCondition(s.descon)
        effect::set_condition(d, e1, Some(condition(descon)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `s.desfilter(c,fid)`
fn desfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.descon(e,tp,...)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let label = effect::get_label(d, a.e);
    // if not g:IsExists(s.desfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, x| desfilter(d, x, label), 1, NO_EX) {
        // g:DeleteGroup()
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        false
    } else {
        // return true
        true
    }
}
/// `s.desop(e,tp,...)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let label = effect::get_label(d, a.e);
    // local tg=g:Filter(s.desfilter,nil,e:GetLabel())
    let tg = group::filter(d, &g, &|d, x| desfilter(d, x, label), NO_EX);
    // Duel.Destroy(tg,REASON_EFFECT)
    duel::destroy(d, tg, REASON_EFFECT, None, None);
}
