//! Magikey Fiend - Transfurlmine
//! (Lua: 魔鍵変鬼－トランスフルミネ)
//! scripted by the Razgriz
use super::prelude::*;

pub const ID: u32 = 69522668;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGIKEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_MAGIKEY),1,1,Synchro.NonTunerEx(Card.IsType,TYPE_NORMAL),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_MAGIKEY as u16], sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_NORMAL, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Make up to 2 attacks on monsters each Battle Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK_MONSTER);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Check materials' attributes before summoning
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE);
    // e2a:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2a, EFFECT_MATERIAL_CHECK);
    // e2a:SetValue(s.matcheck)
    effect::set_value_fn(d, e2a, lua_fn(matcheck));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // Set 1 Magikey S/T from Deck to S/T Zone
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetLabelObject(e2a)
    effect::set_label_object(d, e2, Value::Effect(e2a));
    // e2:SetCondition(s.setcon)
    effect::set_condition(d, e2, Some(condition(setcon)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy a NS/SSd monster with the same Att as monsters in your GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, a: Args) -> Value {
    let (e, c) = (a.e(0), a.c(1));
    // e:SetLabel(c:GetMaterial():GetClassCount(Card.GetAttribute))
    let mg = card::get_material(d, c);
    let n = group::get_class_count(d, &mg, &|d, x| card::get_attribute(d, x) as i64);
    effect::set_label(d, e, &[n as i64]);
    Value::Nil
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local obj=e:GetLabelObject()
    let obj = effect::get_label_object(d, a.e).as_effect();
    // return e:GetHandler():IsSynchroSummoned() and obj and obj:GetLabel()>1
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_synchro_summoned(d, h) {
        return false;
    }
    match obj {
        Some(obj) => effect::get_label(d, obj) > 1,
        None => false,
    }
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MAGIKEY) and c:IsSpellTrap() and not c:IsType(TYPE_FIELD) and c:IsSSetable()
    card::is_set_card(d, c, &[SET_MAGIKEY as u16]) && card::is_spell_trap(d, c) && !card::is_type(d, c, TYPE_FIELD) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g).nil();
        // Duel.SSet(tp,tc)
        duel::s_set(d, tp, CardSet::from_card(tc), None, true);
    }
}
/// `s.get_attr(tp)`
fn get_attr(d: &mut Duel, tp: u8) -> u32 {
    // local att=0
    let mut att = 0u32;
    // for tc in Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_GRAVE,0,nil):Iter() do
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c)), tp, LOCATION_GRAVE, 0, NO_EX);
    for tc in group::iter(d, &g) {
        // att=att|tc:GetAttribute()
        att |= card::get_attribute(d, tc);
    }
    // return att
    att
}
/// `s.desfilter(c,tp,att)`
fn desfilter(d: &mut Duel, c: CardId, tp: u8, att: u32) -> bool {
    // return c:IsSummonPlayer(1-tp) and c:GetAttribute()&att>0
    card::is_summon_player(d, c, 1 - tp) && (card::get_attribute(d, c) & att) > 0
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local att=s.get_attr(tp)
    let att = get_attr(d, a.tp);
    // return eg:IsExists(s.desfilter,1,nil,tp,att)
    let tp = a.tp;
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| desfilter(d, c, tp, att), 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=eg:Filter(s.desfilter,nil,tp,s.get_attr(tp))
    let att = get_attr(d, tp);
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| desfilter(d, c, tp, att), NO_EX);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=eg:Filter(s.desfilter,nil,tp,s.get_attr(tp)):Filter(Card.IsRelateToEffect,nil,e)
    let att = get_attr(d, tp);
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| desfilter(d, c, tp, att), NO_EX);
    let e = a.e;
    let g = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
