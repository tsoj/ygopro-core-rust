//! Valkyrion the Magna Warrior
use super::prelude::*;

pub const ID: u32 = 75347539;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[99785935, 39256679, 11549357],
    ..ScriptInfo::DEFAULT
};

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
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.chk(c,sg,g,code,...)`: the material `c` for `code`, and the other codes matched by the rest of `sg`
fn chk(d: &mut Duel, c: CardId, sg: &CardSet, g: &CardSet, codes: &[u32]) -> bool {
    // if not c:IsCode(code) then return false end
    if !card::is_code(d, c, &[codes[0]]) {
        return false;
    }
    // if ... then
    if codes.len() > 1 {
        // g:AddCard(c)
        let mut g2 = g.clone();
        group::add_card(&mut g2, &CardSet::from_card(c));
        // res=sg:IsExists(s.chk,1,g,sg,g,...)
        group::is_exists(d, sg, &|d, x| chk(d, x, sg, &g2, &codes[1..]), 1, (&g2).into())
        // g:RemoveCard(c)  (g2 was a copy, so the caller's g is unchanged)
    } else {
        // else res=true end
        true
    }
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.chk,1,nil,sg,Group.CreateGroup(),99785935,39256679,11549357)
    let (mmz, _) = aux::chkf_mmz(1)(d, sg, e, tp, mg, None);
    let res = mmz && group::is_exists(d, sg, &|d, c| chk(d, c, sg, &CardSet::new(), &[99785935, 39256679, 11549357]), 1, NO_EX);
    (res, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetReleaseGroup(tp,true)
    let rg = duel::get_release_group(d, tp, true, false, None);
    // local g1=rg:Filter(Card.IsCode,nil,99785935)
    let g1 = group::filter(d, &rg, &|d, x| card::is_code(d, x, &[99785935]), NO_EX);
    // local g2=rg:Filter(Card.IsCode,nil,39256679)
    let g2 = group::filter(d, &rg, &|d, x| card::is_code(d, x, &[39256679]), NO_EX);
    // local g3=rg:Filter(Card.IsCode,nil,11549357)
    let g3 = group::filter(d, &rg, &|d, x| card::is_code(d, x, &[11549357]), NO_EX);
    // local g=g1:Clone()
    let mut g = g1.clone();
    // g:Merge(g2)
    group::add_card(&mut g, &g2);
    // g:Merge(g3)
    group::add_card(&mut g, &g3);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-3 and #g1>0 and #g2>0 and #g3>0
    //     and aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -3
        && !g1.is_empty()
        && !g2.is_empty()
        && !g3.is_empty()
        && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetReleaseGroup(tp,true):Filter(Card.IsCode,nil,99785935,39256679,11549357)
    let rg0 = duel::get_release_group(d, tp, true, false, None);
    let rg = group::filter(d, &rg0, &|d, x| card::is_code(d, x, &[99785935, 39256679, 11549357]), NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,3,3,s.rescon,1,tp,HINTMSG_RELEASE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true).unwrap_or_default();
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
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // g:DeleteGroup()
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    // if chk==0 then return ft>2 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,99785935)
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,39256679)
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,11549357) end
    if a.chk == Some(0) {
        return ft > 2
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, 99785935)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, 39256679)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, 11549357)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g1=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,99785935)
    let mut g1 = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, 99785935)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g2=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,39256679)
    let g2 = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, 39256679)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g3=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,11549357)
    let g3 = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, 11549357)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // g1:Merge(g3)
    group::add_card(&mut g1, &g3);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g1,3,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g1), 3, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if #g~=3 or ft<3 then return end
    if g.len() != 3 || ft < 3 {
        return;
    }
    // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
