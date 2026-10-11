//! Dinomist Spinos
use super::prelude::*;

pub const ID: u32 = 5067884;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOMIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Enable pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Substitute destruction for a "Dinomist" card(s)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetTarget(s.reptg)
    effect::set_target(d, e1, Some(cost_target(reptg)));
    // e1:SetValue(s.repval)
    effect::set_value_fn(d, e1, effect_card_fn(repval));
    // e1:SetOperation(s.repop)
    effect::set_operation(d, e1, Some(operation(repop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of 2 effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetCost(s.atkcost)
    effect::set_cost(d, e2, Some(cost_target(atkcost)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_ONFIELD) and c:IsSetCard(SET_DINOMIST)
    //     and not c:IsReason(REASON_REPLACE) and (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()~=tp))
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_ONFIELD)
        && card::is_set_card(d, c, &[SET_DINOMIST as u16])
        && !card::is_reason(d, c, REASON_REPLACE)
        && (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) != tp))
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return eg:IsExists(s.repfilter,1,c,tp)
    //     and c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, x| repfilter(d, x, a.tp), 1, c.into())
            && card::is_destructable(d, c, Some(a.e))
            && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT|REASON_REPLACE)
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT | REASON_REPLACE, None, None);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsAbleToEnterBP()
    duel::is_able_to_enter_bp(d)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsSetCard,1,false,nil,c,SET_DINOMIST) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, x| card::is_set_card(d, x, &[SET_DINOMIST as u16])), 1, 1, false, None, c.into());
    }
    // local rg=Duel.SelectReleaseGroupCost(tp,Card.IsSetCard,1,1,false,nil,c,SET_DINOMIST)
    let rg = duel::select_release_group_cost(d, a.tp, Some(&|d, x| card::is_set_card(d, x, &[SET_DINOMIST as u16])), 1, 1, false, None, c.into());
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    true
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local con1=c:GetFlagEffect(id)==0
    let con1 = card::get_flag_effect(d, c, ID) == 0;
    // local con2=c:GetFlagEffect(id+1)==0
    let con2 = card::get_flag_effect(d, c, ID + 1) == 0;
    // if chk==0 then return con1 or con2 end
    if a.chk == Some(0) {
        return con1 || con2;
    }
    // local op=0
    // if con1 and con2 then
    let op = if con1 && con2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
    // elseif con1 then
    } else if con1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1))
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)])
    // else
    } else {
        // op=Duel.SelectOption(tp,aux.Stringid(id,2))+1
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]) + 1
    };
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==0 then
    if op == 0 {
        // if c:IsFaceup() and c:IsRelateToEffect(e) then
        if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
            // c:RegisterFlagEffect(id,RESETS_STANDARD_DISABLE_PHASE_END,0,0)
            card::register_flag_effect(d, c, ID, RESETS_STANDARD_DISABLE_PHASE_END, 0, 0, 0, 0);
            // Can attack directly
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3205)
            effect::set_description(d, e1, 3205);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DIRECT_ATTACK)
            effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    // elseif op==1 then
    } else if op == 1 {
        // if c:IsFaceup() and c:IsRelateToEffect(e) then
        if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
            // c:RegisterFlagEffect(id+1,RESETS_STANDARD_DISABLE_PHASE_END,0,0)
            card::register_flag_effect(d, c, ID + 1, RESETS_STANDARD_DISABLE_PHASE_END, 0, 0, 0, 0);
            // Can make a second attack
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetDescription(3201)
            effect::set_description(d, e2, 3201);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_EXTRA_ATTACK)
            effect::set_code(d, e2, EFFECT_EXTRA_ATTACK);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e2:SetValue(1)
            effect::set_value(d, e2, 1);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
        }
    }
}
