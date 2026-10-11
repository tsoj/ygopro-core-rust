//! Double-Edged Sword Technique
use super::prelude::*;

pub const ID: u32 = 21007444;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIX_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SIX_SAMURAI) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //   and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,2,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,2,2,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 2, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    // local sg=g:Filter(Card.IsRelateToEffect,nil,e)
    let mut sg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if #sg==0 or ft<=0 or (#sg>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then return end
    if sg.is_empty() || ft <= 0 || (sg.len() > 1 && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()) {
        return;
    }
    // if ft<#sg then
    if ft < sg.len() as i32 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // sg=sg:FilterSelect(tp,s.filter,ft,ft,nil,e,tp)
        sg = group::filter_select(d, &sg, a.tp, &|d, c| filter(d, c, a.e, a.tp), ft as u16, ft as u16, false, NO_EX).nil();
    }
    // if #sg>0 then
    if !sg.is_empty() {
        // local tc=sg:GetFirst()
        let _ = group::get_first(d, &sg);
        // local fid=e:GetHandler():GetFieldID()
        let h = effect::get_handler(d, a.e).nil();
        let fid = card::get_field_id(d, h);
        // for tc in aux.Next(sg) do
        for tc in group::iter(d, &sg) {
            // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_ATTACK) then
            if duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None) {
                // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,fid)
                card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid as i64, 0);
            }
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
        // sg:KeepAlive()
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetCondition(s.descon)
        effect::set_condition(d, e1, Some(condition(descon)));
        // e1:SetOperation(s.desop)
        effect::set_operation(d, e1, Some(operation(desop)));
        // e1:SetLabel(fid)
        effect::set_label(d, e1, &[fid as i64]);
        // e1:SetLabelObject(sg)
        effect::set_label_object(d, e1, Value::Group(sg));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `s.desfilter(c,fid)`
fn desfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let fid = effect::get_label(d, a.e);
    // if not g:IsExists(s.desfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, c| desfilter(d, c, fid), 1, NO_EX) {
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
    // local sg=e:GetLabelObject()
    let sg = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // local dg=sg:Filter(s.desfilter,nil,e:GetLabel())
    let fid = effect::get_label(d, a.e);
    let dg = group::filter(d, &sg, &|d, c| desfilter(d, c, fid), NO_EX);
    // sg:DeleteGroup()
    // if #dg>0 then
    if !dg.is_empty() {
        let order = group::iter(d, &dg);
        // local tg1=dg:GetFirst()
        let tg1 = order[0];
        // local at1=tg1:GetAttack()
        let at1 = card::get_attack(d, tg1) as i64;
        // local tg2=dg:GetNext()
        let tg2 = order.get(1).copied();
        // local at2=0
        let mut at2 = 0i64;
        // local dam=0
        let mut dam = 0i64;
        // if tg2 then at2=tg2:GetAttack() end
        if let Some(tg2c) = tg2 {
            at2 = card::get_attack(d, tg2c) as i64;
        }
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg.clone(), REASON_EFFECT, None, None);
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // if og:IsContains(tg1) then dam=dam+at1 end
        if group::is_contains(&og, tg1) {
            dam += at1;
        }
        // if tg2 and og:IsContains(tg2) then dam=dam+at2 end
        if let Some(tg2c) = tg2 {
            if group::is_contains(&og, tg2c) {
                dam += at2;
            }
        }
        // if dam~=0 then Duel.Damage(tp,dam,REASON_EFFECT) end
        if dam != 0 {
            duel::damage(d, a.tp, dam, REASON_EFFECT, false, None);
        }
    }
}
