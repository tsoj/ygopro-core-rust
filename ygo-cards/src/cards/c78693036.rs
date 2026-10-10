//! Juraishin, the Cursed Thunder God
use super::prelude::*;

pub const ID: u32 = 78693036;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Each time your opponent activates a card or effect, gain 300 LP immediately after it resolves
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1a:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1a, EVENT_CHAINING);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetOperation(aux.chainreg)
    effect::set_operation(d, e1a, Some(aux::chainreg()));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1b:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1b, EFFECT_FLAG_DELAY, 0);
    // e1b:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e1b, EVENT_CHAIN_SOLVED);
    // e1b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1b, LOCATION_MZONE);
    // e1b:SetCondition(function(e) return e:GetHandler():HasFlagEffect(1) end)
    effect::set_condition(d, e1b, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::has_flag_effect(d, h, 1, None)
    })));
    // e1b:SetOperation(s.lpop)
    effect::set_operation(d, e1b, Some(operation(lpop)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e2:SetCost(Cost.AND(Cost.PayLP(1500),Cost.SelfTribute))
    effect::set_cost(d, e2, Some(cost::and(vec![cost::pay_lp(cost::PayLp::Amount(1500)), cost::self_tribute()])));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    // if rp==1-tp then
    if a.rp == 1 - a.tp {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // Duel.Recover(tp,300,REASON_EFFECT)
        duel::recover(d, a.tp, 300, REASON_EFFECT, false, None);
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g1=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local g2=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSpellTrap),tp,0,LOCATION_ONFIELD,e:GetHandler():GetEquipGroup())
    let h = effect::get_handler(d, a.e).nil();
    let eqg = card::get_equip_group(d, h);
    let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_spell_trap(d, c)), a.tp, 0, LOCATION_ONFIELD, (&eqg).into());
    // local b1=#g1>0
    let b1 = !g1.is_empty();
    // local b2=#g2>0
    let b2 = !g2.is_empty();
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    let op_value = op.map_or(0, |v| v as i64);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op_value]);
    // local g=(op==1 and g1 or g2)
    let g = if op_value == 1 { g1 } else { g2 };
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Destroy all face-up monsters your opponent controls
        // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
        // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
        if !g.is_empty() {
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    } else if op == 2 {
        // Destroy all face-up Spells/Traps your opponent controls
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSpellTrap),tp,0,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_spell_trap(d, c)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
        // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
        if !g.is_empty() {
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
