//! Artorigus, King of the Noble Knights
use super::prelude::*;

pub const ID: u32 = 21223277;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NOBLE_ARMS, SET_NOBLE_KNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_NOBLE_KNIGHT),4,2)
    xyz::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_NOBLE_KNIGHT as u16], sc, st, Some(tp)))), Some(4), 2, None, None, None, None, false, None);
    // Equip to itself up to 3 "Noble Arms" Equip Spell Cards from the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(xyz_cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy Spell/Trap cards on the field up to the number of "Noble Arms" Equip Spell Cards you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn xyz_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsXyzSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, c)
}
/// `s.filter(c,e,tp,ec)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ec: CardId) -> bool {
    // return c:IsSetCard(SET_NOBLE_ARMS) and c:IsCanBeEffectTarget(e) and c:CheckUniqueOnField(tp) and c:CheckEquipTarget(ec)
    card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16])
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::check_unique_on_field(d, c, tp, None, None)
        && card::check_equip_target(d, c, ec)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil,e,tp,c)
    let g = duel::get_matching_group(d, Some(&|d, cc| filter(d, cc, e, tp, c)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc,e,tp,c) end
    if let Some(chkc) = a.chkc() {
        return card::get_controler(d, chkc) == tp && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, e, tp, c);
    }
    // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_SZONE),3)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0.min(3);
    // if chk==0 then return ft>0 and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_matching_card(d, Some(&|d, cc| filter(d, cc, e, tp, c)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local eg=aux.SelectUnselectGroup(g,e,tp,1,ft,aux.dncheck,1,tp,HINTMSG_EQUIP)
    let eg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(ft as usize), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_EQUIP as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,eg,#eg,0,0)
    let n = eg.len();
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(eg), n as u8, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, e).nil();
    // if #g==0 or ft<#g then return end
    if g.is_empty() || (ft as usize) < g.len() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // Duel.Equip(tp,tc,c,true,true)
        duel::equip(d, tp, tc, c, true, true);
    }
    // Duel.EquipComplete()
    duel::equip_complete(d);
}
/// `s.nbarmsfilters(c)`
fn nbarmsfilters(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsEquipSpell() and c:IsSetCard(SET_NOBLE_ARMS)
    card::is_faceup(d, c) && card::is_equip_spell(d, c) && card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16])
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(s.nbarmsfilters,tp,LOCATION_SZONE,0,1,nil)
        //     and Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
        return duel::is_existing_matching_card(d, Some(&|d, c| nbarmsfilters(d, c)), tp, LOCATION_SZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(s.nbarmsfilters,tp,LOCATION_SZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| nbarmsfilters(d, c)), tp, LOCATION_SZONE, 0, NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,ct,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX).nil();
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
