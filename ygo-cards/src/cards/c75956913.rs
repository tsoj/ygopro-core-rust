//! Gagaga Utopic Tactics
//! ガガガ・ホープ・タクティクス
use super::prelude::*;

pub const ID: u32 = 75956913;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UTOPIC, SET_GAGAGA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // "Utopia" and "Utopic" Xyz Monsters you control whose original Attribute is LIGHT cannot be destroyed by card effects, also your opponent cannot target them with card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(s.utopixyzfilter))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| utopixyzfilter(d, c))));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change the Levels of 2 face-up monsters you control, including a "Gagaga" monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e3, CATEGORY_LVCHANGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.lvtg)
    effect::set_target(d, e3, Some(cost_target(lvtg)));
    // e3:SetOperation(s.lvop)
    effect::set_operation(d, e3, Some(operation(lvop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy 1 card your opponent controls
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(s.descon)
    effect::set_condition(d, e4, Some(condition(descon)));
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.utopixyzfilter(c)`
fn utopixyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_UTOPIC) and c:IsType(TYPE_XYZ) and c:IsOriginalAttribute(ATTRIBUTE_LIGHT) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_UTOPIC as u16]) && card::is_type(d, c, TYPE_XYZ) && card::is_original_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_faceup(d, c)
}
/// `s.lvfilter(c,e)`
fn lvfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:HasLevel() and c:IsFaceup() and c:IsCanBeEffectTarget(e)
    card::has_level(d, c) && card::is_faceup(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_GAGAGA)
    (group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_GAGAGA as u16]), 1, NO_EX), false)
}
/// `s.nlvfilter(c,lv)`: `aux.NOT(Card.IsLevel)`
fn nlvfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    !card::is_level(d, c, &[lv])
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local gg=Duel.GetMatchingGroup(s.lvfilter,tp,LOCATION_MZONE,0,nil,e)
    let gg = duel::get_matching_group(d, Some(&|d, c| lvfilter(d, c, a.e)), a.tp, LOCATION_MZONE, 0, NO_EX);
    let rs: aux::Rescon = &rescon;
    // if chk==0 then return aux.SelectUnselectGroup(gg,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &gg, Some(a.e), a.tp, Some(2), Some(2), Some(rs), 0, a.tp, None, None, None, false), Err(true));
    }
    // local g1,g2=gg:Split(Card.IsSetCard,nil,SET_GAGAGA)
    let (g1, g2) = group::split(d, &gg, &|d, c| card::is_set_card(d, c, &[SET_GAGAGA as u16]), NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_LVRANK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_LVRANK as u64);
    // local lv=Duel.AnnounceNumber(tp,s.get_declarable_levels(g1,g2))
    let opts = get_declarable_levels(d, &g1, &g2);
    let lv = duel::announce_number(d, a.tp, &opts).nil().0 as u32;
    // local g=gg:Match(s.nlvfilter,nil,lv)
    let g = group::filter(d, &gg, &|d, c| nlvfilter(d, c, lv), NO_EX);
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(rs), 1, a.tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // e:SetLabel(lv)
    effect::set_label(d, a.e, &[lv as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,tg,2,tp,lv)
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(tg), 2, a.tp, lv as i32);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tg=Duel.GetTargetCards(e):Filter(Card.IsFaceup,nil)
    let tg0 = duel::get_target_cards(d, a.e).nil();
    let tg = group::filter(d, &tg0, &|d, x| card::is_faceup(d, x), NO_EX);
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local lv=e:GetLabel()
    let lv = effect::get_label(d, a.e) as u32;
    // for tc in tg:Iter() do
    for tc in group::iter(d, &tg) {
        // if not tc:IsLevel(lv) then
        if !card::is_level(d, tc, &[lv]) {
            // Its Level becomes the declared Level
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            // e1:SetValue(lv)
            effect::set_value(d, e1, lv as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.get_declarable_levels(g1,g2)`
fn get_declarable_levels(d: &mut Duel, g1: &CardSet, g2: &CardSet) -> Vec<u64> {
    // local opts={}
    let mut opts = Vec::new();
    // for lv=1,12 do
    for lv in 1..=12u32 {
        // local ct=g1:FilterCount(s.nlvfilter,nil,lv)
        let ct = group::filter_count(d, g1, &|d, c| nlvfilter(d, c, lv), NO_EX);
        // if ct>1 or (ct>0 and g2:IsExists(s.nlvfilter,1,nil,lv)) then
        if ct > 1 || (ct > 0 && group::is_exists(d, g2, &|d, c| nlvfilter(d, c, lv), 1, NO_EX)) {
            // table.insert(opts,lv)
            opts.push(lv as u64);
        }
    }
    // return table.unpack(opts)
    opts
}
/// `s.desconfilter(c,tp)`
fn desconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return s.utopixyzfilter(c) and c:IsSummonPlayer(tp) and c:IsXyzSummoned()
    utopixyzfilter(d, c) && card::is_summon_player(d, c, tp) && card::is_xyz_summoned(d, c)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.desconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| desconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
