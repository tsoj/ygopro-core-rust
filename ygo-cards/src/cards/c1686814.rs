//! Ultimaya Tzolkin
use super::prelude::*;

pub const ID: u32 = 1686814;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_POWER_TOOL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special summon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon procedure
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e2, LOCATION_EXTRA);
    // e2:SetCondition(s.sprcon)
    effect::set_condition(d, e2, Some(lua_fn(sprcon)));
    // e2:SetTarget(s.sprtg)
    effect::set_target(d, e2, Some(lua_fn(sprtg)));
    // e2:SetOperation(s.sprop)
    effect::set_operation(d, e2, Some(lua_fn(sprop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special summon monster from the Extra Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SSET)
    effect::set_code(d, e3, EVENT_SSET);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Cannot be target
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.tgcon)
    effect::set_condition(d, e4, Some(condition(tgcon)));
    // e4:SetValue(aux.imval1)
    effect::set_value_fn(d, e4, aux::imval1());
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e5, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e5:SetValue(1)
    effect::set_value(d, e5, 1);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.sprfilter(c)
fn sprfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetLevel()>4 and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::get_level(d, c) > 4 && card::is_able_to_grave_as_cost(d, c)
}
// function s.sprfilter1(c,tp,g,sc)
fn sprfilter1(d: &mut Duel, c: CardId, tp: u8, sc: CardId) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return c:IsType(TYPE_TUNER) and g:IsExists(s.sprfilter2,1,c,tp,c,sc,lv)
    card::is_type(d, c, TYPE_TUNER) && group::is_exists(d, &g, &|d, x| sprfilter2(d, x, tp, c, sc, lv), 1, Exclude::Card(c))
}
// function s.sprfilter2(c,tp,mc,sc,lv)
fn sprfilter2(d: &mut Duel, c: CardId, tp: u8, mc: CardId, sc: CardId, lv: u32) -> bool {
    // local sg=Group.FromCards(c,mc)
    let mut sg = CardSet::from_card(c);
    group::add_card(&mut sg, &CardSet::from_card(mc));
    // return c:GetLevel()==lv and not c:IsType(TYPE_TUNER) and Duel.GetLocationCountFromEx(tp,tp,sg,sc)>0
    card::get_level(d, c) == lv
        && !card::is_type(d, c, TYPE_TUNER)
        && duel::get_location_count_from_ex(d, tp, Some(tp), Exclude::Group(&sg), duel::ExtraCard::Card(sc), None).0 > 0
}
// function s.sprcon(e,c)
fn sprcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return g:IsExists(s.sprfilter1,1,nil,tp,g,c)
    group::is_exists(d, &g, &|d, x| sprfilter1(d, x, tp, c), 1, NO_EX)
}
// function s.sprtg(e,tp,eg,ep,ev,re,r,rp,c)
fn sprtg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g1=g:Filter(s.sprfilter1,nil,tp,g,c)
    let g1 = group::filter(d, &g, &|d, x| sprfilter1(d, x, tp, c), NO_EX);
    // local mg1=aux.SelectUnselectGroup(g1,e,tp,1,1,nil,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
    let mut mg1 = aux::select_unselect_group(d, &g1, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true).unwrap_or_default();
    // if #mg1>0 then
    if !mg1.is_empty() {
        // local mc=mg1:GetFirst()
        let mc = group::get_first(d, &mg1).nil();
        // local g2=g:Filter(s.sprfilter2,mc,tp,mc,c,mc:GetLevel())
        let mc_lv = card::get_level(d, mc);
        let g2 = group::filter(d, &g, &|d, x| sprfilter2(d, x, tp, mc, c, mc_lv), Exclude::Card(mc));
        // local mg2=aux.SelectUnselectGroup(g2,e,tp,1,1,nil,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
        let mg2 = aux::select_unselect_group(d, &g2, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true).unwrap_or_default();
        // mg1:Merge(mg2)
        group::add_card(&mut mg1, &mg2);
    }
    // if #mg1==2 then
    if mg1.len() == 2 {
        // mg1:KeepAlive()
        // e:SetLabelObject(mg1)
        effect::set_label_object(d, e, Value::Group(mg1));
        // return true
        return true;
    }
    // return false
    false
}
// function s.sprop(e,tp,eg,ep,ev,re,r,rp,c)
fn sprop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    if let Value::Group(g) = effect::get_label_object(d, e) {
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g, REASON_COST, None, None);
    }
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsControler,1,nil,tp)
    let tp = a.tp;
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| card::is_controler(d, c, tp), 1, NO_EX)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_POWER_TOOL) or ((c:GetLevel()==7 or c:GetLevel()==8) and c:IsRace(RACE_DRAGON)))
    //     and c:IsType(TYPE_SYNCHRO) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_set_card(d, c, &[SET_POWER_TOOL as u16]) || ((card::get_level(d, c) == 7 || card::get_level(d, c) == 8) && card::is_race(d, c, RACE_DRAGON)))
        && card::is_type(d, c, TYPE_SYNCHRO)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.tgcon(e)
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_SYNCHRO),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,e:GetHandler())
    let p = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_SYNCHRO))), p, LOCATION_MZONE, 0, 1, Exclude::Card(h))
}
