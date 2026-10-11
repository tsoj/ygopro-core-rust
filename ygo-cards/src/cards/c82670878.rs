//! Ogre of the Scarlet Sorrow
use super::prelude::*;

pub const ID: u32 = 82670878;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk/def up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.adcon)
    effect::set_condition(d, e2, Some(condition(adcon)));
    // e2:SetOperation(s.adop)
    effect::set_operation(d, e2, Some(operation(adop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // s[0]=0
        set_s_var(d, ID, 0, Value::Int(0));
        // s[1]=0
        set_s_var(d, ID, 1, Value::Int(0));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_ATTACK_ANNOUNCE)
        effect::set_code(d, ge1, EVENT_ATTACK_ANNOUNCE);
        // ge1:SetOperation(s.check)
        effect::set_operation(d, ge1, Some(operation(check)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=Effect.CreateEffect(c)
        let ge2 = effect::create_effect(d, c);
        // ge2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge2:SetCode(EVENT_ATTACK_DISABLED)
        effect::set_code(d, ge2, EVENT_ATTACK_DISABLED);
        // ge2:SetOperation(s.check2)
        effect::set_operation(d, ge2, Some(operation(check2)));
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
        // aux.AddValuesReset(function()
        //     s[0]=0
        //     s[1]=0
        // end)
        aux::add_values_reset(d, reset_values);
    });
    // end)
}
/// The reset function of `aux.AddValuesReset` (`s[0]=0 s[1]=0`).
fn reset_values(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
}
/// `s.check(e,tp,eg,ep,ev,re,r,rp)`
fn check(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &a.eg.clone().nil());
    // if Duel.GetAttackTarget()==nil then
    if duel::get_attack_target(d).is_none() {
        let tc = tc.nil();
        // s[1-tc:GetControler()]=s[1-tc:GetControler()]+1
        let k = (1 - card::get_controler(d, tc)) as u64;
        let v = s_var(d, ID, k).int() + 1;
        set_s_var(d, ID, k, Value::Int(v));
        // if s[1-tc:GetControler()]==1 then
        if s_var(d, ID, k).int() == 1 {
            // s[2]=tc
            set_s_var(d, ID, 2, Value::Card(tc));
            // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
            card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        } else if s_var(d, ID, k).int() == 2 {
            // Duel.RaiseEvent(tc,EVENT_CUSTOM+id,e,0,0,0,0)
            duel::raise_event(d, CardSet::from_card(tc), EVENT_CUSTOM + ID, Some(a.e), 0, 0, 0, 0);
        }
    }
}
/// `s.check2(e,tp,eg,ep,ev,re,r,rp)`
fn check2(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let tc = group::get_first(d, &a.eg.clone().nil()).nil();
    // if tc:GetFlagEffect(id)~=0 and Duel.GetAttackTarget()~=nil then
    if card::get_flag_effect(d, tc, ID) != 0 && duel::get_attack_target(d).is_some() {
        // s[1-tc:GetControler()]=s[1-tc:GetControler()]-1
        let k = (1 - card::get_controler(d, tc)) as u64;
        let v = s_var(d, ID, k).int() - 1;
        set_s_var(d, ID, k, Value::Int(v));
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.GetAttackTarget()==nil and s[tp]==2
    duel::is_turn_player(d, 1 - a.tp) && duel::get_attack_target(d).is_none() && s_var(d, ID, a.tp as u64).int() == 2
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,1,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, CardSet::from_card(c), 1, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
/// `s.adcon(e,tp,eg,ep,ev,re,r,rp)`
fn adcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1
    let c = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, c) == SUMMON_TYPE_SPECIAL + 1
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler()
    c != effect::get_handler(d, e).nil()
}
/// `s.adop(e,tp,eg,ep,ev,re,r,rp)`
fn adop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=s[2]
    let tc = s_var(d, ID, 2).as_card();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // if tc and tc:GetFlagEffect(id) then
        // (a flag count is a number, which Lua reads as true: only tc is tested)
        if let Some(tc) = tc {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // e1:SetValue(tc:GetAttack())
            let atk = card::get_attack(d, tc);
            effect::set_value(d, e1, atk as i64);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
            // e2:SetValue(tc:GetDefense())
            let def = card::get_defense(d, tc);
            effect::set_value(d, e2, def as i64);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
        }
        // at limit
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD);
        // e3:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
        effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
        // e3:SetRange(LOCATION_MZONE)
        effect::set_range(d, e3, LOCATION_MZONE);
        // e3:SetTargetRange(0,LOCATION_MZONE)
        effect::set_target_range(d, e3, 0, LOCATION_MZONE);
        // e3:SetValue(s.atlimit)
        effect::set_value_fn(d, e3, effect_card_fn(atlimit));
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    }
}
