//! Oh Tokenbaum!
use super::prelude::*;

pub const ID: u32 = 97173708;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
// function s.rfilter(c,e,tp)
fn rfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return lv>0 and c:IsType(TYPE_TOKEN) and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,lv)
    lv > 0
        && card::is_type(d, c, TYPE_TOKEN)
        && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, e, tp, lv)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,clv)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, clv: u32) -> bool {
    // return c:IsLevel(clv) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level(d, c, &[clv]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.spcheck(sg,tp)
fn spcheck(d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet) -> (bool, bool) {
    // return aux.ReleaseCheckMMZ(sg,tp) and sg:GetClassCount(Card.GetLevel)==1
    let (mmz, _) = aux::release_check_mmz(d, sg, tp, exg);
    (mmz && group::get_class_count(d, sg, &|d, c| card::get_level(d, c) as i64) == 1, false)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp,e:GetLabel()) end
    if let Some(chkc) = a.chkc() {
        let lbl = effect::get_label(d, e) as u32;
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, e, tp, lbl);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return Duel.CheckReleaseGroupCost(tp,s.rfilter,2,false,s.spcheck,nil,e,tp)
        return duel::check_release_group_cost(
            d,
            tp,
            Some(&|d, c| rfilter(d, c, e, tp)),
            2,
            2,
            false,
            Some(&spcheck),
            NO_EX,
        );
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,s.rfilter,2,99,false,s.spcheck,nil,e,tp)
    let sg = duel::select_release_group_cost(d, tp, Some(&|d, c| rfilter(d, c, e, tp)), 2, 99, false, Some(&spcheck), NO_EX);
    // local lv=sg:GetFirst():GetLevel()
    let first = group::get_first(d, &sg).nil();
    let lv = card::get_level(d, first);
    // e:SetLabel(lv)
    effect::set_label(d, e, &[lv as i64]);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg.clone(), REASON_COST, None);
    // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_MZONE),#sg)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0.min(sg.len() as i32);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,ft,nil,e,tp,lv)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp, lv)), tp, LOCATION_GRAVE, 0, 1, ft.max(0) as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), n, 0, 0);
    true
}
// function s.desfilter(c,fid)
fn desfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first() == Some(&fid)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, e).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // if ft<#g then return end
    if ft < g.len() as i32 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local fid=c:GetFieldID()
    let fid = card::get_field_id(d, c) as i64;
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
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
    // g:KeepAlive()
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
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
    effect::set_label(d, e1, &[fid]);
    // e1:SetLabelObject(g)
    effect::set_label_object(d, e1, Value::Group(g));
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned().nil();
    let fid = effect::get_label(d, e);
    // if not g:IsExists(s.desfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, c| desfilter(d, c, fid), 1, NO_EX) {
        // g:DeleteGroup()
        // e:Reset()
        effect::reset(d, e);
        // return false
        false
    } else {
        // return true
        true
    }
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned().nil();
    let fid = effect::get_label(d, e);
    // local tg=g:Filter(s.desfilter,nil,e:GetLabel())
    let tg = group::filter(d, &g, &|d, c| desfilter(d, c, fid), NO_EX);
    // Duel.Destroy(tg,REASON_EFFECT)
    duel::destroy(d, tg, REASON_EFFECT, None, None);
}
