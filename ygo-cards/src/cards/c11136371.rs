//! Counter Gem
use super::prelude::*;

pub const ID: u32 = 11136371;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CRYSTAL_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place as many "Crystal Beast" monsters as possible from your GY face-up in your S/Trap Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.plcost)
    effect::set_cost(d, e1, Some(cost_target(plcost)));
    // e1:SetTarget(s.pltg)
    effect::set_target(d, e1, Some(cost_target(pltg)));
    // e1:SetOperation(s.plop)
    effect::set_operation(d, e1, Some(operation(plop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.plcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn plcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_STZONE,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let mut g = duel::get_matching_group(d, None, tp, LOCATION_STZONE, 0, h.into());
    // if chk==0 then return #g>0 and #g==g:FilterCount(Card.IsAbleToGraveAsCost,nil) end
    if a.chk == Some(0) {
        let cnt = group::filter_count(d, &g, &|d, c| card::is_able_to_grave_as_cost(d, c), NO_EX);
        return !g.is_empty() && g.len() == cnt;
    }
    // g:Filter(Card.IsAbleToGraveAsCost,nil)
    group::match_(d, &mut g, &|d, c| card::is_able_to_grave_as_cost(d, c), NO_EX);
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && !card::is_forbidden(d, c)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&plfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_GRAVE,0,ft,ft,nil)
    let g = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_GRAVE, 0, ft as u16, ft as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
            duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // e1:SetValue(TYPE_SPELL+TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_SPELL + TYPE_CONTINUOUS) as i64);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
        // Duel.RaiseEvent(g,EVENT_CUSTOM+CARD_CRYSTAL_TREE,e,0,tp,0,0)--Event used by Crystal Tree
        duel::raise_event(d, &g, EVENT_CUSTOM + CARD_CRYSTAL_TREE, Some(e), 0, tp, 0, 0);
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_CRYSTAL_BEAST),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]))),
        a.tp,
        LOCATION_ONFIELD,
        0,
        1,
        NO_EX,
    )
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_CRYSTAL_BEAST),tp,LOCATION_ONFIELD,0,nil)
    let g = duel::get_matching_group(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]))),
        a.tp,
        LOCATION_ONFIELD,
        0,
        NO_EX,
    );
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, &g, REASON_EFFECT, None, None);
}
