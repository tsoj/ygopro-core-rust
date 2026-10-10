//! D-HERO Dogma
//! Ｄ－ＨＥＲＯ ドグマガイ
use super::prelude::*;

pub const ID: u32 = 17132130;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DESTINY_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your hand) by Tributing 3 monsters, including at least 1 "Destiny HERO" monster
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND)
    effect::set_range(d, e0, LOCATION_HAND);
    // e0:SetCondition(s.spcon)
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    // e0:SetTarget(s.sptg)
    effect::set_target(d, e0, Some(lua_fn(sptg)));
    // e0:SetOperation(s.spop)
    effect::set_operation(d, e0, Some(lua_fn(spop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Once per turn, if you still control this face-up card during your opponent's next Standby Phase after you Special Summoned it this way: Halve your opponent's LP
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_STANDBY);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) and e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetOperation(function(e,tp) local opp=1-tp Duel.SetLP(opp,Duel.GetLP(opp)/2) end)
    effect::set_operation(d, e1, Some(operation(e1_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0 and sg:IsExists(Card.IsSetCard,1,nil,SET_DESTINY_HERO)
    let res = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0
        && group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_DESTINY_HERO as u16]), 1, NO_EX);
    (res, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(_c) = a.card(1) else {
        return true;
    };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local g=Duel.GetMatchingGroup(Card.IsReleasable,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_releasable(d, c, None, None)), tp, LOCATION_MZONE, 0, NO_EX);
    // return #g>=3 and aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,0)
    if g.len() < 3 {
        return false;
    }
    let rs: aux::Rescon = &rescon;
    matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(rs), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=Duel.GetMatchingGroup(Card.IsReleasable,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_releasable(d, c, None, None)), tp, LOCATION_MZONE, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_RELEASE,nil,nil,true)
    let rs: aux::Rescon = &rescon;
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true).unwrap_or_default();
    // if sg and #sg>0 then
    if !sg.is_empty() {
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
    // local sg=e:GetLabelObject()
    let sg = effect::get_label_object(d, e).as_group().cloned();
    // if sg and #sg==3 then
    if let Some(sg) = sg {
        if sg.len() == 3 {
            // Duel.Release(sg,REASON_COST)
            duel::release(d, sg, REASON_COST, None);
            // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~(RESET_TOFIELD|RESET_TEMP_REMOVE|RESET_LEAVE))|RESET_PHASE|PHASE_STANDBY,0,1)
            let c = a.c(8);
            let reset = RESET_EVENT | (RESETS_STANDARD & !(RESET_TOFIELD | RESET_TEMP_REMOVE | RESET_LEAVE)) | RESET_PHASE | PHASE_STANDBY;
            card::register_flag_effect(d, c, ID, reset, 0, 1, 0, 0);
        }
    }
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) and e:GetHandler():HasFlagEffect(id) end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && {
        let h = effect::get_handler(d, a.e).nil();
        card::has_flag_effect(d, h, ID, None)
    }
}
/// `function(e,tp) local opp=1-tp Duel.SetLP(opp,Duel.GetLP(opp)/2) end`
fn e1_op(d: &mut Duel, a: &EffArgs) {
    // local opp=1-tp
    let opp = 1 - a.tp;
    // Duel.SetLP(opp,Duel.GetLP(opp)/2)
    let lp = duel::get_lp(d, opp);
    duel::set_lp(d, opp, lua_int(lp as f64 / 2.0) as i32);
}
