//! Bystial Baldrake
use super::prelude::*;

pub const ID: u32 = 72656408;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_BRANDED as u32, SET_BYSTIAL as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand (Ignition)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(aux.NOT(s.spquickcon))
    effect::set_condition(d, e1, Some(condition(|d, a| !spquickcon(d, a))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon itself from the hand (Quick if the opponent controls monsters)
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetCondition(s.spquickcon)
    effect::set_condition(d, e2, Some(condition(spquickcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish 1 monster summoned by the opponent
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3a, aux::stringid(ID, 1));
    // e3a:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3a, CATEGORY_REMOVE);
    // e3a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3a, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3a:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e3a, EVENT_CUSTOM + ID);
    // e3a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3a, LOCATION_MZONE);
    // e3a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3a, 1, ID, 1, 0);
    // e3a:SetCost(s.rmcost)
    effect::set_cost(d, e3a, Some(cost_target(rmcost)));
    // e3a:SetTarget(s.rmtg)
    effect::set_target(d, e3a, Some(cost_target(rmtg)));
    // e3a:SetOperation(s.rmop)
    effect::set_operation(d, e3a, Some(operation(rmop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e3a:SetLabelObject(g)
    effect::set_label_object(d, e3a, Value::Group(CardSet::new()));
    // Register summons
    // local e3b=Effect.CreateEffect(c)
    let e3b = effect::create_effect(d, c);
    // e3b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3b, EVENT_SPSUMMON_SUCCESS);
    // e3b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3b, LOCATION_MZONE);
    // e3b:SetLabelObject(e3a)
    effect::set_label_object(d, e3b, Value::Effect(e3a));
    // e3b:SetOperation(s.regop)
    effect::set_operation(d, e3b, Some(operation(regop)));
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
/// `s.spquickcon(e,tp,eg,ep,ev,re,r,rp)`
fn spquickcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0
}
/// `s.spfilter(c,tp)`
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    //     and c:IsFaceup() and Duel.GetMZoneCount(tp,c)>0
    card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false)
        && card::is_faceup(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.spfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && spfilter(d, chkc, a.tp);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_GRAVE,1,nil,tp)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, x| spfilter(d, x, a.tp)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_REMOVED) and c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, tc, LOCATION_REMOVED)
        && card::is_relate_to_effect(d, c, a.e)
    {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.tgfilter(c,tp,e)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8, e: Option<EffectId>) -> bool {
    // return c:IsSummonPlayer(1-tp) and c:IsLocation(LOCATION_MZONE)
    //     and c:IsAbleToRemove() and c:IsType(TYPE_RITUAL|TYPE_EXTRA)
    //     and (not e or c:IsCanBeEffectTarget(e))
    card::is_summon_player(d, c, 1 - tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_able_to_remove(d, c, None, None, None)
        && card::is_type(d, c, TYPE_RITUAL | TYPE_EXTRA)
        && (e.is_none() || card::is_can_be_effect_target(d, c, e))
}
/// `s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local sg=e:GetLabelObject():Filter(s.tgfilter,nil,tp,e)
    let lo = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let sg = group::filter(d, &lo, &|d, x| tgfilter(d, x, a.tp, Some(a.e)), NO_EX);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_DARK|ATTRIBUTE_LIGHT),1,false,aux.ReleaseCheckTarget,c,sg) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(
            d,
            a.tp,
            Some(&|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK | ATTRIBUTE_LIGHT)),
            1,
            1,
            false,
            Some(&|d: &mut Duel, sel: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sel, tp, exg, &sg)),
            c.into(),
        );
    }
    // local g=Duel.SelectReleaseGroupCost(tp,aux.FilterBoolFunction(Card.IsAttribute,ATTRIBUTE_DARK|ATTRIBUTE_LIGHT),1,1,false,aux.ReleaseCheckTarget,c,sg)
    let g = aux::select_release_group_cost(
        d,
        a.tp,
        Some(&|d, x| card::is_attribute(d, x, ATTRIBUTE_DARK | ATTRIBUTE_LIGHT)),
        1,
        1,
        false,
        Some(&|d: &mut Duel, sel: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sel, tp, exg, &sg)),
        c.into(),
    );
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject():Filter(s.tgfilter,nil,tp,e)
    let lo = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &lo, &|d, x| tgfilter(d, x, a.tp, Some(a.e)), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.tgfilter(chkc,tp,nil) end
    if let Some(chkc) = a.chkc() {
        return g.contains(chkc) && tgfilter(d, chkc, a.tp, None);
    }
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local tc=nil
    // if #g==1 then
    let tc: CardSet = if g.len() == 1 {
        // tc=g:GetFirst()
        let t = group::get_first(d, &g).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, t);
        CardSet::from_card(t)
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // tc=g:Select(tp,1,1,nil)
        let t = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, t.clone());
        t
    };
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(tc), 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFlagEffect(tp,id)>0 then return end
    if duel::get_flag_effect(d, a.tp, ID) > 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tg=eg:Filter(s.tgfilter,nil,tp)
    let tg = group::filter(d, &a.eg.clone().nil(), &|d, x| tgfilter(d, x, a.tp, None), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e3a = effect::get_label_object(d, a.e).as_effect().nil();
        let mut g = effect::get_label_object(d, e3a).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g.clear();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, x| card::get_flag_effect(d, x, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e3a, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
    }
}
