//! Rescue Ferret
use super::prelude::*;

pub const ID: u32 = 56343672;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfToDeck)
    effect::set_cost(d, e1, Some(cost::self_to_deck()));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:GetLevel()>0 and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::get_level(d, c) > 0 && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `Card.GetLevel` as a value function
fn level_of(d: &mut Duel, c: CardId) -> i64 {
    card::get_level(d, c) as i64
}
/// `s.desfilter(c,fid)`
fn desfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local zone=aux.GetMMZonesPointedTo(tp)
        let zone = aux::get_mm_zones_pointed_to(d, a.tp, None, None, None, None);
        // local ct=Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)
        let mut ct = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0;
        // local seq=e:GetHandler():GetSequence()
        let h = effect::get_handler(d, a.e).nil();
        let seq = card::get_sequence(d, h);
        // if seq<5 and bit.extract(zone,seq)~=0 then ct=ct+1 end
        if seq < 5 && (zone >> seq) & 1 != 0 {
            ct += 1;
        }
        // if ct<=0 then return false end
        if ct <= 0 {
            return false;
        }
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
        if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ct = 1;
        }
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
        // return g:CheckWithSumEqual(Card.GetLevel,6,1,ct)
        return group::check_with_sum_equal(d, &g, &level_of, 6, 1, ct).0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zone=aux.GetMMZonesPointedTo(tp)
    let zone = aux::get_mm_zones_pointed_to(d, a.tp, None, None, None, None);
    // local ct=Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)
    let mut ct = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0;
    // if ct<=0 then return end
    if ct <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ct=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    // if g:CheckWithSumEqual(Card.GetLevel,6,1,ct) then
    if group::check_with_sum_equal(d, &g, &level_of, 6, 1, ct).0 {
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c) as i64;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:SelectWithSumEqual(tp,Card.GetLevel,6,1,ct)
        let sg = group::select_with_sum_equal(d, &g, a.tp, &level_of, 6, 1, ct).nil();
        // for tc in sg:Iter() do
        for tc in group::iter(d, &sg) {
            // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP,zone)
            duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid, 0);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // sg:KeepAlive()
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e3:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
        // e3:SetCountLimit(1)
        effect::set_count_limit(d, e3, 1, 0, 0, 0);
        // e3:SetLabel(fid)
        effect::set_label(d, e3, &[fid]);
        // e3:SetLabelObject(sg)
        effect::set_label_object(d, e3, Value::Group(sg));
        // e3:SetCondition(s.descon)
        effect::set_condition(d, e3, Some(condition(descon)));
        // e3:SetOperation(s.desop)
        effect::set_operation(d, e3, Some(operation(desop)));
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, a.tp);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let label = effect::get_label(d, a.e);
    // if not g:IsExists(s.desfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, c| desfilter(d, c, label), 1, NO_EX) {
        // g:DeleteGroup()
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // local tg=g:Filter(s.desfilter,nil,e:GetLabel())
    let label = effect::get_label(d, a.e);
    let tg = group::filter(d, &g, &|d, c| desfilter(d, c, label), NO_EX);
    // Duel.Destroy(tg,REASON_EFFECT)
    duel::destroy(d, tg, REASON_EFFECT, None, None);
}
