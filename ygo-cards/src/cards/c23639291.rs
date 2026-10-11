//! Raging Cloudian
use super::prelude::*;

pub const ID: u32 = 23639291;
pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[COUNTER_FOG],
    listed_series: &[SET_CLOUDIAN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Cloudian" monster that was destroyed by its own effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e1:SetLabelObject(g)
    effect::set_label_object(d, e1, Value::Group(CardSet::new()));
    // Register when "Cloudian" monsters are sent to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // Duel.RegisterEffect(e2,0)
    duel::register_effect(d, e2, 0);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_CLOUDIAN) and c:IsControler(tp) and c:IsPreviousControler(tp)
    //     and c:IsReason(REASON_DESTROY) and c:GetReasonEffect() and c:GetReasonEffect():GetOwner()==c
    //     and c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK)
    card::is_set_card(d, c, &[SET_CLOUDIAN as u16])
        && card::is_controler(d, c, tp)
        && card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::get_reason_effect(d, c).is_some()
        && effect::get_owner(d, card::get_reason_effect(d, c).nil()) == Some(c)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_ATTACK as u8), None, None)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tg=eg:Filter(s.cfilter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| cfilter(d, c, e, tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do tc:RegisterFlagEffect(id,RESET_CHAIN,0,1) end
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e1 = effect::get_label_object(d, e).as_effect().nil();
        let mut g = effect::get_label_object(d, e1).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e1, Value::Group(g));
        // if Duel.GetFlagEffect(tp,id)==0 then
        if duel::get_flag_effect(d, tp, ID) == 0 {
            // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
            duel::register_flag_effect(d, tp, ID, RESET_CHAIN, 0, 1, 0);
            // Duel.RaiseEvent(eg,EVENT_CUSTOM+id,e,0,tp,tp,0)
            duel::raise_event(d, eg, EVENT_CUSTOM + ID, Some(e), 0, tp, tp, 0);
        }
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=e:GetLabelObject():Filter(s.cfilter,nil,e,tp)
    let lo = effect::get_label_object(d, e).as_group().cloned().nil();
    let g = group::filter(d, &lo, &|d, c| cfilter(d, c, e, tp), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.cfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && cfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && !g.is_empty();
    }
    // local tg=nil
    // if #g>1 then
    let tg = if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // tg=g:Select(tp,1,1,nil)
        group::select(d, &g, tp, 1, 1, false, NO_EX)
    } else {
        // tg=g
        Some(g)
    };
    // Duel.SetTargetCard(tg)
    let tg = tg.nil();
    duel::set_target_card(d, &tg);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), 1, tp, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_ATTACK) then
    if card::is_relate_to_effect(d, tc, e) && duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP_ATTACK as u8, None) {
        // tc:AddCounter(COUNTER_FOG,1)
        card::add_counter(d, tc, COUNTER_FOG as u16, 1, false);
        // It cannot be changed to Defense Position by card effects
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_CHANGE_POS_E)
        effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POS_E);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
