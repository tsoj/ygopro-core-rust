//! Number 100: Numeron Dragon
use super::prelude::*;

pub const ID: u32 = 57314798;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(100), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,s.xyzfilter,nil,2,nil,nil,nil,nil,false,s.xyzcheck)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(xyzfilter)),
        None,
        2,
        None,
        None,
        None,
        None,
        false,
        Some(xyz::ex_chk(xyzcheck)),
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Gains ATK equal to the combined Ranks of all Xyz monsters on the field x 1000
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy as many monsters on the field as possible and each player sets 1 Spell/Trap from the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(function(e) return e:GetHandler():IsReason(REASON_EFFECT) end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon this card from the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.xyzfilter(c,xyz,sumtype,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, xyz: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_XYZ,xyz,sumtype,tp) and c:IsSetCard(SET_NUMBER,xyz,sumtype,tp)
    card::is_type_ctx(d, c, TYPE_XYZ, xyz, sumtype, Some(tp)) && card::is_set_card_ctx(d, c, &[SET_NUMBER as u16], xyz, sumtype, Some(tp))
}
/// `s.xyzcheck(g,tp,xyz)`
fn xyzcheck(d: &mut Duel, g: &CardSet, _tp: u8, _xyz: CardId) -> bool {
    // local mg=g:Filter(function(c) return not c:IsHasEffect(EFFECT_EQUIP_SPELL_XYZ_MAT) end,nil)
    let mg = group::filter(d, g, &|d, c| card::is_has_effect(d, c, EFFECT_EQUIP_SPELL_XYZ_MAT, None).is_empty(), NO_EX);
    // return mg:GetClassCount(Card.GetRank)==1 and mg:GetClassCount(Card.GetCode)==1
    group::get_class_count(d, &mg, &|d, c| card::get_rank(d, c) as i64) == 1
        && group::get_class_count(d, &mg, &|d, c| card::get_code(d, c)[0] as i64) == 1
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:GetRank()>0
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::get_rank(d, c) > 0
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsFaceup() or not c:IsRelateToEffect(e) then return end
    if !card::is_faceup(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local atk=g:GetSum(Card.GetRank)
    let atk = group::get_sum(d, &g, &|d, c| card::get_rank(d, c) as i64);
    // if atk<=0 then return end
    if atk <= 0 {
        return;
    }
    // --Gain 1000 ATK per Rank
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(atk*1000)
    effect::set_value(d, e1, atk * 1000);
    // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END|RESET_OPPO_TURN)
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END | RESET_OPPO_TURN, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.setfilter(c,tp)`
fn setfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSpellTrap() and c:IsSSetable(false,tp)
    card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, Some(tp))
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,2,PLAYER_ALL,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 2, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if Duel.Destroy(g,REASON_EFFECT)==0 then return end
    if duel::destroy(d, g, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local tc1=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let sel1 = duel::select_matching_card(d, tp, Some(&|d, c| setfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc1 = group::get_first(d, &sel1);
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SET as u64);
    // local tc2=Duel.SelectMatchingCard(1-tp,s.setfilter,1-tp,LOCATION_GRAVE,0,1,1,nil,1-tp):GetFirst()
    let sel2 = duel::select_matching_card(d, 1 - tp, Some(&|d, c| setfilter(d, c, 1 - tp)), 1 - tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc2 = group::get_first(d, &sel2);
    // if (tc1 and tc1:IsHasEffect(EFFECT_NECRO_VALLEY)) or (tc2 and tc2:IsHasEffect(EFFECT_NECRO_VALLEY)) then return end
    let necro1 = tc1.is_some_and(|c| !card::is_has_effect(d, c, EFFECT_NECRO_VALLEY, None).is_empty());
    let necro2 = tc2.is_some_and(|c| !card::is_has_effect(d, c, EFFECT_NECRO_VALLEY, None).is_empty());
    if necro1 || necro2 {
        return;
    }
    // if tc1 then Duel.SSet(tp,tc1) end
    if let Some(tc1) = tc1 {
        duel::s_set(d, tp, tc1, None, true);
    }
    // if tc2 then Duel.SSet(1-tp,tc2) end
    if let Some(tc2) = tc2 {
        duel::s_set(d, 1 - tp, tc2, None, true);
    }
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsStatus(STATUS_LEAVE_CONFIRMED)
    !card::is_status(d, c, STATUS_LEAVE_CONFIRMED)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()==nil
    card::is_controler(d, duel::get_attacker(d).nil(), 1 - a.tp)
        && duel::get_attack_target(d).is_none()
        // and not Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_ONFIELD|LOCATION_HAND,0,1,nil)
        && !duel::is_existing_matching_card(d, Some(&spfilter), a.tp, LOCATION_ONFIELD | LOCATION_HAND, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `function(e) return e:GetHandler():IsReason(REASON_EFFECT) end`
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_reason(d, c, REASON_EFFECT)
}
