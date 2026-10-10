//! Pyro Clock of Destiny
use super::prelude::*;

pub const ID: u32 = 1082946;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `Card.IsHasEffect(c, id)` as a filter
fn has_id_effect(d: &mut Duel, c: CardId) -> bool {
    // Card.IsHasEffect(c,id)
    !card::is_has_effect(d, c, ID, None).is_empty()
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsHasEffect,tp,LOCATION_ALL,LOCATION_ALL,1,nil,id) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&has_id_effect), a.tp, LOCATION_ALL, LOCATION_ALL, 1, NO_EX);
    }
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local g=Duel.SelectMatchingCard(tp,Card.IsHasEffect,tp,LOCATION_ALL,LOCATION_ALL,1,1,nil,id)
    let g = duel::select_matching_card(d, a.tp, Some(&has_id_effect), a.tp, LOCATION_ALL, LOCATION_ALL, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // local eff={tc:GetCardEffect(id)}
    let eff = card::get_card_effect(d, tc, ID);
    // local sel={}
    let mut sel: Vec<EffectId> = Vec::new();
    // local seld={}
    let mut seld: Vec<u64> = Vec::new();
    // local turne
    let mut turne: Option<EffectId> = None;
    // for _,te in ipairs(eff) do
    for te in eff {
        // table.insert(sel,te)
        sel.push(te);
        // table.insert(seld,te:GetDescription())
        seld.push(effect::get_description(d, te));
    }
    // if #sel==1 then turne=sel[1] elseif #sel>1 then
    if sel.len() == 1 {
        turne = Some(sel[0]);
    } else if sel.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EFFECT)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EFFECT as u64);
        // local op=Duel.SelectOption(tp,table.unpack(seld))+1
        let op = duel::select_option(d, a.tp, true, &seld);
        // turne=sel[op]
        turne = Some(sel[op as usize]);
    }
    // if not turne then return end
    let Some(turne) = turne else { return };
    // local op=turne:GetOperation()
    let op = effect::get_operation(d, turne).nil();
    // op(turne,turne:GetOwnerPlayer(),nil,0,id,nil,0,0)
    let owner = effect::get_owner_player(d, turne);
    op.call(
        d,
        &[
            Value::Effect(turne),
            Value::Int(owner as i64),
            Value::Nil,
            Value::Int(0),
            Value::Int(ID as i64),
            Value::Nil,
            Value::Int(0),
            Value::Int(0),
        ],
    );
}
