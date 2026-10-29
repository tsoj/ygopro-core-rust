//! Fullmetalfoes Alkahest
use super::prelude::*;

pub const ID: u32 = 77693536;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_METALFOES], material_setcode: &[SET_METALFOES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_METALFOES),aux.FilterBoolFunctionEx(Card.IsType,TYPE_NORMAL))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_METALFOES as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_NORMAL, Some(fc), st, Some(tp)))),
        ],
    );
    // Must be Fusion Summoned
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e0, aux::fuslimit());
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Equip 1 Effect Monster on the field to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,aux.FilterBoolFunction(Card.IsType,TYPE_EFFECT),function(c,e,tp,tc) c:EquipByEffectAndLimitRegister(e,tp,tc,id,true) end,e1)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| card::is_type(d, a.c(0), TYPE_EFFECT)),
        lua_fn(|d, a| {
            card::equip_by_effect_and_limit_register(d, a.c(0), a.e(1), a.p(2), a.c(3), Some(ID), Some(true));
        }),
        e1,
        None,
        None,
        None,
    );
    // Gains DEF equal to the combine original ATK of the monsters equipped to it by its effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.defval)
    effect::set_value_fn(d, e2, effect_value(defval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Can use monsters you control equipped to this card you control as material for the Fusion Summon of a "Metalfoes" Fusion Monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_EXTRA_FUSION_MATERIAL)
    effect::set_code(d, e3, EFFECT_EXTRA_FUSION_MATERIAL);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_SZONE,0)
    effect::set_target_range(d, e3, LOCATION_SZONE, 0);
    // e3:SetTarget(function(e,c) return c:GetEquipTarget()==e:GetHandler() and c:IsOriginalType(TYPE_MONSTER) end)
    effect::set_target(d, e3, Some(effect_card_fn(e3_target)));
    // e3:SetValue(function(e,c) return c and c:IsSetCard(SET_METALFOES) and c:IsControler(e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e3, lua_fn(e3_value));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and (c:IsControler(tp) or c:IsAbleToChangeControler())
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && (card::is_controler(d, c, tp) || card::is_able_to_change_controler(d, c))
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.eqfilter(chkc,tp) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && eqfilter(d, chkc, tp) && chkc != c;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.eqfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| eqfilter(d, x, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.eqfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| eqfilter(d, x, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g), 1, tp, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsType(TYPE_EFFECT) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_type(d, tc, TYPE_EFFECT) {
        // c:EquipByEffectAndLimitRegister(e,tp,tc,id,true)
        card::equip_by_effect_and_limit_register(d, c, a.e, tp, tc, Some(ID), Some(true));
    }
}
/// `s.defvalfilter(c)`
fn defvalfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasFlagEffect(id) and c:GetTextAttack()>=0
    card::get_flag_effect(d, c, ID) > 0 && card::get_text_attack(d, c) >= 0
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, e: EffectId) -> i64 {
    // return e:GetHandler():GetEquipGroup():Filter(s.defvalfilter,nil):GetSum(Card.GetTextAttack)
    let h = effect::get_handler(d, e).nil();
    let g = card::get_equip_group(d, h);
    let f = group::filter(d, &g, &|d, x| defvalfilter(d, x), NO_EX);
    group::get_sum(d, &f, &|d, x| card::get_text_attack(d, x) as i64)
}
/// `s.eqfilter` target of the Extra Fusion Material effect: `function(e,c) return c:GetEquipTarget()==e:GetHandler() and c:IsOriginalType(TYPE_MONSTER) end`
fn e3_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    card::get_equip_target(d, c) == Some(h) && card::is_original_type(d, c, TYPE_MONSTER)
}
/// `function(e,c) return c and c:IsSetCard(SET_METALFOES) and c:IsControler(e:GetHandlerPlayer()) end`
fn e3_value(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else {
        return false;
    };
    let p = effect::get_handler_player(d, a.e(0));
    card::is_set_card(d, c, &[SET_METALFOES as u16]) && card::is_controler(d, c, p)
}
