//! Vagnawa the Moon-Eating Dragon
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 9839115;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Check the Tuner material's Level
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e0, EFFECT_MATERIAL_CHECK);
    // e0:SetLabel(0)
    effect::set_label(d, e0, &[0]);
    // e0:SetValue(s.valcheck)
    effect::set_value_fn(d, e0, effect_card_value(valcheck));
    // e0:SetOperation(s.matop)
    effect::set_operation(d, e0, Some(lua_fn(matop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Make this card gain ATK and inflict damage to your opponent
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(condition(lv_cond)));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.lvtg)
    effect::set_target(d, e1, Some(cost_target(lvtg)));
    // e1:SetOperation(s.lvop)
    effect::set_operation(d, e1, Some(operation(lvop)));
    // e1:SetLabelObject(e0)
    effect::set_label_object(d, e1, Value::Effect(e0));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.puretuner(c,sc,tp)`
fn puretuner(d: &mut Duel, c: CardId, sc: CardId, tp: u8) -> bool {
    // return not c:IsHasEffect(EFFECT_NONTUNER) and c:IsType(TYPE_TUNER,sc,SUMMON_TYPE_SYNCHRO|MATERIAL_SYNCHRO,tp)
    card::is_has_effect(d, c, EFFECT_NONTUNER, None).is_empty()
        && card::is_type_ctx(d, c, TYPE_TUNER, Some(sc), SUMMON_TYPE_SYNCHRO as u64 | MATERIAL_SYNCHRO as u64, Some(tp))
}
/// `s.othertuner(c,sc,tp)`
fn othertuner(d: &mut Duel, c: CardId, sc: CardId, tp: u8) -> bool {
    // return (c:IsHasEffect(EFFECT_NONTUNER) and c:IsType(TYPE_TUNER,sc,SUMMON_TYPE_SYNCHRO|MATERIAL_SYNCHRO,tp))
    //     or c:IsHasEffect(EFFECT_CAN_BE_TUNER)
    (!card::is_has_effect(d, c, EFFECT_NONTUNER, None).is_empty()
        && card::is_type_ctx(d, c, TYPE_TUNER, Some(sc), SUMMON_TYPE_SYNCHRO as u64 | MATERIAL_SYNCHRO as u64, Some(tp)))
        || !card::is_has_effect(d, c, EFFECT_CAN_BE_TUNER, None).is_empty()
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local mg=c:GetMaterial()
    let mg = card::get_material(d, c);
    // if #mg==0 then return e:SetLabel(0) end
    if mg.is_empty() {
        effect::set_label(d, e, &[0]);
        return 0;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local pure_tuner_g=mg:Filter(s.puretuner,nil,c,tp)
    let pure_tuner_g = group::filter(d, &mg, &|d, x| puretuner(d, x, c, tp), NO_EX);
    // if #pure_tuner_g>0 then
    if !pure_tuner_g.is_empty() {
        // local tunerlv=pure_tuner_g:GetFirst():GetSynchroLevel(c)
        let tuner = group::get_first(d, &pure_tuner_g).nil();
        let tunerlv = card::get_synchro_level(d, tuner, c) as i64;
        // e:SetLabelObject(nil)
        effect::set_label_object(d, e, Value::Nil);
        // e:SetLabel(tunerlv,c:GetLevel()-tunerlv)
        let lv = card::get_level(d, c) as i64;
        effect::set_label(d, e, &[tunerlv, lv - tunerlv]);
        // return
        return 0;
    }
    // local other_tuner_g=mg:Filter(s.othertuner,nil,c,tp)
    let other_tuner_g = group::filter(d, &mg, &|d, x| othertuner(d, x, c, tp), NO_EX);
    // if #other_tuner_g>1 then
    if other_tuner_g.len() > 1 {
        // e:SetLabelObject(other_tuner_g)
        effect::set_label_object(d, e, Value::Group(other_tuner_g));
    } else {
        // local tunerlv=other_tuner_g:GetFirst():GetSynchroLevel(c)
        let tuner = group::get_first(d, &other_tuner_g).nil();
        let tunerlv = card::get_synchro_level(d, tuner, c) as i64;
        // e:SetLabelObject(nil)
        effect::set_label_object(d, e, Value::Nil);
        // e:SetLabel(tunerlv,c:GetLevel()-tunerlv)
        let lv = card::get_level(d, c) as i64;
        effect::set_label(d, e, &[tunerlv, lv - tunerlv]);
    }
    0
}
/// `s.matop(e,c)`
fn matop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(1);
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e);
    // if not g then return end
    let Some(g) = g.as_group().cloned() else {
        return;
    };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local tuner=g:Select(tp,1,1,nil):GetFirst()
    let tuner = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    let tuner = group::get_first(d, &tuner).nil();
    // Duel.HintSelection(tuner,true)
    duel::hint_selection(d, tuner, true);
    // local tunerlv=tuner:GetSynchroLevel(c)
    let tunerlv = card::get_synchro_level(d, tuner, c) as i64;
    // e:SetLabel(tunerlv,e:GetHandler():GetLevel()-tunerlv)
    let h = effect::get_handler(d, e).nil();
    let lv = card::get_level(d, h) as i64;
    effect::set_label(d, e, &[tunerlv, lv - tunerlv]);
}
/// `function(e) return e:GetHandler():IsSynchroSummoned() end`
fn lv_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetLabelObject():GetLabel()>0 end
    if a.chk == Some(0) {
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        return effect::get_label(d, lo) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,e:GetHandler(),1,tp,300)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(h)), 1, a.tp, 300);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,1,1-tp,300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, 1 - a.tp, 300);
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tuner_lv,nontuner_lv=e:GetLabelObject():GetLabel()
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let labels = effect::get_labels(d, lo);
    let tuner_lv = labels.first().copied();
    let nontuner_lv = labels.get(1).copied();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Gains ATK equal to the total Levels of the non-Tuners used x 300
        // e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(nontuner_lv*300)
        effect::set_value(d, e1, nontuner_lv.nil() * 300);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END,2)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 2);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,tuner_lv*300,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, tuner_lv.nil() * 300, REASON_EFFECT, false, None);
    }
}
