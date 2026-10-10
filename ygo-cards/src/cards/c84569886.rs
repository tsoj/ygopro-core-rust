//! D/D/D Super Doom King Purple Armageddon
use super::prelude::*;

pub const ID: u32 = 84569886;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DDD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DDD),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_DDD as u16], Some(fc), st, Some(tp)))), 2)],
    );
    // Make an opponent's monster lose 1000 ATK until the end of this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_CONFIRM)
    effect::set_code(d, e1, EVENT_BATTLE_CONFIRM);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.atkcon1)
    effect::set_condition(d, e1, Some(condition(atkcon1)));
    // e1:SetOperation(s.atkop1)
    effect::set_operation(d, e1, Some(operation(atkop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 Attack Position monster your opponent controls and inflict damage to your opponent equal to half its original ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Make the ATK of an opponent's monster become equal to its original ATK until the end of the Damage Step
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_CONFIRM)
    effect::set_code(d, e3, EVENT_BATTLE_CONFIRM);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.atkcon2)
    effect::set_condition(d, e3, Some(condition(atkcon2)));
    // e3:SetOperation(s.atkop2)
    effect::set_operation(d, e3, Some(operation(atkop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Place this card in your Pendulum Zone
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e6, aux::stringid(ID, 3));
    // e6:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e6:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e6, EVENT_DESTROYED);
    // e6:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e6, EFFECT_FLAG_DELAY, 0);
    // e6:SetCondition(s.pencon)
    effect::set_condition(d, e6, Some(condition(pencon)));
    // e6:SetTarget(s.pentg)
    effect::set_target(d, e6, Some(cost_target(pentg)));
    // e6:SetOperation(s.penop)
    effect::set_operation(d, e6, Some(operation(penop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.atkcon1(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let mut atk = duel::get_attacker(d).nil();
    // local d=a:GetBattleTarget()
    let btg = card::get_battle_target(d, atk);
    // if not a or not d then return false end
    let Some(mut bt) = btg else { return false };
    // if a:IsControler(1-tp) then a,d=d,a end
    if card::is_controler(d, atk, 1 - a.tp) {
        std::mem::swap(&mut atk, &mut bt);
    }
    // return a:IsFaceup() and a:IsRelateToBattle() and a:IsSetCard(SET_DDD) and a:IsType(TYPE_FUSION)
    //     and d and d:IsFaceup() and d:IsRelateToBattle() and d:GetAttack()>0 and a:GetControler()~=d:GetControler()
    card::is_faceup(d, atk)
        && card::is_relate_to_battle(d, atk)
        && card::is_set_card(d, atk, &[SET_DDD as u16])
        && card::is_type(d, atk, TYPE_FUSION)
        && card::is_faceup(d, bt)
        && card::is_relate_to_battle(d, bt)
        && card::get_attack(d, bt) > 0
        && card::get_controler(d, atk) != card::get_controler(d, bt)
}
/// `s.atkop1(e,tp,ep,ev,re,r,rp)`
fn atkop1(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d).nil();
    // local d=a:GetBattleTarget()
    let btg = card::get_battle_target(d, atk);
    // if a:IsControler(1-tp) then a,d=d,a end
    let dd = if card::is_controler(d, atk, 1 - a.tp) { Some(atk) } else { btg };
    // if e:GetHandler():IsRelateToEffect(e) and d:IsFaceup() and d:IsRelateToBattle() then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        let dd = dd.nil();
        if card::is_faceup(d, dd) && card::is_relate_to_battle(d, dd) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(-1000)
            effect::set_value(d, e1, -1000);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // d:RegisterEffect(e1)
            card::register_effect(d, dd, e1, false);
        }
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsPosition(POS_FACEUP_ATTACK) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_position(d, chkc, POS_FACEUP_ATTACK as u32);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsPosition,tp,0,LOCATION_MZONE,1,nil,POS_FACEUP_ATTACK) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_position(d, c, POS_FACEUP_ATTACK as u32)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsPosition,tp,0,LOCATION_MZONE,1,1,nil,POS_FACEUP_ATTACK)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_position(d, c, POS_FACEUP_ATTACK as u32)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // local atk=math.floor(g:GetFirst():GetBaseAttack()/2)
    let first = group::get_first(d, &g).nil();
    let mut atk = card::get_base_attack(d, first).div_euclid(2);
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,atk)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local atk=math.floor(tc:GetBaseAttack()/2)
        let mut atk = card::get_base_attack(d, tc).div_euclid(2);
        // if atk<0 then atk=0 end
        if atk < 0 {
            atk = 0;
        }
        // if Duel.Destroy(tc,REASON_EFFECT)~=0 then
        if duel::destroy(d, tc, REASON_EFFECT, None, None) != 0 {
            // Duel.Damage(1-tp,atk,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
/// `s.atkcon2(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d).nil();
    // local d=a:GetBattleTarget()
    let btg = card::get_battle_target(d, atk);
    // if a:IsControler(tp) then a,d=d,a end
    let x = if card::is_controler(d, atk, a.tp) { btg } else { Some(atk) };
    // return a and a:IsControler(1-tp) and a:GetAttack()~=a:GetBaseAttack()
    match x {
        Some(x) => card::is_controler(d, x, 1 - a.tp) && card::get_attack(d, x) != card::get_base_attack(d, x),
        None => false,
    }
}
/// `s.atkop2(e,tp,eg,ep,ev,re,r,rp)`
fn atkop2(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetAttacker()
    let atk = duel::get_attacker(d).nil();
    // if tc:IsControler(tp) then tc=tc:GetBattleTarget() end
    let tc = if card::is_controler(d, atk, a.tp) { card::get_battle_target(d, atk) } else { Some(atk) };
    // if tc and tc:IsFaceup() and tc:GetAttack()~=tc:GetBaseAttack() then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::get_attack(d, tc) != card::get_base_attack(d, tc) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(tc:GetBaseAttack())
            let base = card::get_base_attack(d, tc);
            effect::set_value(d, e1, base as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.pencon(e,tp,eg,ep,ev,re,r,rp)`
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsFaceup()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)
}
/// `s.pentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    true
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.CheckPendulumZones(tp) then return false end
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
