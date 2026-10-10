//! Beast Machine King Barbaros Ür
use super::prelude::*;

pub const ID: u32 = 19028307;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // no battle damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_NO_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_NO_BATTLE_DAMAGE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.atchk1,1,nil,sg)
    let mmz = aux::chkf_mmz(1)(d, sg, e, tp, mg, c).0;
    if !mmz {
        return (false, false);
    }
    let res = group::is_exists(d, sg, &|d, x| atchk1(d, x, sg), 1, NO_EX);
    (res, false)
}
/// `s.atchk1(c,sg)`
fn atchk1(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    // return c:IsRace(RACE_MACHINE) and sg:FilterCount(s.atchk2,c)==1
    card::is_race(d, c, RACE_MACHINE) && group::filter_count(d, sg, &|d, x| atchk2(d, x), c.into()) == 1
}
/// `s.atchk2(c)`
fn atchk2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_BEASTWARRIOR)
    card::is_race(d, c, RACE_BEASTWARRIOR)
}
/// `s.spfilter(c,rac)`
fn spfilter(d: &mut Duel, c: CardId, rac: u64) -> bool {
    // return c:IsRace(rac) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true,true)
    card::is_race(d, c, rac) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, true)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    // if c==nil then return true end
    let Some(c) = effect::get_handler(d, a.e(0)) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_HAND,0,c,RACE_MACHINE)
    let loc = LOCATION_MZONE | LOCATION_GRAVE | LOCATION_HAND;
    let rg1 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, RACE_MACHINE)), tp, loc, 0, c.into());
    // local rg2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_HAND,0,c,RACE_BEASTWARRIOR)
    let rg2 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, RACE_BEASTWARRIOR)), tp, loc, 0, c.into());
    // local rg=rg1:Clone()
    let mut rg = rg1.clone();
    // rg:Merge(rg2)
    group::add_card(&mut rg, &rg2);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #rg1>0 and #rg2>0
    //     and aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,0)
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= -2 {
        return false;
    }
    if rg1.is_empty() || rg2.is_empty() {
        return false;
    }
    matches!(
        aux::select_unselect_group(d, &rg, Some(a.e(0)), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
        Err(true)
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_HAND,0,c,RACE_MACHINE)
    let loc = LOCATION_MZONE | LOCATION_GRAVE | LOCATION_HAND;
    let mut rg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, RACE_MACHINE)), tp, loc, 0, c.into());
    // rg:Merge(Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE|LOCATION_HAND,0,c,RACE_BEASTWARRIOR))
    let rg2 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, RACE_BEASTWARRIOR)), tp, loc, 0, c.into());
    group::add_card(&mut rg, &rg2);
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(
        d,
        &rg,
        Some(a.e(0)),
        tp,
        Some(2),
        Some(2),
        Some(&rescon),
        1,
        tp,
        Some(HINTMSG_REMOVE as u64),
        None,
        None,
        true,
    )
    .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, a.e(0), Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e(0)).as_group().cloned();
    // if not g then return end
    let Some(g) = g else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
