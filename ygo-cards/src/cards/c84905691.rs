//! Naturia Horneedle
use super::prelude::*;

pub const ID: u32 = 84905691;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NATURIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy Special Summoned monster(s)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(Cost.Replaceable(s.descost))
    effect::set_cost(d, e1, Some(cost::replaceable(cost_target(descost), None)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,aux.FaceupFilter(Card.IsSetCard,SET_NATURIA),1,false,nil,c) end
    let f = aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_NATURIA as u16]));
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&f), 1, 1, false, None, c.into());
    }
    // local g=Duel.SelectReleaseGroupCost(tp,aux.FaceupFilter(Card.IsSetCard,SET_NATURIA),1,1,false,nil,c)
    let g = duel::select_release_group_cost(d, tp, Some(&f), 1, 1, false, None, c.into());
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.desfilter(c,e,tp)`
fn desfilter(d: &mut Duel, c: CardId, e: Option<EffectId>, tp: u8) -> bool {
    // return c:IsSummonPlayer(1-tp) and (not e or c:IsRelateToEffect(e))
    card::is_summon_player(d, c, 1 - tp) && match e {
        None => true,
        Some(e) => card::is_relate_to_effect(d, c, e),
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=eg:Filter(s.desfilter,nil,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, x| desfilter(d, x, None, tp), NO_EX);
    // if chk==0 then return #g>0 and not c:IsStatus(STATUS_CHAINING) and not eg:IsContains(c) end
    if a.chk == Some(0) {
        return !g.is_empty() && !card::is_status(d, c, STATUS_CHAINING) && !eg.contains(c);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=eg:Filter(s.desfilter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, x| desfilter(d, x, Some(e), tp), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
