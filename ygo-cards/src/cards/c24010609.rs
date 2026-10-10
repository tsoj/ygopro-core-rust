//! Sky Striker Mecha Modules - Multirole
use super::prelude::*;

pub const ID: u32 = 24010609;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SKY_STRIKER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // to grave
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.tgcon)
    effect::set_condition(d, e2, Some(condition(tgcon)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // set cards
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_LEAVE_GRAVE+CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_LEAVE_GRAVE + CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.settg)
    effect::set_target(d, e3, Some(cost_target(settg)));
    // e3:SetOperation(s.setop)
    effect::set_operation(d, e3, Some(operation(setop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetOperation(s.regop)
    effect::set_operation(d, e4, Some(operation(regop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_CHAIN_NEGATED)
    effect::set_code(d, e5, EVENT_CHAIN_NEGATED);
    // e5:SetOperation(s.regop2)
    effect::set_operation(d, e5, Some(operation(regop2)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(tp,id+1)==0
    duel::get_flag_effect(d, a.tp, ID + 1) == 0
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,0,1,1,c)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, 0, 0);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.SendtoGrave(tc,REASON_EFFECT) end
    if card::is_relate_to_effect(d, tc, e) {
        duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    }
    // if Duel.GetFlagEffect(tp,id+1)~=0 then return end
    if duel::get_flag_effect(d, tp, ID + 1) != 0 {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetOperation(s.actop)
    effect::set_operation(d, e1, Some(operation(actop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
}
/// `s.actop(e,tp,eg,ep,ev,re,r,rp)`
fn actop(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let _rc = effect::get_handler(d, a.re.nil()).nil();
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() then
    let re = a.re.nil();
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) {
        // Duel.SetChainLimit(s.chainlm)
        duel::set_chain_limit(d, lua_fn(chainlm));
    }
}
/// `s.chainlm(e,rp,tp)`: `return tp==rp`
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    let (rp, tp) = (a.p(1), a.p(2));
    tp == rp
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let (e, tp, rp) = (a.e, a.tp, a.rp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    let re = a.re.nil();
    // if re:GetHandler():IsSetCard(SET_SKY_STRIKER) and re:IsSpellEffect() and rp==tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) then
    let rh = effect::get_handler(d, re).nil();
    if card::is_set_card(d, rh, &[SET_SKY_STRIKER as u16])
        && effect::is_spell_effect(d, re)
        && rp == tp
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
    {
        // local flag=c:GetFlagEffectLabel(id)
        let flag = card::get_flag_effect_label(d, c, ID).first().copied();
        // if flag then
        if let Some(flag) = flag {
            // c:SetFlagEffectLabel(id,flag+1)
            card::set_flag_effect_label(d, c, ID, (flag + 1) as u32);
        } else {
            // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,1)
            card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 1, 0);
        }
    }
}
/// `s.regop2(e,tp,eg,ep,ev,re,r,rp)`
fn regop2(d: &mut Duel, a: &EffArgs) {
    let (e, tp, rp) = (a.e, a.tp, a.rp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    let re = a.re.nil();
    // if re:GetHandler():IsSetCard(SET_SKY_STRIKER) and re:IsSpellEffect() and rp==tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) then
    let rh = effect::get_handler(d, re).nil();
    if card::is_set_card(d, rh, &[SET_SKY_STRIKER as u16])
        && effect::is_spell_effect(d, re)
        && rp == tp
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
    {
        // local flag=c:GetFlagEffectLabel(id)
        let flag = card::get_flag_effect_label(d, c, ID).first().copied();
        // if flag and flag>0 then
        if let Some(flag) = flag.filter(|&f| f > 0) {
            // c:SetFlagEffectLabel(id,flag-1)
            card::set_flag_effect_label(d, c, ID, (flag - 1) as u32);
        }
    }
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SKY_STRIKER) and c:IsSpell() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_SKY_STRIKER as u16]) && card::is_spell(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=e:GetHandler():GetFlagEffectLabel(id)
    let h = effect::get_handler(d, a.e).nil();
    let ct = card::get_flag_effect_label(d, h, ID).first().copied();
    // if chk==0 then return ct and ct>0 and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return matches!(ct, Some(ct) if ct > 0) && duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, 0);
    true
}
/// `s.rescon(ft)`: the rescon of `setop` with `ft` free Spell & Trap zones
fn rescon_fn(ft: i64) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| {
        // local fc=sg:FilterCount(Card.IsType,nil,TYPE_FIELD)
        let fc = group::filter_count(d, sg, &|d, c| card::is_type(d, c, TYPE_FIELD), NO_EX) as i64;
        // local c1=sg:GetClassCount(Card.GetCode)
        let c1 = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) as i64;
        // local c2=#sg
        let c2 = sg.len() as i64;
        // return c1==c2 and fc<=1 and c2-fc<=ft, c1~=c2 or fc>1 or c2-fc>ft
        (c1 == c2 && fc <= 1 && c2 - fc <= ft, c1 != c2 || fc > 1 || c2 - fc > ft)
    }
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.setfilter),tp,LOCATION_GRAVE,0,nil)
    let nf = aux::necro_valley_filter(setfilter);
    let g = duel::get_matching_group(d, Some(&nf), tp, LOCATION_GRAVE, 0, NO_EX);
    // local ct=e:GetHandler():GetFlagEffectLabel(id) or 0
    let ct = card::get_flag_effect_label(d, c, ID).first().copied().unwrap_or(0);
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 as i64;
    // local tg=aux.SelectUnselectGroup(g,e,tp,1,math.min(ct,ft+1),s.rescon(ft),1,tp,HINTMSG_SET)
    let rs = rescon_fn(ft);
    let rs: aux::Rescon = &rs;
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(ct.min(ft + 1).max(0) as usize), Some(rs), 1, tp, Some(HINTMSG_SET as u64), None, None, false).unwrap_or_default();
    // if #tg==0 or Duel.SSet(tp,tg)==0 then return end
    if tg.is_empty() || duel::s_set(d, tp, tg.clone(), None, true) == 0 {
        return;
    }
    // for tc in aux.Next(tg) do
    for tc in group::iter(d, &tg) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
