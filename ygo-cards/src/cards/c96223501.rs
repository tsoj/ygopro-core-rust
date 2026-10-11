//! Satellarknight Zefrathuban
use super::prelude::*;

pub const ID: u32 = 96223501;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TELLARKNIGHT, SET_ZEFRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // You cannot Pendulum Summon monsters, except "tellarknight" and "Zefra" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c,sump,sumtype) return (sumtype&SUMMON_TYPE_PENDULUM)==SUMMON_TYPE_PENDULUM and not c:IsSetCard({SET_TELLARKNIGHT,SET_ZEFRA}) end)
    effect::set_target(d, e1, Some(lua_fn(e1_target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 other "tellarknight" or "Zefra" card in your Monster Zone or Pendulum Zone and 1 face-up card your opponent controls
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2a, aux::stringid(ID, 0));
    // e2a:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2a, CATEGORY_DESTROY);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetCountLimit(1,id)
    effect::set_count_limit(d, e2a, 1, ID, 0, 0);
    // e2a:SetTarget(s.destg)
    effect::set_target(d, e2a, Some(cost_target(destg)));
    // e2a:SetOperation(s.desop)
    effect::set_operation(d, e2a, Some(operation(desop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // local e2c=e2a:Clone()
    let e2c = effect::clone(d, e2a);
    // e2c:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2c, EVENT_SPSUMMON_SUCCESS);
    // e2c:SetCondition(function(e) return e:GetHandler():IsPendulumSummoned() end)
    effect::set_condition(d, e2c, Some(condition(e2c_cond)));
    // c:RegisterEffect(e2c)
    card::register_effect(d, c, e2c, false);
}
/// `function(e,c,sump,sumtype)` of the first effect
fn e1_target(d: &mut Duel, a: Args) -> bool {
    // (sumtype&SUMMON_TYPE_PENDULUM)==SUMMON_TYPE_PENDULUM and not c:IsSetCard({SET_TELLARKNIGHT,SET_ZEFRA})
    let sumtype = a.int(3) as u64;
    let c = a.c(1);
    (sumtype & SUMMON_TYPE_PENDULUM as u64) == SUMMON_TYPE_PENDULUM as u64
        && !card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16, SET_ZEFRA as u16])
}
/// `s.desfilter(c,e,tp)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return ((c:IsControler(tp) and c:IsSetCard({SET_TELLARKNIGHT,SET_ZEFRA})) or c:IsControler(1-tp)) and c:IsFaceup()
    //     and c:IsCanBeEffectTarget(e)
    ((card::is_controler(d, c, tp) && card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16, SET_ZEFRA as u16])) || card::is_controler(d, c, 1 - tp))
        && card::is_faceup(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `Card.GetControler` as the value of `aux.dpcheck`
fn card_controler_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_controler(d, c) as i64
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE|LOCATION_PZONE,LOCATION_ONFIELD,e:GetHandler(),e,tp)
    let h = effect::get_handler(d, e).nil();
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, e, tp)), tp, LOCATION_MZONE | LOCATION_PZONE, LOCATION_ONFIELD, h.into());
    let dp = aux::dpcheck(card_controler_value);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&dp), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_DESTROY)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&dp), 1, tp, Some(HINTMSG_DESTROY as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tg), 2, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
/// `function(e,c) ... end` of the summon-condition effect e2c
fn e2c_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPendulumSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_pendulum_summoned(d, h)
}
