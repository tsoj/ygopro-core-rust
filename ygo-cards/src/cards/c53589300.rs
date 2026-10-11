//! Nerva the Power Patron of Creation
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 53589300;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARTMAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 3 "Artmage" monsters
    // Fusion.AddProcMixRep(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ARTMAGE),3,3)
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_ARTMAGE as u16], Some(fc), st, Some(tp)))),
        3,
        3,
        vec![],
    );
    // c:AddMustFirstBeFusionSummoned()
    card::add_must_first_be_fusion_summoned(d, c);
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Must first be either Fusion Summoned, or Special Summoned (from your Extra Deck) in Defense Position by Tributing 3 monsters with different Types
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SPSUM_PARAM)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SPSUM_PARAM, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetTargetRange(POS_FACEUP_DEFENSE,0)
    effect::set_target_range(d, e0, POS_FACEUP_DEFENSE, 0);
    // e0:SetCondition(s.spcon)
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    // e0:SetTarget(s.sptg)
    effect::set_target(d, e0, Some(lua_fn(sptg)));
    // e0:SetOperation(s.spop)
    effect::set_operation(d, e0, Some(lua_fn(spop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Cannot be destroyed by card effects while a card is in the Field Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function() return Duel.GetFieldGroupCount(0,LOCATION_FZONE,LOCATION_FZONE)>0 end)
    effect::set_condition(d, e1, Some(lua_fn(e1_cond)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make your "Artmage" monster's effect become "Destroy all cards your opponent controls"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.chcon)
    effect::set_condition(d, e2, Some(condition(chcon)));
    // e2:SetTarget(s.chtg)
    effect::set_target(d, e2, Some(cost_target(chtg)));
    // e2:SetOperation(s.chop)
    effect::set_operation(d, e2, Some(operation(chop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rescon(sg,e,tp)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetLocationCountFromEx(tp,tp,sg,e:GetHandler())>0 and sg:GetClassCount(Card.GetRace)==#sg
    let h = effect::get_handler(d, e.nil()).nil();
    let loc = duel::get_location_count_from_ex(d, tp, Some(tp), sg.into(), duel::ExtraCard::Card(h), None).0 > 0;
    let res = loc && group::get_class_count(d, sg, &|d, c| card::get_race(d, c) as i64) == sg.len();
    (res, false)
}
/// `s.spcon(e,c)` (`c` is argument 1)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetReleaseGroup(tp)
    let rg = duel::get_release_group(d, tp, false, false, None);
    // return aux.SelectUnselectGroup(rg,e,tp,3,3,s.rescon,0)
    let e = a.e(0);
    matches!(
        aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(&rescon_fn), 0, tp, None, None, None, false),
        Err(true)
    )
}
/// The `s.rescon` of `aux.SelectUnselectGroup` (`sg,e,tp,mg`)
fn rescon_fn(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    rescon(d, sg, e, tp, mg, c)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)` (`EFFECT_SPSUMMON_PROC` target; `c` is argument 9)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    let e = a.e(0);
    // local rg=Duel.GetReleaseGroup(tp)
    let rg = duel::get_release_group(d, tp, false, false, None);
    // local mg=aux.SelectUnselectGroup(rg,e,tp,3,3,s.rescon,1,tp,HINTMSG_RELEASE,nil,nil,true)
    let mg = aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(&rescon_fn), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true)
        .unwrap_or_default();
    // if #mg==3 then
    if mg.len() == 3 {
        // mg:KeepAlive()
        // e:SetLabelObject(mg)
        effect::set_label_object(d, e, Value::Group(mg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)` (`EFFECT_SPSUMMON_PROC` operation)
fn spop(d: &mut Duel, a: Args) -> Value {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e(0)).as_group().cloned();
    // if not g then return end
    let Some(g) = g else {
        return Value::Nil;
    };
    // Duel.Release(g,REASON_COST|REASON_MATERIAL)
    duel::release(d, g, REASON_COST | REASON_MATERIAL, None);
    // g:DeleteGroup()
    Value::Nil
}
/// `s.chcon(e,tp,eg,ep,ev,re,r,rp)`
fn chcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==tp and re:IsMonsterEffect() and Chain.IsSetcode(ev,SET_ARTMAGE)
    a.rp == a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && chain::is_triggering(d, a.ev as u8, chain::Prop::SetCard, &[SET_ARTMAGE as i64]).truthy()
}
/// `s.chtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn chtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) > 0;
    }
    true
}
/// `s.chop(e,tp,eg,ep,ev,re,r,rp)`
fn chop(d: &mut Duel, a: &EffArgs) {
    // local g=Group.CreateGroup()
    let g = CardSet::new();
    // Duel.ChangeTargetCard(ev,g)
    duel::change_target_card(d, a.ev as u8, &g);
    // Duel.ChangeChainOperation(ev,s.desop)
    duel::change_chain_operation(d, a.ev as u8, lua_fn(chain_destroy));
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`: the operation that replaces the chain's operation
fn chain_destroy(d: &mut Duel, a: Args) -> Value {
    let tp = a.p(1);
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
    Value::Nil
}
/// `function() return Duel.GetFieldGroupCount(0,LOCATION_FZONE,LOCATION_FZONE)>0 end`
fn e1_cond(d: &mut Duel, _a: Args) -> bool {
    duel::get_field_group_count(d, 0, LOCATION_FZONE, LOCATION_FZONE) > 0
}
