//! Qliphort Carrier
use super::prelude::*;

pub const ID: u32 = 91907707;
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
    // atk up
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_PZONE)
    effect::set_range(d, e3, LOCATION_PZONE);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_QLI))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_QLI as u16]))));
    // e3:SetValue(300)
    effect::set_value(d, e3, 300);
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
    // e7:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e7, EFFECT_IMMUNE_EFFECT);
    // e7:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e7, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e7:SetRange(LOCATION_MZONE)
    effect::set_range(d, e7, LOCATION_MZONE);
    // e7:SetCondition(s.immcon)
    effect::set_condition(d, e7, Some(condition(immcon)));
    // e7:SetValue(aux.qlifilter)
    effect::set_value_fn(d, e7, aux::qlifilter());
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // tohand
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e8, aux::stringid(ID, 1));
    // e8:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e8, CATEGORY_TOHAND);
    // e8:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e8, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e8:SetCode(EVENT_RELEASE)
    effect::set_code(d, e8, EVENT_RELEASE);
    // e8:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e8, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e8:SetTarget(s.thtg)
    effect::set_target(d, e8, Some(cost_target(thtg)));
    // e8:SetOperation(s.thop)
    effect::set_operation(d, e8, Some(operation(thop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_QLI)
    !card::is_set_card(d, c, &[SET_QLI as u16])
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return minc==0 and c:GetLevel()>4 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let tp = card::get_controler(d, c);
    matches!(a.v(2), Value::Int(0))
        && card::get_level(d, c) > 4
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.lvcon(e)`
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetMaterialCount()==0
    let h = effect::get_handler(d, a.e).nil();
    card::get_material_count(d, h) == 0
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
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
/// `s.lvop2(e,tp,eg,ep,ev,re,r,rp)`
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
/// `s.immcon(e)`
fn immcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsNormalSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_normal_summoned(d, h)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
