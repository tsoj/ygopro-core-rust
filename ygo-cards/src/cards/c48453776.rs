//! Sky Scourge Norleras
use super::prelude::*;

pub const ID: u32 = 48453776;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.FALSE)
    effect::set_value_fn(d, e1, aux_false());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // to grave
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOGRAVE+CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_TOGRAVE + CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e3, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e3:SetTarget(s.sgtg)
    effect::set_target(d, e3, Some(cost_target(sgtg)));
    // e3:SetOperation(s.sgop)
    effect::set_operation(d, e3, Some(operation(sgop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.atchk1,1,nil,sg)
    let chkf = aux::chkf_mmz(1);
    let (ok, _) = chkf(d, sg, e, tp, mg, None);
    (ok && group::is_exists(d, sg, &|d, x| atchk1(d, x, sg), 1, NO_EX), false)
}
/// `s.atchk1(c,sg)`
fn atchk1(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_FAIRY) and sg:FilterCount(s.atchk2,c)==3
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_FAIRY as u64)
        && group::filter_count(d, sg, &|d, x| atchk2(d, x), NO_EX) == 3
}
/// `s.atchk2(c,sg)`
fn atchk2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_FIEND)
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_race(d, c, RACE_FIEND as u64)
}
/// `s.spfilter(c,att,race)`
fn spfilter(d: &mut Duel, c: CardId, att: u32, race: u64) -> bool {
    // return c:IsAttribute(att) and c:IsRace(race) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_attribute(d, c, att) && card::is_race(d, c, race) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let e = a.e(0);
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_LIGHT,RACE_FAIRY)
    let rg1 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, ATTRIBUTE_LIGHT, RACE_FAIRY as u64)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_DARK,RACE_FIEND)
    let rg2 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, ATTRIBUTE_DARK, RACE_FIEND as u64)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local rg=rg1:Clone()
    // rg:Merge(rg2)
    let mut rg = rg1.clone();
    group::add_card(&mut rg, &rg2);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-4 and #rg1>0 and #rg2>2
    //   and aux.SelectUnselectGroup(rg,e,tp,4,4,s.rescon,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -4
        && !rg1.is_empty()
        && rg2.len() > 2
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(4), Some(4), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_LIGHT,RACE_FAIRY)
    let mut rg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, ATTRIBUTE_LIGHT, RACE_FAIRY as u64)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // rg:Merge(Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,ATTRIBUTE_DARK,RACE_FIEND))
    let rg2 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, ATTRIBUTE_DARK, RACE_FIEND as u64)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    group::add_card(&mut rg, &rg2);
    // local g=aux.SelectUnselectGroup(rg,e,tp,4,4,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(4), Some(4), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()  (no-op: groups are values)
}
/// `s.sgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND|LOCATION_ONFIELD,LOCATION_HAND|LOCATION_ONFIELD)
    let g = duel::get_field_group(d, a.tp, LOCATION_HAND | LOCATION_ONFIELD, LOCATION_HAND | LOCATION_ONFIELD);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,0,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.sgop(e,tp,eg,ep,ev,re,r,rp)`
fn sgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND|LOCATION_ONFIELD,LOCATION_HAND|LOCATION_ONFIELD)
    let g = duel::get_field_group(d, tp, LOCATION_HAND | LOCATION_ONFIELD, LOCATION_HAND | LOCATION_ONFIELD);
    // Duel.SendtoGrave(g,REASON_EFFECT)
    duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, tp, 1, REASON_EFFECT);
}
