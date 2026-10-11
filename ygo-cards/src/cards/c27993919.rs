//! LaSolfachord Angelia
use super::prelude::*;

pub const ID: u32 = 27993919;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SOLFACHORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum procedure
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Prevent the activation of Spell/Traps or monster effects when you Pendulum Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetOperation(s.sucop)
    effect::set_operation(d, e1, Some(operation(sucop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Solfachord" monster from the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Prevent the activation of Spell/Traps or monster effects when "Solfachord" monsters attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.btcon)
    effect::set_condition(d, e3, Some(condition(btcon)));
    // e3:SetOperation(s.btop)
    effect::set_operation(d, e3, Some(operation(btop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.sucfilter(c,tp)`
fn sucfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SOLFACHORD) and c:IsType(TYPE_PENDULUM) and c:IsControler(tp) and c:IsPendulumSummoned()
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_controler(d, c, tp)
        && card::is_pendulum_summoned(d, c)
}
/// `s.sucop(e,tp,eg,ep,ev,re,r,rp)`
fn sucop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(s.sucfilter,1,nil,tp) then
    if group::is_exists(d, &a.eg.clone().nil(), &|d, c| sucfilter(d, c, a.tp), 1, NO_EX) {
        // Duel.SetChainLimitTillChainEnd(s.chainlm)
        duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
    }
}
/// `s.chainlm(e,rp,tp)`
fn chainlm(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return tp==rp or (e:IsSpellTrapEffect() and not e:IsHasType(EFFECT_TYPE_ACTIVATE))
    a.int(2) == a.int(1) || (effect::is_spell_trap_effect(d, e) && !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE))
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SOLFACHORD) and c:IsType(TYPE_PENDULUM) and c:IsReleasableByEffect()
    //     and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetLeftScale())
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_releasable_by_effect(d, c, None, None)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && {
            let sc = card::get_left_scale(d, c) as i64;
            duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, sc)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
/// `s.spfilter(c,e,tp,sc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sc: i64) -> bool {
    // return c:IsSetCard(SET_SOLFACHORD) and c:IsType(TYPE_PENDULUM) and math.abs(c:GetLeftScale()-sc)==2
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && (card::get_left_scale(d, c) as i64 - sc).abs() == 2
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && !card::is_code(d, c, &[ID])
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroup(tp,s.cfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        let (e, tp) = (a.e, a.tp);
        return duel::check_release_group(d, a.tp, Some(&|d, c| cfilter(d, c, e, tp)), 1, duel::ReleaseArgs::default(), NO_EX, false);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local rg=Duel.SelectReleaseGroup(tp,s.cfilter,1,1,nil,e,tp)
    let rg = duel::select_release_group(d, tp, Some(&|d, c| cfilter(d, c, e, tp)), 1, 1, duel::ReleaseArgs::default(), NO_EX, false);
    // if rg and Duel.Release(rg,REASON_EFFECT)>0 then
    if let Some(rg) = rg {
        if duel::release(d, rg.clone(), REASON_EFFECT, None) > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,rg:GetFirst():GetLeftScale())
            let first = group::get_first(d, &rg).nil();
            let sc = card::get_left_scale(d, first) as i64;
            let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, sc)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
/// `s.btcon(e,tp,eg,ep,ev,re,r,rp)`
fn btcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ac=Duel.GetAttacker()
    // return ac and ac:IsControler(tp) and ac:IsType(TYPE_PENDULUM) and ac:IsSetCard(SET_SOLFACHORD)
    match duel::get_attacker(d) {
        Some(ac) => {
            card::is_controler(d, ac, a.tp)
                && card::is_type(d, ac, TYPE_PENDULUM)
                && card::is_set_card(d, ac, &[SET_SOLFACHORD as u16])
        }
        None => false,
    }
}
/// `s.btop(e,tp,eg,ep,ev,re,r,rp)`
fn btop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetCondition(s.actcon)
    effect::set_condition(d, e1, Some(lua_fn(actcon)));
    // e1:SetValue(s.actlimit)
    effect::set_value_fn(d, e1, lua_fn(actlimit));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsOddScale),e:GetHandlerPlayer(),LOCATION_PZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_odd_scale(d, c))), p, LOCATION_PZONE, 0, 1, NO_EX)
}
/// `s.actlimit(e,re,tp)`
fn actlimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsSpellTrapEffect()
    effect::is_spell_trap_effect(d, a.e(1))
}
