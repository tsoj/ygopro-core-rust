//! Scramble!! Scramble!!
use super::prelude::*;

pub const ID: u32 = 83054225;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MECHA_PHANTOM_BEAST],
    listed_names: &[TOKEN_MECHA_PHANTOM_BEAST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsType(TYPE_TOKEN)
    !card::is_type(d, c, TYPE_TOKEN)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let opp = duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE);
    let mine = duel::get_matching_group_count(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, NO_EX) as i32;
    opp > mine
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MECHA_PHANTOM_BEAST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MECHA_PHANTOM_BEAST as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let mut ct = duel::get_matching_group_count(d, Some(&|d, cc| spfilter(d, cc, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX) as i32;
    // if chk==0 then
    if a.chk == Some(0) {
        // if Duel.GetFlagEffect(tp,id)~=0 or Duel.GetLocationCount(tp,LOCATION_MZONE)<0 then return false end
        if duel::get_flag_effect(d, a.tp, ID) != 0 || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 0 {
            return false;
        }
        // return ct>0 and Duel.CheckReleaseGroupCost(tp,Card.IsCode,1,false,nil,nil,TOKEN_MECHA_PHANTOM_BEAST)
        return ct > 0
            && aux::check_release_group_cost(d, a.tp, Some(&|d, cc| card::is_code(d, cc, &[TOKEN_MECHA_PHANTOM_BEAST])), 1, 1, false, None, NO_EX);
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsCode,1,ct,false,nil,nil,TOKEN_MECHA_PHANTOM_BEAST)
    let g = aux::select_release_group_cost(d, a.tp, Some(&|d, cc| card::is_code(d, cc, &[TOKEN_MECHA_PHANTOM_BEAST])), 1, ct as usize, false, None, NO_EX);
    let n = g.len();
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // e:SetLabel(#g)
    effect::set_label(d, a.e, &[n as i64]);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,#g,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, n as u8, a.tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // if ft<ct then return end
    if (ft as i64) < ct {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, cc| spfilter(d, cc, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g<ct then return end
    if (g.len() as i64) < ct {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sg=g:Select(tp,ct,ct,nil)
    let sg = group::select(d, &g, a.tp, ct as u16, ct as u16, false, NO_EX).nil();
    // local fid=e:GetHandler():GetFieldID()
    let h = effect::get_handler(d, a.e).nil();
    let fid = card::get_field_id(d, h);
    // local tc=sg:GetFirst()
    let _tc = group::get_first(d, &sg);
    // for tc in aux.Next(sg) do
    for tc in group::iter(d, &sg) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // sg:KeepAlive() does nothing in Rust (groups are values)
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetLabel(fid)
    effect::set_label(d, e1, &[fid as i64]);
    // e1:SetLabelObject(sg)
    effect::set_label_object(d, e1, Value::Group(sg));
    // e1:SetCondition(s.retcon)
    effect::set_condition(d, e1, Some(condition(retcon)));
    // e1:SetOperation(s.retop)
    effect::set_operation(d, e1, Some(operation(retop)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// function s.retfilter(c,fid)
fn retfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
// function s.retcon(e,tp,eg,ep,ev,re,r,rp)
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    // if not g:IsExists(s.retfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, c| retfilter(d, c, fid), 1, NO_EX) {
        // g:DeleteGroup()
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
// function s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // local tg=g:Filter(s.retfilter,nil,e:GetLabel())
    let fid = effect::get_label(d, a.e);
    let tg = group::filter(d, &g, &|d, c| retfilter(d, c, fid), NO_EX);
    // Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
