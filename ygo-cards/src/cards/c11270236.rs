//! Invoked Elysium
use super::prelude::*;

pub const ID: u32 = 11270236;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_INVOKED}
    listed_series: &[SET_INVOKED],
    // s.material_setcode=SET_INVOKED
    material_setcode: &[SET_INVOKED],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion materials
    // Fusion.AddProcMix(c,false,false,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_INVOKED),s.ffilter2)
    fusion::add_proc_mix(
        d,
        c,
        false,
        false,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_INVOKED as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, _fc, _st, _tp| ffilter2(d, x))),
        ],
    );
    // Must first be Fusion Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.splimit)
    effect::set_value_fn(d, e1, lua_fn(splimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Treated as DARK, EARTH, WATER, FIRE and WIND
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_ADD_ATTRIBUTE)
    effect::set_code(d, e2, EFFECT_ADD_ATTRIBUTE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(ATTRIBUTE_EARTH|ATTRIBUTE_WATER|ATTRIBUTE_FIRE|ATTRIBUTE_WIND|ATTRIBUTE_DARK)
    effect::set_value(d, e2, (ATTRIBUTE_EARTH | ATTRIBUTE_WATER | ATTRIBUTE_FIRE | ATTRIBUTE_WIND | ATTRIBUTE_DARK) as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish 1 "Invoked" monster and monsters with the same attribute
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.ffilter2(c)
fn ffilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSummonLocation(LOCATION_EXTRA) and c:IsLocation(LOCATION_MZONE)
    card::is_summon_location(d, c, LOCATION_EXTRA) && card::is_location(d, c, LOCATION_MZONE)
}
// function s.splimit(e,se,sp,st)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not e:GetHandler():IsLocation(LOCATION_EXTRA) or aux.fuslimit(e,se,sp,st)
    let h = effect::get_handler(d, a.e(0)).nil();
    if !card::is_location(d, h, LOCATION_EXTRA) {
        return true;
    }
    aux::fuslimit()
        .call(d, &[Value::Effect(a.e(0)), Value::Effect(a.e(1)), Value::Int(a.int(2)), Value::Int(a.int(3))])
        .truthy()
}
// function s.rmfilter1(c)
fn rmfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_INVOKED) and c:IsMonster() and (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup())
    //     and c:IsAbleToRemove() and aux.SpElimFilter(c,true,true)
    card::is_set_card(d, c, &[SET_INVOKED as u16])
        && card::is_monster(d, c)
        && (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, true)
}
// function s.rmfilter2(c,att)
fn rmfilter2(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsFaceup() and c:IsAttribute(att) and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_attribute(d, c, att) && card::is_able_to_remove(d, c, None, None, None)
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.rmfilter1(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && rmfilter1(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter1), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g1=Duel.SelectTarget(tp,s.rmfilter1,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let mut g1 = duel::select_target(d, tp, Some(&rmfilter1), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local g2=Duel.GetMatchingGroup(s.rmfilter2,tp,0,LOCATION_MZONE,nil,g1:GetFirst():GetAttribute())
    let first = group::get_first(d, &g1).nil();
    let att = card::get_attribute(d, first);
    let g2 = duel::get_matching_group(d, Some(&|d, x| rmfilter2(d, x, att)), tp, 0, LOCATION_MZONE, NO_EX);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g1,#g1,0,0)
    let n = g1.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g1), n, 0, 0);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local tg=Group.FromCards(tc)
        let mut tg = CardSet::from_card(tc);
        // if tc:IsFaceup() then
        if card::is_faceup(d, tc) {
            // local g=Duel.GetMatchingGroup(s.rmfilter2,tp,0,LOCATION_MZONE,nil,tc:GetAttribute())
            let att = card::get_attribute(d, tc);
            let g = duel::get_matching_group(d, Some(&|d, x| rmfilter2(d, x, att)), a.tp, 0, LOCATION_MZONE, NO_EX);
            // tg:Merge(g)
            group::add_card(&mut tg, &g);
        }
        // Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, &tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
