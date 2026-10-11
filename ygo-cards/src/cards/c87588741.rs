//! Qliphort Cephalopod
//! クリフォート・アクセス
use super::prelude::*;

pub const ID: u32 = 87588741;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_QLI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // splimit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetTarget(s.splimit)
    effect::set_target(d, e2, Some(effect_card_fn(splimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atk down
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_PZONE)
    effect::set_range(d, e3, LOCATION_PZONE);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetValue(-300)
    effect::set_value(d, e3, -300);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // summon with no tribute
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e4, EFFECT_SUMMON_PROC);
    // e4:SetCondition(s.ntcon)
    effect::set_condition(d, e4, Some(lua_fn(ntcon)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // change level
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_SUMMON_COST)
    effect::set_code(d, e5, EFFECT_SUMMON_COST);
    // e5:SetOperation(s.lvop)
    effect::set_operation(d, e5, Some(operation(lvop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetCode(EFFECT_SPSUMMON_COST)
    effect::set_code(d, e6, EFFECT_SPSUMMON_COST);
    // e6:SetOperation(s.lvop2)
    effect::set_operation(d, e6, Some(operation(lvop2)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // immune
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE);
    // e7:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e7, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e7:SetRange(LOCATION_MZONE)
    effect::set_range(d, e7, LOCATION_MZONE);
    // e7:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e7, EFFECT_IMMUNE_EFFECT);
    // e7:SetCondition(s.immcon)
    effect::set_condition(d, e7, Some(condition(immcon)));
    // e7:SetValue(aux.qlifilter)
    effect::set_value_fn(d, e7, aux::qlifilter());
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // damage
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e8, aux::stringid(ID, 1));
    // e8:SetCategory(CATEGORY_DAMAGE+CATEGORY_RECOVER)
    effect::set_category(d, e8, CATEGORY_DAMAGE + CATEGORY_RECOVER);
    // e8:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e8, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e8:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e8, EVENT_SUMMON_SUCCESS);
    // e8:SetCondition(s.damcon)
    effect::set_condition(d, e8, Some(condition(damcon)));
    // e8:SetTarget(s.damtg)
    effect::set_target(d, e8, Some(cost_target(damtg)));
    // e8:SetOperation(s.damop)
    effect::set_operation(d, e8, Some(operation(damop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
    // tribute check
    // local e9=Effect.CreateEffect(c)
    let e9 = effect::create_effect(d, c);
    // e9:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e9, EFFECT_TYPE_SINGLE);
    // e9:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e9, EFFECT_MATERIAL_CHECK);
    // e9:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e9, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e9:SetValue(s.valcheck)
    effect::set_value_fn(d, e9, lua_fn(valcheck));
    // e9:SetLabelObject(e8)
    effect::set_label_object(d, e9, Value::Effect(e8));
    // c:RegisterEffect(e9)
    card::register_effect(d, c, e9, false);
}
// function s.splimit(e,c)
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_QLI)
    !card::is_set_card(d, c, &[SET_QLI as u16])
}
// function s.ntcon(e,c,minc)
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return minc==0 and c:GetLevel()>4 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    a.int(2) == 0 && card::get_level(d, c) > 4 && duel::get_location_count(d, card::get_controler(d, c), LOCATION_MZONE as u8, None, None, None).0 > 0
}
// function s.lvcon(e)
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetMaterialCount()==0
    let h = effect::get_handler(d, a.e).nil();
    card::get_material_count(d, h) == 0
}
// function s.lvop(e,tp,eg,ep,ev,re,r,rp)
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.lvcon)
    effect::set_condition(d, e1, Some(condition(lvcon)));
    // e1:SetValue(4)
    effect::set_value(d, e1, 4);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE&~RESET_TOFIELD)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e2, EFFECT_SET_BASE_ATTACK);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.lvcon)
    effect::set_condition(d, e2, Some(condition(lvcon)));
    // e2:SetValue(1800)
    effect::set_value(d, e2, 1800);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE&~RESET_TOFIELD)
    effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.lvop2(e,tp,eg,ep,ev,re,r,rp)
fn lvop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(4)
    effect::set_value(d, e1, 4);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE)&~(RESET_TOFIELD|RESET_LEAVE))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !(RESET_TOFIELD | RESET_LEAVE)), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e2, EFFECT_SET_BASE_ATTACK);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(1800)
    effect::set_value(d, e2, 1800);
    // e2:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE)&~(RESET_TOFIELD|RESET_LEAVE))
    effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD_DISABLE & !(RESET_TOFIELD | RESET_LEAVE)), 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.immcon(e)
fn immcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsNormalSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_normal_summoned(d, h)
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned() and e:GetLabel()==1
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h) && effect::get_label(d, a.e) == 1
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct1=Duel.GetMatchingGroupCount(Card.IsMonster,tp,LOCATION_GRAVE,0,nil)
    let ct1 = duel::get_matching_group_count(d, Some(&card::is_monster), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64;
    // local ct2=Duel.GetMatchingGroupCount(Card.IsMonster,tp,0,LOCATION_GRAVE,nil)
    let ct2 = duel::get_matching_group_count(d, Some(&card::is_monster), a.tp, 0, LOCATION_GRAVE, NO_EX) as i64;
    // local ct=ct2-ct1
    let ct = ct2 - ct1;
    // if chk==0 then return ct>0 end
    if a.chk == Some(0) {
        return ct > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,ct*300)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, (ct * 300) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,ct*300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (ct * 300) as i32);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // local ct1=Duel.GetMatchingGroupCount(Card.IsMonster,tp,LOCATION_GRAVE,0,nil)
    let ct1 = duel::get_matching_group_count(d, Some(&card::is_monster), a.tp, LOCATION_GRAVE, 0, NO_EX) as i64;
    // local ct2=Duel.GetMatchingGroupCount(Card.IsMonster,tp,0,LOCATION_GRAVE,nil)
    let ct2 = duel::get_matching_group_count(d, Some(&card::is_monster), a.tp, 0, LOCATION_GRAVE, NO_EX) as i64;
    // local ct=ct2-ct1
    let ct = ct2 - ct1;
    // if ct>0 then
    if ct > 0 {
        // local val=Duel.Recover(tp,ct*300,REASON_EFFECT)
        let val = duel::recover(d, a.tp, ct * 300, REASON_EFFECT, false, None);
        // Duel.Damage(1-tp,val,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, val as i64, REASON_EFFECT, false, None);
    }
}
// function s.valcheck(e,c)
fn valcheck(d: &mut Duel, a: Args) -> Value {
    let (e, c) = (a.e(0), a.c(1));
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    let lbl_obj = effect::get_label_object(d, e).as_effect().nil();
    // if g:IsExists(Card.IsSetCard,1,nil,SET_QLI) then
    if group::is_exists(d, &g, &|d, x| card::is_set_card(d, x, &[SET_QLI as u16]), 1, NO_EX) {
        // e:GetLabelObject():SetLabel(1)
        effect::set_label(d, lbl_obj, &[1]);
    } else {
        // e:GetLabelObject():SetLabel(0)
        effect::set_label(d, lbl_obj, &[0]);
    }
    Value::Nil
}
