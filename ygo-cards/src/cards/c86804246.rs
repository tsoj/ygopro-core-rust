//! Super Crashbug
use super::prelude::*;

pub const ID: u32 = 86804246;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[87526784, 23915499, 50319138, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // There can only be 1 face-up "Super Crashbug" on the field
    // c:SetUniqueOnField(1,1,id)
    card::set_unique_on_field(d, c, 1, 1, card::UniqueCode::Code(ID), None);
    // Must first be Special Summoned (from your hand) in face-up Defense Position, by banishing "Crashbug X", "Crashbug Y", and "Crashbug Z" from your GY
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
    // e0:SetRange(LOCATION_HAND)
    effect::set_range(d, e0, LOCATION_HAND);
    // e0:SetTargetRange(POS_FACEUP_DEFENSE,0)
    effect::set_target_range(d, e0, POS_FACEUP_DEFENSE, 0);
    // e0:SetCondition(s.spcon)
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    // e0:SetTarget(s.sptg)
    effect::set_target(d, e0, Some(cost_target(sptg)));
    // e0:SetOperation(s.spop)
    effect::set_operation(d, e0, Some(operation(spop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Switch the ATK and DEF of all face-up Attack Position monsters on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SWAP_AD)
    effect::set_code(d, e1, EFFECT_SWAP_AD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(function(e,c) return c:IsPosition(POS_FACEUP_ATTACK) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_position(d, c, POS_FACEUP_ATTACK))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spcostfilter(c)`
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(87526784,23915499,50319138) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_code(d, c, &[87526784, 23915499, 50319138]) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0 and sg:GetClassCount(Card.GetCode)==3
    let res = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0
        && group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == 3;
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
    // local rg=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&spcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // return #rg>=3 and Duel.GetMZoneCount(tp,rg)>0 and aux.SelectUnselectGroup(rg,e,tp,3,3,s.rescon,0)
    if rg.len() < 3 {
        return false;
    }
    if duel::get_m_zone_count(d, tp, (&rg).into(), None, None, None).0 <= 0 {
        return false;
    }
    let rs: aux::Rescon = &rescon;
    matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(rs), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local rg=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&spcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,3,3,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let rs: aux::Rescon = &rescon;
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive() does nothing in Rust (groups are values)
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, a.e) else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup() does nothing in Rust (groups are values)
}
