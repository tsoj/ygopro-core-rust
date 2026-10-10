//! Gate Guardian
use super::prelude::*;

pub const ID: u32 = 25833572;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SANGA_OF_THE_THUNDER, CARD_KAZEJIN, CARD_SUIJIN], ..ScriptInfo::DEFAULT };

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
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.chk,1,nil,sg,Group.CreateGroup(),CARD_SANGA_OF_THE_THUNDER,CARD_KAZEJIN,CARD_SUIJIN)
    let (ok, _) = aux::chkf_mmz(1)(d, sg, e, tp, mg, c);
    let codes = [CARD_SANGA_OF_THE_THUNDER, CARD_KAZEJIN, CARD_SUIJIN];
    (ok && group::is_exists(d, sg, &|d, x| {
        let mut g = CardSet::new();
        chk(d, x, sg, &mut g, &codes)
    }, 1, NO_EX), false)
}
/// `s.chk(c,sg,g,code,...)`: `c` has the first code, and the other codes (`...`) can be matched by
/// other cards of `sg` that are not in `g`.
fn chk(d: &mut Duel, c: CardId, sg: &CardSet, g: &mut CardSet, codes: &[u32]) -> bool {
    // if not c:IsCode(code) then return false end
    if !card::is_code(d, c, &[codes[0]]) {
        return false;
    }
    // if ... then
    if codes.len() > 1 {
        // g:AddCard(c)
        group::add_card(g, &CardSet::from_card(c));
        // res=sg:IsExists(s.chk,1,g,sg,g,...)
        // (the cards of g are left out: sg minus g)
        let mut cand = sg.clone();
        group::remove_card(&mut cand, g);
        let res = group::is_exists(d, &cand, &|d, x| {
            let mut g2 = g.clone();
            chk(d, x, sg, &mut g2, &codes[1..])
        }, 1, NO_EX);
        // g:RemoveCard(c)
        group::remove_card(g, &CardSet::from_card(c));
        // return res
        res
    } else {
        // res=true
        true
    }
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
    // local rg=Duel.GetReleaseGroup(tp)
    let rg = duel::get_release_group(d, tp, false, false, None);
    // local g1=rg:Filter(Card.IsCode,nil,CARD_SANGA_OF_THE_THUNDER)
    let g1 = group::filter(d, &rg, &|d, c| card::is_code(d, c, &[CARD_SANGA_OF_THE_THUNDER]), NO_EX);
    // local g2=rg:Filter(Card.IsCode,nil,CARD_KAZEJIN)
    let g2 = group::filter(d, &rg, &|d, c| card::is_code(d, c, &[CARD_KAZEJIN]), NO_EX);
    // local g3=rg:Filter(Card.IsCode,nil,CARD_SUIJIN)
    let g3 = group::filter(d, &rg, &|d, c| card::is_code(d, c, &[CARD_SUIJIN]), NO_EX);
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
    // local rg=Duel.GetReleaseGroup(tp)
    let rg = duel::get_release_group(d, tp, false, false, None);
    // local g1=rg:Filter(Card.IsCode,nil,CARD_SANGA_OF_THE_THUNDER)
    let mut g1 = group::filter(d, &rg, &|d, c| card::is_code(d, c, &[CARD_SANGA_OF_THE_THUNDER]), NO_EX);
    // local g2=rg:Filter(Card.IsCode,nil,CARD_KAZEJIN)
    let g2 = group::filter(d, &rg, &|d, c| card::is_code(d, c, &[CARD_KAZEJIN]), NO_EX);
    // local g3=rg:Filter(Card.IsCode,nil,CARD_SUIJIN)
    let g3 = group::filter(d, &rg, &|d, c| card::is_code(d, c, &[CARD_SUIJIN]), NO_EX);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // g1:Merge(g3)
    group::add_card(&mut g1, &g3);
    // local g1=aux.SelectUnselectGroup(g1,e,tp,3,3,s.rescon,1,tp,HINTMSG_RELEASE,s.rescon,nil,true)
    let g1 = aux::select_unselect_group(d, &g1, Some(e), tp, Some(3), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_RELEASE as u64), Some(&rescon), None, true)
        .unwrap_or_default();
    // if #g1>0 then
    if !g1.is_empty() {
        // g1:KeepAlive() (groups are values in Rust)
        // e:SetLabelObject(g1)
        effect::set_label_object(d, e, Value::Group(g1));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g1=e:GetLabelObject()
    // if not g1 then return end
    let Value::Group(g1) = effect::get_label_object(d, e) else {
        return;
    };
    // Duel.Release(g1,REASON_COST)
    duel::release(d, g1, REASON_COST, None);
    // g1:DeleteGroup() (nothing in Rust)
}
