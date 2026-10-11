//! DoomZ Command "D.O.O.M.D.U.R.G."
use super::prelude::*;

pub const ID: u32 = 68831625;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DOOMZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Inflict 500 damage to the equipped monster's controller during the Standby Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():GetEquipTarget() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::get_equip_target(d, h).is_some()
    })));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent cannot target this card with card effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // Destroy 1 other face-up card you control, also this card gains ATK equal to its own Level/Rank x 100, it can attack directly and if it battles destroy it at the end of the Damage Step
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e3:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e3, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e3:SetTarget(s.desatktg)
    effect::set_target(d, e3, Some(cost_target(desatktg)));
    // e3:SetOperation(s.desatkop)
    effect::set_operation(d, e3, Some(operation(desatkop)));
    // Grant the above effects to a "DoomZ" monster equipped with this card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_GRANT)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    // e4:SetTarget(function(e,c) return e:GetHandler():GetEquipTarget()==c and c:IsSetCard(SET_DOOMZ) end)
    effect::set_target(d, e4, Some(effect_card_fn(|d, e, c| {
        let h = effect::get_handler(d, e).nil();
        card::get_equip_target(d, h) == Some(c) && card::is_set_card(d, c, &[SET_DOOMZ as u16])
    })));
    // e4:SetLabelObject(e2)
    effect::set_label_object(d, e4, Value::Effect(e2));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetLabelObject(e3)
    effect::set_label_object(d, e5, Value::Effect(e3));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Grant the above protection effect (e2) to a WIND Machine Xyz Monster that has it as material
    // local e6=e2:Clone()
    let e6 = effect::clone(d, e2);
    // e6:SetType(EFFECT_TYPE_XMATERIAL)
    effect::set_type(d, e6, EFFECT_TYPE_XMATERIAL);
    // e6:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e6, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // Grant the above destruction+atk change effect (e3) to a WIND Machine Xyz Monster that has it as material
    // local e7=e3:Clone()
    let e7 = effect::clone(d, e3);
    // e7:SetType(EFFECT_TYPE_XMATERIAL+EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e7, EFFECT_TYPE_XMATERIAL + EFFECT_TYPE_QUICK_O);
    // e7:SetRange(LOCATION_MZONE)
    effect::set_range(d, e7, LOCATION_MZONE);
    // e7:SetCondition(function(e) local c=e:GetHandler() return c:IsAttribute(ATTRIBUTE_WIND) and c:IsRace(RACE_MACHINE) end)
    effect::set_condition(d, e7, Some(condition(|d, a| {
        let c = effect::get_handler(d, a.e).nil();
        card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_race(d, c, RACE_MACHINE)
    })));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local p=e:GetHandler():GetEquipTarget():GetControler()
    let h = effect::get_handler(d, a.e).nil();
    let et = card::get_equip_target(d, h).nil();
    let p = card::get_controler(d, et);
    // Duel.SetTargetPlayer(p)
    duel::set_target_player(d, p);
    // Duel.SetTargetParam(500)
    duel::set_target_param(d, 500);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,0,1,p,500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, p, 500);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(e:GetHandler():GetEquipTarget():GetControler(),500,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    let et = card::get_equip_target(d, h).nil();
    let p = card::get_controler(d, et);
    duel::damage(d, p, 500, REASON_EFFECT, false, None);
}
/// `s.desatktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desatktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local dg=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_ONFIELD,0,c)
    let dg = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_ONFIELD, 0, c.into());
    // if chk==0 then return #dg>0 end
    if a.chk == Some(0) {
        return !dg.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, tp, 0);
    // local atk=(c:HasLevel() and c:GetLevel() or c:GetRank())*100
    let atk = level_or_rank_card(d, c) * 100;
    // Duel.SetPossibleOperationInfo(0,CATEGORY_ATKCHANGE,nil,1,tp,atk)
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, None, 1, tp, atk as i32);
    true
}
/// `(c:HasLevel() and c:GetLevel() or c:GetRank())`
fn level_or_rank_card(d: &mut Duel, c: CardId) -> i64 {
    if card::has_level(d, c) {
        card::get_level(d, c) as i64
    } else {
        card::get_rank(d, c) as i64
    }
}
/// `s.desatkop(e,tp,eg,ep,ev,re,r,rp)`
fn desatkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc = if card::is_relate_to_effect(d, c, a.e) { Some(c) } else { None };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local desg=Duel.SelectMatchingCard(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,0,1,1,exc)
    let desg = duel::select_matching_card(d, tp, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_ONFIELD, 0, 1, 1, false, exc.into()).nil();
    // if #desg==0 then return end
    if desg.is_empty() {
        return;
    }
    // Duel.HintSelection(desg)
    duel::hint_selection(d, desg.clone(), true);
    // if Duel.Destroy(desg,REASON_EFFECT)>0 and c:IsRelateToEffect(e) then
    if duel::destroy(d, desg, REASON_EFFECT, None, None) > 0 && card::is_relate_to_effect(d, c, a.e) {
        // if c:IsFaceup() then
        if card::is_faceup(d, c) {
            // local atk=(c:HasLevel() and c:GetLevel() or c:GetRank())*100
            let atk = level_or_rank_card(d, c) * 100;
            // --This card gains ATK equal to its own Level/Rank x 100
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(atk)
            effect::set_value(d, e1, atk);
            // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
        // --It can attack directly
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e2, aux::stringid(ID, 2));
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // --If it battles, destroy it at the end of the Damage Step
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e3:SetCode(EVENT_DAMAGE_STEP_END)
        effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
        // e3:SetCondition(function(e) return e:GetHandler():IsRelateToBattle() end)
        effect::set_condition(d, e3, Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_relate_to_battle(d, h)
        })));
        // e3:SetOperation(function(e) Duel.Hint(HINT_CARD,0,c:GetOriginalCodeRule()) Duel.Destroy(e:GetHandler(),REASON_EFFECT) end)
        effect::set_operation(d, e3, Some(operation(move |d, a| {
            let code = card::get_original_code_rule(d, c)[0];
            duel::hint(d, HINT_CARD as u8, 0, code as u64);
            let h = effect::get_handler(d, a.e).nil();
            duel::destroy(d, h, REASON_EFFECT, None, None);
        })));
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    }
}
