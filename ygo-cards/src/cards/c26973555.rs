//! Number F0: Utopic Draco Future
use super::prelude::*;

pub const ID: u32 = 26973555;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER], xyz_number: Some(0), listed_names: &[65305468], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz summon procedure
    // Xyz.AddProcedure(c,s.xyzfilter,nil,3,s.ovfilter,aux.Stringid(id,0),nil,nil,false,s.xyzcheck)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(xyzfilter)),
        None,
        3,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        None,
        None,
        false,
        Some(xyz::ex_chk(xyzcheck)),
    );
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Cannot be destroyed by battle or card effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Negate the activation of opponent's monster effect and take control of that monster
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_NEGATE+CATEGORY_CONTROL)
    effect::set_category(d, e4, CATEGORY_NEGATE + CATEGORY_CONTROL);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.discon)
    effect::set_condition(d, e4, Some(condition(discon)));
    // e4:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e4, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e4:SetTarget(s.distg)
    effect::set_target(d, e4, Some(cost_target(distg)));
    // e4:SetOperation(s.disop)
    effect::set_operation(d, e4, Some(operation(disop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.xyzfilter(c,xyz,sumtype,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, xyz: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_XYZ,xyz,sumtype,tp) and not c:IsSetCard(SET_NUMBER)
    card::is_type_ctx(d, c, TYPE_XYZ, xyz, st, Some(tp)) && !card::is_set_card(d, c, &[SET_NUMBER as u16])
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSummonCode(lc,SUMMON_TYPE_XYZ,tp,65305468)
    card::is_faceup(d, c) && card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_XYZ as u64, tp, &[65305468])
}
/// `s.xyzcheck(g,tp,xyz)`
fn xyzcheck(d: &mut Duel, g: &CardSet, _tp: u8, _xyz: CardId) -> bool {
    // return g:GetClassCount(Card.GetRank)==1
    group::get_class_count(d, g, &|d, c| card::get_rank(d, c) as i64) == 1
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and re:IsMonsterEffect() and Duel.IsChainNegatable(ev)
    a.ep == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if rc:IsOnField() and rc:IsRelateToEffect(re) and rc:IsAbleToChangeControler() then
    if card::is_on_field(d, rc) && card::is_relate_to_effect(d, rc, re) && card::is_able_to_change_controler(d, rc) {
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if Duel.NegateActivation(ev) and rc:IsOnField() and rc:IsRelateToEffect(re) and rc:IsAbleToChangeControler() then
    if duel::negate_activation(d, a.ev as u8)
        && card::is_on_field(d, rc)
        && card::is_relate_to_effect(d, rc, re)
        && card::is_able_to_change_controler(d, rc)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.GetControl(rc,tp)
        duel::get_control(d, rc, a.tp, 0, 0, None, None);
    }
}
