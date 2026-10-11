//! World Legacy Monstrosity
use super::prelude::*;

pub const ID: u32 = 14604710;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 Level 9 monster from the hand or 2 from the deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter1(c,e,tp)`
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevel(9) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level(d, c, &[9]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spfilter3(c,e,tp,tc)`
fn spfilter3(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) -> bool {
    // return c:IsLevel(9) and c:GetOriginalRace()~=tc:GetOriginalRace() and c:GetOriginalAttribute()~=tc:GetOriginalAttribute()
    //   and not c:IsCode(tc:GetCode()) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let (race_c, race_tc) = (card::get_original_race(d, c), card::get_original_race(d, tc));
    let (attr_c, attr_tc) = (card::get_original_attribute(d, c), card::get_original_attribute(d, tc));
    let codes_tc = card::get_code(d, tc);
    card::is_level(d, c, &[9])
        && race_c != race_tc
        && attr_c != attr_tc
        && !card::is_code(d, c, &codes_tc)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not c:IsFaceup() or not c:IsLevel(9) then return false end
    if !card::is_faceup(d, c) || !card::is_level(d, c, &[9]) {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter3,tp,LOCATION_DECK,0,nil,e,tp,c)
    let g = duel::get_matching_group(d, Some(&|d, cc| spfilter3(d, cc, e, tp, c)), tp, LOCATION_DECK, 0, NO_EX);
    // return g:GetClassCount(Card.GetCode)>1
    group::get_class_count(d, &g, &|d, cc| card::get_code(d, cc)[0] as i64) > 1
}
/// `s.sptg1(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg1(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter1(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter2(chkc,e,tp) end
    // `s.filter2` is never defined in this script: calling it raises the Lua error.
    if let Some(chkc) = a.chkc() {
        if card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) {
            crate::lua_error("attempt to call a nil value");
        }
        return false;
    }
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //   and Duel.IsExistingTarget(s.spfilter2,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_existing_target(d, Some(&|d, c| spfilter2(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.spfilter2,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    duel::select_target(d, tp, Some(&|d, c| spfilter2(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_DECK as i32);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc) end
    if a.chkc().is_some() {
        return sptg2(d, a);
    }
    // local b1=s.sptg1(e,tp,eg,ep,ev,re,r,rp,0)
    let a0 = EffArgs { chk: Some(0), ..a.clone() };
    let b1 = sptg1(d, &a0);
    // local b2=s.sptg2(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = sptg2(d, &a0);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,0)}, {b2,aux.Stringid(id,1)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1))]);
    // if op==1 then
    if op == Some(1) {
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // e:SetOperation(s.spop1)
        effect::set_operation(d, a.e, Some(operation(spop1)));
        // s.sptg1(e,tp,eg,ep,ev,re,r,rp,1)
        let a1 = EffArgs { chk: Some(1), ..a.clone() };
        sptg1(d, &a1);
    } else if op == Some(2) {
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // e:SetOperation(s.spop2)
        effect::set_operation(d, a.e, Some(operation(spop2)));
        // s.sptg2(e,tp,eg,ep,ev,re,r,rp,1)
        let a1 = EffArgs { chk: Some(1), ..a.clone() };
        sptg2(d, &a1);
    }
    true
}
/// `s.spop1(e,tp,eg,ep,ev,re,r,rp)`
fn spop1(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter1,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter1(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<2 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if ft < 2 || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter3,tp,LOCATION_DECK,0,nil,e,tp,tc)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter3(d, c, e, tp, tc)), tp, LOCATION_DECK, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // if sg and #sg==2 then
    if sg.len() == 2 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c);
        // for sc in sg:Iter() do
        for sc in group::iter(d, &sg) {
            // Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            // Cannot attack
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3206)
            effect::set_description(d, e1, 3206);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1,true)
            card::register_effect(d, sc, e1, true);
            // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
            card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // sg:KeepAlive()
        // Destroy them during end phase
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
        // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e2:SetCountLimit(1)
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        // e2:SetLabel(fid)
        effect::set_label(d, e2, &[fid as i64]);
        // e2:SetLabelObject(sg)
        effect::set_label_object(d, e2, Value::Group(sg));
        // e2:SetCondition(s.descon)
        effect::set_condition(d, e2, Some(condition(descon)));
        // e2:SetOperation(s.desop)
        effect::set_operation(d, e2, Some(operation(desop)));
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
    }
}
/// `s.desfilter(c,fid)`
fn desfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first() == Some(&fid)
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
        false
    } else {
        // else return true end
        true
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let label = effect::get_label(d, a.e);
    // local tg=g:Filter(s.desfilter,nil,e:GetLabel())
    let tg = group::filter(d, &g, &|d, c| desfilter(d, c, label), NO_EX);
    // Duel.Destroy(tg,REASON_EFFECT)
    duel::destroy(d, tg, REASON_EFFECT, None, None);
}
