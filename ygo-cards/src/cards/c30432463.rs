//! Dimonno Vaalmonica
use super::prelude::*;

pub const ID: u32 = 30432463;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[3048768],
    listed_series: &[SET_VAALMONICA],
    counter_place_list: &[COUNTER_RESONANCE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // c:EnableCounterPermit(COUNTER_RESONANCE,LOCATION_PZONE)
    card::enable_counter_permit(d, c, COUNTER_RESONANCE as u16, Some(LOCATION_PZONE), None);
    // Place 1 Resonance Counter on this card each time you take effect damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e1, EVENT_DAMAGE);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==tp and r&REASON_EFFECT>0 and Duel.IsExistingMatchingCard(Card.IsOriginalRace,tp,LOCATION_PZONE,0,1,e:GetHandler(),RACE_FAIRY) end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Opponent's monsters lose 100 ATK for each Resonance Counter on your field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetValue(function(e) return Duel.GetCounter(e:GetHandlerPlayer(),LOCATION_ONFIELD,0,COUNTER_RESONANCE)*-100 end)
    effect::set_value_fn(d, e2, lua_fn(e2_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place both this card and 1 "Angello Vaalmonica" from your Deck in your Pendulum Zone
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(s.pzcost)
    effect::set_cost(d, e3, Some(cost_target(pzcost)));
    // e3:SetTarget(s.pztg)
    effect::set_target(d, e3, Some(cost_target(pztg)));
    // e3:SetOperation(s.pzop)
    effect::set_operation(d, e3, Some(operation(pzop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Apply the damage-inflicting effect of 1 "Vaalmonica" Normal Spell/Trap
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e4, Some(condition(e4_cond)));
    // e4:SetCost(s.cpcost)
    effect::set_cost(d, e4, Some(cost_target(cpcost)));
    // e4:SetTarget(s.cptg)
    effect::set_target(d, e4, Some(cost_target(cptg)));
    // e4:SetOperation(s.cpop)
    effect::set_operation(d, e4, Some(operation(cpop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Register flag on summon
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, ge1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // ge1:SetCode(EVENT_SUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SUMMON_SUCCESS);
        // ge1:SetLabel(id)
        effect::set_label(d, ge1, &[ID as i64]);
        // ge1:SetOperation(aux.sumreg)
        effect::set_operation(d, ge1, Some(aux::sumreg()));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=ge1:Clone()
        let ge2 = effect::clone(d, ge1);
        // ge2:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge2, EVENT_SPSUMMON_SUCCESS);
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
    });
}
/// `s.e1` condition: `ep==tp and r&REASON_EFFECT>0 and Duel.IsExistingMatchingCard(Card.IsOriginalRace,tp,LOCATION_PZONE,0,1,e:GetHandler(),RACE_FAIRY)`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e);
    a.ep == tp
        && (a.r & REASON_EFFECT) > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_original_race(d, c, RACE_FAIRY)), tp, LOCATION_PZONE, 0, 1, h.into())
}
/// `function(e) return e:GetHandler():HasFlagEffect(id) end`
fn e4_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::has_flag_effect(d, h, ID, None)
}
/// `function(e) return Duel.GetCounter(e:GetHandlerPlayer(),LOCATION_ONFIELD,0,COUNTER_RESONANCE)*-100 end`
fn e2_value(d: &mut Duel, a: Args) -> i64 {
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_counter(d, p, LOCATION_ONFIELD as u8, 0, COUNTER_RESONANCE as u16) as i64 * -100
}
/// `s.addcounter(e,tp,eg,ep,ev,re,r,rp)`
fn addcounter(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:AddCounter(COUNTER_RESONANCE,1)
    card::add_counter(d, c, COUNTER_RESONANCE as u16, 1, false);
    // Duel.RaiseEvent(c,EVENT_CUSTOM+39210885,e,0,tp,tp,1)
    duel::raise_event(d, c, EVENT_CUSTOM + 39210885, Some(a.e), 0, a.tp, a.tp, 1);
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // s.addcounter(e,tp,eg,ep,ev,re,r,rp)
        addcounter(d, a);
    } else {
        // Place 1 Resonance Counter on this card at the end of the Chain Link
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
        // e1:SetRange(LOCATION_PZONE)
        effect::set_range(d, e1, LOCATION_PZONE);
        // e1:SetOperation(s.addcounter)
        effect::set_operation(d, e1, Some(operation(addcounter)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CHAIN)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // Reset "e1" at the end of the Chain Link
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
        // e2:SetOperation(function() e1:Reset() end)
        effect::set_operation(d, e2, Some(operation(move |d, _a| effect::reset(d, e1))));
        // e2:SetReset(RESET_CHAIN)
        effect::set_reset(d, e2, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
    }
}
/// `s.pzcostfilter`-less discard cost: `s.pzcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pzcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_discardable(d, x, None, None)), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD,c)
    duel::discard_hand(d, tp, Some(&|d, x| card::is_discardable(d, x, None, None)), 1, 1, REASON_COST | REASON_DISCARD, c.into());
    true
}
/// `s.pzfilter(c)`
fn pzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(3048768) and not c:IsForbidden()
    card::is_code(d, c, &[3048768]) && !card::is_forbidden(d, c)
}
/// `s.pztg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pztg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckLocation(tp,LOCATION_PZONE,0) and Duel.CheckLocation(tp,LOCATION_PZONE,1)
    //     and Duel.IsExistingMatchingCard(s.pzfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::check_location(d, tp, LOCATION_PZONE, 0)
            && duel::check_location(d, tp, LOCATION_PZONE, 1)
            && duel::is_existing_matching_card(d, Some(&pzfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.pzop(e,tp,eg,ep,ev,re,r,rp)`
fn pzop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (Duel.CheckLocation(tp,LOCATION_PZONE,0) and Duel.CheckLocation(tp,LOCATION_PZONE,1) and c:IsRelateToEffect(e)) then return end
    if !(duel::check_location(d, tp, LOCATION_PZONE, 0) && duel::check_location(d, tp, LOCATION_PZONE, 1) && card::is_relate_to_effect(d, c, a.e)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.pzfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&pzfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else {
        return;
    };
    // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
    duel::move_to_field(d, c, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    // Duel.MoveToField(tc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
    duel::move_to_field(d, tc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
}
/// `s.cpcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cpcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.cpfilter(c)`
fn cpfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_VAALMONICA) and (c:IsNormalSpell() or c:IsNormalTrap()) and c:IsAbleToRemoveAsCost()
    //     --and c:CheckActivateEffect(false,true,false)
    card::is_set_card(d, c, &[SET_VAALMONICA as u16])
        && (card::is_normal_spell(d, c) || card::is_normal_trap(d, c))
        && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.cptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // if e:GetLabel()==0 then return false end
        if effect::get_label(d, a.e) == 0 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingMatchingCard(s.cpfilter,tp,LOCATION_GRAVE,0,1,nil)
        return duel::is_existing_matching_card(d, Some(&cpfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.cpfilter,tp,LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&cpfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // Duel.Remove(tc,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, Value::card(tc));
    true
}
/// `s.cpop(e,tp,eg,ep,ev,re,r,rp)`
fn cpop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if not tc then return end
    let Some(tc) = tc else {
        return;
    };
    // local eff=tc:GetActivateEffect()
    // local op=eff:GetOperation()
    let eff = card::get_activate_effect(d, tc).first().copied().nil();
    let op = effect::get_operation(d, eff);
    // if op then op(e,tp,eg,ep,ev,re,r,rp,2) end
    if let Some(op) = op {
        let args = [
            Value::Effect(a.e),
            Value::Int(a.tp as i64),
            a.eg.clone().map_or(Value::Nil, Value::Group),
            Value::Int(a.ep as i64),
            Value::Int(a.ev as i64),
            a.re.map_or(Value::Nil, Value::Effect),
            Value::Int(a.r as i64),
            Value::Int(a.rp as i64),
            Value::Int(2),
        ];
        op.call(d, &args);
    }
}
