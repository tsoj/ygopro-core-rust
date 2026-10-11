//! Sophia, Goddess of Rebirth
use super::prelude::*;

pub const ID: u32 = 4335427;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
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
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_DISABLE_SPSUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_DISABLE_SPSUMMON);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e3, EFFECT_SPSUMMON_CONDITION);
    // e3:SetValue(aux.FALSE)
    effect::set_value_fn(d, e3, aux_false());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // remove
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e4, CATEGORY_REMOVE);
    // e4:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetTarget(s.rmtg)
    effect::set_target(d, e4, Some(cost_target(rmtg)));
    // e4:SetOperation(s.rmop)
    effect::set_operation(d, e4, Some(operation(rmop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spfilter(c,tpe)`
fn spfilter(d: &mut Duel, c: CardId, tpe: u32) -> bool {
    // return c:IsFaceup() and c:IsType(tpe) and c:IsAbleToRemoveAsCost()
    card::is_faceup(d, c) && card::is_type(d, c, tpe) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.chk,1,nil,sg,Group.CreateGroup(),TYPE_RITUAL,TYPE_FUSION,TYPE_SYNCHRO,TYPE_XYZ)
    let chkf = aux::chkf_mmz(1);
    let (ok, _) = chkf(d, sg, e, tp, mg, c);
    let res = ok && group::is_exists(d, sg, &|d, x| chk(d, x, sg, &CardSet::new(), &[TYPE_RITUAL, TYPE_FUSION, TYPE_SYNCHRO, TYPE_XYZ]), 1, NO_EX);
    (res, false)
}
/// `s.chk(c,sg,g,tpe,...)`
fn chk(d: &mut Duel, c: CardId, sg: &CardSet, g: &CardSet, tpes: &[u32]) -> bool {
    let tpe = tpes[0];
    // if not c:IsType(tpe) then return false end
    if !card::is_type(d, c, tpe) {
        return false;
    }
    let rest = &tpes[1..];
    // if ... then
    if !rest.is_empty() {
        // g:AddCard(c)
        let mut g2 = g.clone();
        group::add_card(&mut g2, &CardSet::from_card(c));
        // res=sg:IsExists(s.chk,1,g,sg,g,...)
        let res = group::is_exists(d, sg, &|d, x| chk(d, x, sg, &g2, rest), 1, (&g2).into());
        // g:RemoveCard(c)
        // (the caller's group is not changed: g2 is a copy)
        res
    } else {
        // else res=true end
        true
    }
}
/// `s.sptg`-side tribute check: the Lua `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_RITUAL)
    let g1 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, TYPE_RITUAL)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_FUSION)
    let g2 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, TYPE_FUSION)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local g3=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_SYNCHRO)
    let g3 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, TYPE_SYNCHRO)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local g4=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_XYZ)
    let g4 = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, TYPE_XYZ)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local g=g1:Clone()
    // g:Merge(g2)
    // g:Merge(g3)
    // g:Merge(g4)
    let mut g = g1.clone();
    group::add_card(&mut g, &g2);
    group::add_card(&mut g, &g3);
    group::add_card(&mut g, &g4);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-4 and #g1>0 and #g2>0 and #g3>0 and #g4>0
    //     and aux.SelectUnselectGroup(g,e,tp,4,4,s.rescon,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -4
        && !g1.is_empty()
        && !g2.is_empty()
        && !g3.is_empty()
        && !g4.is_empty()
        && {
            let rs: aux::Rescon = &rescon;
            matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(4), Some(4), Some(rs), 0, tp, None, None, None, false), Err(true))
        }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_RITUAL+TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ)
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, TYPE_RITUAL + TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,4,4,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let rs: aux::Rescon = &rescon;
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(4), Some(4), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #sg==4 then
    if sg.len() == 4 {
        // sg:KeepAlive()
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
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
    let Value::Group(g) = effect::get_label_object(d, e) else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and (c:IsLocation(LOCATION_HAND|LOCATION_SZONE) or aux.SpElimFilter(c,false,true))
    card::is_able_to_remove(d, c, None, None, None)
        && (card::is_location(d, c, LOCATION_HAND | LOCATION_SZONE) || aux::sp_elim_filter(d, c, false, true))
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let loc = LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE;
    let g = duel::get_matching_group(d, Some(&rmfilter), a.tp, loc, loc, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    // Duel.SetChainLimit(aux.FALSE)
    duel::set_chain_limit(d, aux_false());
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let loc = LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE;
    let g = duel::get_matching_group(d, Some(&rmfilter), a.tp, loc, loc, h.into());
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
