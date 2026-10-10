//! Number S0: Utopic ZEXAL
use super::prelude::*;

pub const ID: u32 = 52653092;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NUMBER, SET_RANK_UP_MAGIC, SET_UTOPIA],
    xyz_number: Some(0),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,s.xyzfilter,nil,3,s.ovfilter,aux.Stringid(id,0),nil,s.xyzop,false,s.xyzcheck)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(xyzfilter)),
        None,
        3,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        None,
        Some(lua_fn(xyzop)),
        false,
        Some(xyz::ex_chk(xyzcheck)),
    );
    // xyz summon cannot be negated
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_DISABLE_SPSUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_DISABLE_SPSUMMON);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // summon success
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCondition(s.effcon2)
    effect::set_condition(d, e3, Some(condition(effcon2)));
    // e3:SetOperation(s.spsumsuc)
    effect::set_operation(d, e3, Some(operation(spsumsuc)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // atk & def increase
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e4, EFFECT_UPDATE_ATTACK);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetValue(s.atkval)
    effect::set_value_fn(d, e4, effect_card_value(atkval));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e5, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // prevent effects activation
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e6, aux::stringid(ID, 1));
    // e6:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e6, EFFECT_TYPE_QUICK_O);
    // e6:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e6, EVENT_FREE_CHAIN);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetHintTiming(0,TIMING_DRAW_PHASE)
    effect::set_hint_timing(d, e6, 0, Some(TIMING_DRAW_PHASE));
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetCondition(s.actcon)
    effect::set_condition(d, e6, Some(condition(actcon)));
    // e6:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e6, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e6:SetOperation(s.actop)
    effect::set_operation(d, e6, Some(operation(actop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.xyzfilter(c,xyz,sumtype,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, xyz: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_XYZ,xyz,sumtype,tp) and c:IsSetCard(SET_NUMBER,xyz,sumtype,tp)
    card::is_type_ctx(d, c, TYPE_XYZ, xyz, sumtype, Some(tp))
        && card::is_set_card_ctx(d, c, &[SET_NUMBER as u16], xyz, sumtype, Some(tp))
}
/// `s.xyzcheck(g,tp,xyz)`
fn xyzcheck(d: &mut Duel, g: &CardSet, _tp: u8, _xyz: CardId) -> bool {
    // local mg=g:Filter(function(c) return not c:IsHasEffect(EFFECT_EQUIP_SPELL_XYZ_MAT) end,nil)
    let mg = group::filter(d, g, &|d, c| card::is_has_effect(d, c, EFFECT_EQUIP_SPELL_XYZ_MAT, None).is_empty(), NO_EX);
    // return mg:GetClassCount(Card.GetRank)==1
    group::get_class_count(d, &mg, &|d, c| card::get_rank(d, c) as i64) == 1
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RANK_UP_MAGIC) and c:IsNormalSpell() and c:IsDiscardable()
    card::is_set_card(d, c, &[SET_RANK_UP_MAGIC as u16]) && card::is_normal_spell(d, c) && card::is_discardable(d, c, None, None)
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_UTOPIA,lc,SUMMON_TYPE_XYZ,tp)
    card::is_faceup(d, c) && card::is_set_card_ctx(d, c, &[SET_UTOPIA as u16], Some(lc), SUMMON_TYPE_XYZ as u64, Some(tp))
}
/// `s.xyzop(e,tp,chk,mc)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.int(2) == 0 {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local tc=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND,0,nil):SelectUnselect(Group.CreateGroup(),tp,false,Xyz.ProcCancellable)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_HAND, 0, NO_EX);
    let cancellable = xyz::proc_cancellable(d);
    let tc = group::select_unselect(d, &g, Some(&CardSet::new()), tp, false, cancellable, None, None);
    // if tc then
    if let Some(tc) = tc {
        // Duel.SendtoGrave(tc,REASON_DISCARD|REASON_COST)
        duel::send_to_grave(d, tc, REASON_DISCARD | REASON_COST, None, None);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.effcon(e)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_XYZ
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_XYZ
}
/// `s.effcon2(e,tp,eg,ep,ev,re,r,rp)`
fn effcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_XYZ
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_XYZ
}
/// `s.spsumsuc(e,tp,eg,ep,ev,re,r,rp)`
fn spsumsuc(d: &mut Duel, _a: &EffArgs) {
    // Duel.SetChainLimitTillChainEnd(s.chlimit)
    duel::set_chain_limit_till_chain_end(d, lua_fn(chlimit));
}
/// `s.chlimit(e,ep,tp)`
fn chlimit(_d: &mut Duel, a: Args) -> bool {
    // return tp==ep
    a.p(2) == a.p(1)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetOverlayCount()*1000
    (card::get_overlay_count(d, c) * 1000) as i64
}
/// `s.actcon(e,tp,eg,ep,ev,re,r,rp)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.actop(e,tp,eg,ep,ev,re,r,rp)`
fn actop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
