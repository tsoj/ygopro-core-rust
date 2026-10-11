//! Gimmick Puppet Gigantes Doll
use super::prelude::*;

pub const ID: u32 = 7593748;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GIMMICK_PUPPET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,aux.FilterBoolFunction(Card.IsSetCard,SET_GIMMICK_PUPPET),4,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, _sc, _st, _tp| card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16]))),
        Some(4),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(2))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(2), None, None)));
    // e1:SetTarget(s.cttg)
    effect::set_target(d, e1, Some(cost_target(cttg)));
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // level
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsAbleToChangeControler() and chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_able_to_change_controler(d, chkc) && card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return ft>0 and Duel.IsExistingTarget(Card.IsAbleToChangeControler,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_target(d, Some(&|d, c| card::is_able_to_change_controler(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local ct=math.min(ft,2)
    let ct = ft.min(2);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToChangeControler,tp,0,LOCATION_MZONE,1,ct,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_change_controler(d, c)), a.tp, 0, LOCATION_MZONE, 1, ct as u16, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,#g,0,0)
    let n = g.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, n, 0, 0);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ge1=Effect.CreateEffect(c)
    let ge1 = effect::create_effect(d, c);
    // ge1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, ge1, EFFECT_TYPE_FIELD);
    // ge1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, ge1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // ge1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, ge1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // ge1:SetTargetRange(1,0)
    effect::set_target_range(d, ge1, 1, 0);
    // ge1:SetTarget(s.splimit)
    effect::set_target(d, ge1, Some(lua_fn(splimit)));
    // ge1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, ge1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(ge1,tp)
    duel::register_effect(d, ge1, a.tp);
    // local ge2=Effect.CreateEffect(c)
    let ge2 = effect::create_effect(d, c);
    // ge2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, ge2, EFFECT_TYPE_FIELD);
    // ge2:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, ge2, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // ge2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, ge2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // ge2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, ge2, LOCATION_MZONE, 0);
    // ge2:SetTarget(s.atktg)
    effect::set_target(d, ge2, Some(effect_card_fn(|d, _e, c| !card::is_type(d, c, TYPE_XYZ))));
    // ge2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, ge2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(ge2,tp)
    duel::register_effect(d, ge2, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,2),nil)
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.GetControl(tg,tp,PHASE_END,1)
        duel::get_control(d, tg, a.tp, PHASE_END as u16, 1, None, None);
    }
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not c:IsSetCard(SET_GIMMICK_PUPPET)
    let c = a.c(1);
    !card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16])
}
/// `s.lvfilter(c)`
fn lvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:HasLevel() and not c:IsLevel(8)
    card::is_faceup(d, c) && card::has_level(d, c) && !card::is_level(d, c, &[8])
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.lvfilter,tp,LOCATION_MZONE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&lvfilter), a.tp, LOCATION_MZONE, 0, 1, h.into());
    }
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.lvfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&lvfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(8)
        effect::set_value(d, e1, 8);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
