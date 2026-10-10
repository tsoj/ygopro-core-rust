//! Judgment of the Pharaoh
use super::prelude::*;

pub const ID: u32 = 55948544;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[81332143, 14731897], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,math.floor(Duel.GetLP(tp)/2))
    let amount = duel::get_lp(d, a.tp).div_euclid(2) as u32;
    duel::pay_lp_cost(d, a.tp, amount);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,81332143)
    let b1 = !duel::has_flag_effect(d, tp, ID, None) && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[81332143])), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local b2=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,14731897)
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None) && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[14731897])), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateEffect(ev) end` (the same for monster and Spell/Trap effects)
fn negate_op(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let yu_jo = |d: &mut Duel| duel::is_existing_matching_card(d, Some(&|d, x| card::is_code(d, x, &[81332143])), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    let unity = |d: &mut Duel| duel::is_existing_matching_card(d, Some(&|d, x| card::is_code(d, x, &[14731897])), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    if op == 1 && yu_jo(d) && !duel::has_flag_effect(d, tp, ID, None) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,3))
        effect::set_description(d, e1, aux::stringid(ID, 3));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
        // e1:SetTargetRange(0,1)
        effect::set_target_range(d, e1, 0, 1);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CANNOT_MSET)
        effect::set_code(d, e2, EFFECT_CANNOT_MSET);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        // e3:SetCode(EFFECT_CANNOT_FLIP_SUMMON)
        effect::set_code(d, e3, EFFECT_CANNOT_FLIP_SUMMON);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, tp);
        // local e4=e1:Clone()
        let e4 = effect::clone(d, e1);
        // e4:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
        // Duel.RegisterEffect(e4,tp)
        duel::register_effect(d, e4, tp);
        // local e5=e1:Clone()
        let e5 = effect::clone(d, e1);
        // e5:SetCode(EFFECT_CANNOT_TURN_SET)
        effect::set_code(d, e5, EFFECT_CANNOT_TURN_SET);
        // Duel.RegisterEffect(e5,tp)
        duel::register_effect(d, e5, tp);
        // local e6=e1:Clone()
        let e6 = effect::clone(d, e1);
        // e6:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e6, EFFECT_CANNOT_ACTIVATE);
        // e6:SetValue(function(e,re,tp) return re:IsMonsterEffect() end)
        effect::set_value_fn(d, e6, lua_fn(|d, x| effect::is_monster_effect(d, x.e(1))));
        // Duel.RegisterEffect(e6,tp)
        duel::register_effect(d, e6, tp);
        // local e7=Effect.CreateEffect(c)
        let e7 = effect::create_effect(d, c);
        // e7:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e7, EFFECT_TYPE_FIELD);
        // e7:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e7, EFFECT_DISABLE);
        // e7:SetTargetRange(0,LOCATION_MZONE)
        effect::set_target_range(d, e7, 0, LOCATION_MZONE);
        // e7:SetTarget(function(e,c) return c:IsType(TYPE_EFFECT) or c:IsOriginalType(TYPE_EFFECT) end)
        effect::set_target(d, e7, Some(effect_card_fn(|d, _e, x| card::is_type(d, x, TYPE_EFFECT) || card::is_original_type(d, x, TYPE_EFFECT))));
        // e7:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e7, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e7,tp)
        duel::register_effect(d, e7, tp);
        // local e8=Effect.CreateEffect(c)
        let e8 = effect::create_effect(d, c);
        // e8:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e8, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e8:SetCode(EVENT_CHAIN_SOLVING)
        effect::set_code(d, e8, EVENT_CHAIN_SOLVING);
        // e8:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and re:IsMonsterEffect() end)
        effect::set_condition(d, e8, Some(condition(|d, x| x.rp == 1 - x.tp && effect::is_monster_effect(d, x.re.nil()))));
        // e8:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateEffect(ev) end)
        effect::set_operation(d, e8, Some(operation(negate_op)));
        // e8:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e8, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e8,tp)
        duel::register_effect(d, e8, tp);
    } else if op == 2 && unity(d) && !duel::has_flag_effect(d, tp, ID + 1, None) {
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // local e9=Effect.CreateEffect(c)
        let e9 = effect::create_effect(d, c);
        // e9:SetDescription(aux.Stringid(id,4))
        effect::set_description(d, e9, aux::stringid(ID, 4));
        // e9:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e9, EFFECT_TYPE_FIELD);
        // e9:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e9, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e9:SetCode(EFFECT_CANNOT_SSET)
        effect::set_code(d, e9, EFFECT_CANNOT_SSET);
        // e9:SetTargetRange(0,1)
        effect::set_target_range(d, e9, 0, 1);
        // e9:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e9, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e9,tp)
        duel::register_effect(d, e9, tp);
        // local e10=e9:Clone()
        let e10 = effect::clone(d, e9);
        // e10:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e10, EFFECT_CANNOT_ACTIVATE);
        // e10:SetValue(function(e,re,tp) return re:IsSpellTrapEffect() end)
        effect::set_value_fn(d, e10, lua_fn(|d, x| effect::is_spell_trap_effect(d, x.e(1))));
        // Duel.RegisterEffect(e10,tp)
        duel::register_effect(d, e10, tp);
        // local e11=Effect.CreateEffect(c)
        let e11 = effect::create_effect(d, c);
        // e11:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e11, EFFECT_TYPE_FIELD);
        // e11:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e11, EFFECT_DISABLE);
        // e11:SetTargetRange(0,LOCATION_SZONE)
        effect::set_target_range(d, e11, 0, LOCATION_SZONE);
        // e11:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e11, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e11,tp)
        duel::register_effect(d, e11, tp);
        // local e12=e11:Clone()
        let e12 = effect::clone(d, e11);
        // e12:SetCode(EFFECT_DISABLE_TRAPMONSTER)
        effect::set_code(d, e12, EFFECT_DISABLE_TRAPMONSTER);
        // e12:SetTargetRange(0,LOCATION_MZONE)
        effect::set_target_range(d, e12, 0, LOCATION_MZONE);
        // Duel.RegisterEffect(e12,tp)
        duel::register_effect(d, e12, tp);
        // local e13=Effect.CreateEffect(c)
        let e13 = effect::create_effect(d, c);
        // e13:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e13, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e13:SetCode(EVENT_CHAIN_SOLVING)
        effect::set_code(d, e13, EVENT_CHAIN_SOLVING);
        // e13:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and re:IsSpellTrapEffect() end)
        effect::set_condition(d, e13, Some(condition(|d, x| x.rp == 1 - x.tp && effect::is_spell_trap_effect(d, x.re.nil()))));
        // e13:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateEffect(ev) end)
        effect::set_operation(d, e13, Some(operation(negate_op)));
        // e13:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e13, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e13,tp)
        duel::register_effect(d, e13, tp);
    }
}
