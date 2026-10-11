//! Gaia Stream, the Graceful Force
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 13289758;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 6 monsters OR 1 Rank 5 or 7 Xyz Monster you control
    // Xyz.AddProcedure(c,nil,6,2,s.ovfilter,aux.Stringid(id,0),Xyz.InfiniteMats,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(6),
        2,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::INFINITE_MATS),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // Cannot be used as material for an Xyz Summon the turn it was Xyz Summoned
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_CANNOT_BE_XYZ_MATERIAL)
    effect::set_code(d, e0, EFFECT_CANNOT_BE_XYZ_MATERIAL);
    // e0:SetCondition(s.matxyzcond)
    effect::set_condition(d, e0, Some(condition(matxyzcond)));
    // e0:SetValue(1)
    effect::set_value(d, e0, 1);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Cannot attack directly
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains ATK equal to the combined Levels/Ranks of its attached materials x 200
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Detach 1 material from this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
    // e3:SetOperation(s.detachop)
    effect::set_operation(d, e3, Some(operation(detachop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsRank(5,7) and c:IsType(TYPE_XYZ,lc,SUMMON_TYPE_XYZ,tp) and c:IsFaceup()
    card::is_rank(d, c, &[5, 7])
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(lc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_faceup(d, c)
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) end
    if a.int(2) == 0 {
        return !duel::has_flag_effect(d, tp, ID, None);
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0);
    // return true
    true
}
/// `s.matxyzcond(e)`
fn matxyzcond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsStatus(STATUS_SPSUMMON_TURN) and c:IsXyzSummoned()
    card::is_status(d, c, STATUS_SPSUMMON_TURN) && card::is_xyz_summoned(d, c)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local g=e:GetHandler():GetOverlayGroup():Filter(aux.OR(Card.HasLevel,Card.HasRank),nil)
    let h = effect::get_handler(d, e).nil();
    let overlay = card::get_overlay_group(d, h);
    let g = group::filter(d, &overlay, &|d, x| card::has_level(d, x) || card::has_rank(d, x), NO_EX);
    // return 200*(g:GetSum(Card.GetLevel)+g:GetSum(Card.GetRank))
    let levels = group::get_sum(d, &g, &|d, x| card::get_level(d, x) as i64);
    let ranks = group::get_sum(d, &g, &|d, x| card::get_rank(d, x) as i64);
    200 * (levels + ranks)
}
/// `s.detachop(e,tp,eg,ep,ev,re,r,rp)`
fn detachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:GetOverlayCount()>0 then
    if card::is_relate_to_effect(d, c, a.e) && card::get_overlay_count(d, c) > 0 {
        // c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)
        card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT);
    }
}
