//! Number F0: Utopic Future Slash
use super::prelude::*;

pub const ID: u32 = 43490025;

pub const INFO: ScriptInfo = ScriptInfo {
    xyz_number: Some(0),
    listed_series: &[SET_NUMBER, SET_UTOPIA],
    listed_names: &[65305468],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Xyz Monsters with the same Rank, except "Number" monsters OR 1 "Utopia" monster or "Number F0: Utopic Future" you control
    // Xyz.AddProcedure(c,s.xyzfilter,nil,2,s.ovfilter,aux.Stringid(id,0),nil,nil,false,s.xyzcheck)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, xyz, sumtype, tp| xyzfilter(d, x, xyz, sumtype, tp))),
        None,
        2,
        Some(crate::procs::xyz::alter_filter(|d, x, tp, xyzc| ovfilter(d, x, tp, xyzc))),
        Some(aux::stringid(ID, 0)),
        None,
        None,
        false,
        Some(crate::procs::xyz::ex_chk(|d, g, tp, xyz| xyzcheck(d, g, tp, xyz))),
    );
    // Gains 500 ATK for each "Number" Xyz Monster in the GYs
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return Duel.GetMatchingGroupCount(function(c) return c:IsSetCard(SET_NUMBER) and c:IsType(TYPE_XYZ) end,0,LOCATION_GRAVE,LOCATION_GRAVE,nil)*500 end)
    effect::set_value_fn(d, e1, effect_value(|d, _e| {
        duel::get_matching_group_count(d, Some(&|d, x| card::is_set_card(d, x, &[SET_NUMBER as u16]) && card::is_type(d, x, TYPE_XYZ)), 0, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX) as i64 * 500
    }));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by battle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Make this card able to make a second attack during each Battle Phase this turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function() return Duel.IsAbleToEnterBP() end)
    effect::set_condition(d, e3, Some(condition(bp_cond)));
    // e3:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e3:SetTarget(s.secondatktg)
    effect::set_target(d, e3, Some(cost_target(secondatktg)));
    // e3:SetOperation(s.secondatkop)
    effect::set_operation(d, e3, Some(operation(secondatkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.xyzfilter(c,xyz,sumtype,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, xyz: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_XYZ,xyz,sumtype,tp) and not c:IsSetCard(SET_NUMBER,xyz,sumtype,tp)
    card::is_type_ctx(d, c, TYPE_XYZ, xyz, sumtype, Some(tp))
        && !card::is_set_card_ctx(d, c, &[SET_NUMBER as u16], xyz, sumtype, Some(tp))
}
/// `s.xyzcheck(g,tp,xyz)`
fn xyzcheck(d: &mut Duel, g: &CardSet, _tp: u8, _xyz: CardId) -> bool {
    // local mg=g:Filter(function(c) return not c:IsHasEffect(EFFECT_EQUIP_SPELL_XYZ_MAT) end,nil)
    let mg = group::filter(d, g, &|d, c| card::is_has_effect(d, c, EFFECT_EQUIP_SPELL_XYZ_MAT, None).is_empty(), NO_EX);
    // return mg:GetClassCount(Card.GetRank)==1
    group::get_class_count(d, &mg, &|d, c| card::get_rank(d, c) as i64) == 1
}
/// `s.ovfilter(c,tp,xyzc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return (c:IsSetCard(SET_UTOPIA,xyzc,SUMMON_TYPE_XYZ,tp) or c:IsSummonCode(xyzc,SUMMON_TYPE_XYZ,tp,65305468)) and c:IsFaceup()
    (card::is_set_card_ctx(d, c, &[SET_UTOPIA as u16], Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        || card::is_summon_code(d, c, Some(xyzc), SUMMON_TYPE_XYZ as u64, tp, &[65305468]))
        && card::is_faceup(d, c)
}
/// `function() return Duel.IsAbleToEnterBP() end`
fn bp_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_able_to_enter_bp(d)
}
/// `s.secondatktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn secondatktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // return c:CanAttack() and c:GetEffectCount(EFFECT_EXTRA_ATTACK)==0
        return card::can_attack(d, c) && card::get_effect_count(d, c, EFFECT_EXTRA_ATTACK) == 0;
    }
    true
}
/// `s.secondatkop(e,tp,eg,ep,ev,re,r,rp)`
fn secondatkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // This card can make a second attack during each Battle Phase this turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3201)
        effect::set_description(d, e1, 3201);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
