//! Linkslayer
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 35595518;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If you control no monsters, you can Special Summon this card (from your hand)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy up to 2 Spells/Traps on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.Discard(nil,nil,1,
    //     function(e,tp) return math.min(2,Duel.GetTargetCount(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)) end,
    //     function(e,tp,og) e:SetLabel(#og) end))
    effect::set_cost(
        d,
        e2,
        Some(cost::discard(
            None,
            false,
            Some(cost::Count::N(1)),
            Some(cost::Count::F(Arc::new(discard_max))),
            Some(Arc::new(discard_op)),
        )),
    );
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0,nil)==0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `function(e,tp) return math.min(2,Duel.GetTargetCount(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)) end`
fn discard_max(d: &mut Duel, _e: EffectId, tp: u8) -> i64 {
    duel::get_target_count(d, Some(&card::is_spell_trap), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX).min(2) as i64
}
/// `function(e,tp,og) e:SetLabel(#og) end`
fn discard_op(d: &mut Duel, e: EffectId, _tp: u8, og: &CardSet) {
    effect::set_label(d, e, &[group::get_count(og) as i64]);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsSpellTrap() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_spell_trap(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,ct,ct,nil)
    let g = duel::select_target(d, a.tp, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, ct as u16, ct as u16, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,ct,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, ct as u8, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #tg>0 then
    if group::get_count(&tg) > 0 {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
