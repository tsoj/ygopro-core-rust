//! Sour Scheduling - Red Vinegar Vamoose
use super::prelude::*;

pub const ID: u32 = 65107325;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCondition(s.actcon)
    effect::set_condition(d, e1, Some(condition(actcon)));
    // e1:SetTarget(s.acttg)
    effect::set_target(d, e1, Some(cost_target(acttg)));
    // e1:SetOperation(s.actop)
    effect::set_operation(d, e1, Some(operation(actop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Zones in the same column cannot be used
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_DISABLE_FIELD)
    effect::set_code(d, e2, EFFECT_DISABLE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetOperation(s.znop)
    effect::set_operation(d, e2, Some(lua_fn(znop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.actcon(e,tp,eg,ep,ev,re,r,rp)
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetColumnGroup():IsExists(Card.IsControler,1,nil,1-tp)
    let h = effect::get_handler(d, a.e).nil();
    let colg = card::get_column_group(d, h, 0, 0);
    let tp = a.tp;
    group::is_exists(d, &colg, &|d, c| card::is_controler(d, c, 1 - tp), 1, NO_EX)
}
// function s.acttg(e,tp,eg,ep,ev,re,r,rp,chk)
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetHandler():GetColumnGroup():Filter(Card.IsAbleToHand,nil)
    let h = effect::get_handler(d, a.e).nil();
    let colg = card::get_column_group(d, h, 0, 0);
    let g = group::filter(d, &colg, &|d, c| card::is_able_to_hand(d, c, None), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
// function s.actop(e,tp,eg,ep,ev,re,r,rp)
fn actop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetHandler():GetColumnGroup():Filter(Card.IsAbleToHand,nil)
    let h = effect::get_handler(d, a.e).nil();
    let colg = card::get_column_group(d, h, 0, 0);
    let g = group::filter(d, &colg, &|d, c| card::is_able_to_hand(d, c, None), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
// function s.znop(e,tp,eg,ep,ev,re,r,rp)
fn znop(d: &mut Duel, a: Args) -> i64 {
    let tp = a.p(1);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // local zones=c:GetColumnZone(LOCATION_ONFIELD)
    let mut zones = card::get_column_zone(d, c, LOCATION_ONFIELD, 0, 0, None) as i64;
    // for tc in c:GetColumnGroup():Iter() do
    let colg = card::get_column_group(d, c, 0, 0);
    for tc in group::iter(d, &colg) {
        // local ctrl=tc:IsControler(tp)
        let ctrl = card::is_controler(d, tc, tp);
        // local seq=tc:GetSequence()
        let seq = card::get_sequence(d, tc) as i64;
        // zones=zones&~(ctrl and (1<<seq) or (1<<(16+seq)))
        zones &= !(if ctrl { 1i64 << seq } else { 1i64 << (16 + seq) });
        // if tc:IsInExtraMZone() then
        if card::is_in_extra_m_zone(d, tc, None) {
            // zones=zones&~(ctrl and (1<<(27-seq)) or (1<<(11-seq)))
            zones &= !(if ctrl { 1i64 << (27 - seq) } else { 1i64 << (11 - seq) });
        }
    }
    // return zones
    zones
}
