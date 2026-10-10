//! Gradius' Option
use super::prelude::*;

pub const ID: u32 = 14291024;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[10992251], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon con
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e2, EFFECT_SPSUMMON_CONDITION);
    // e2:SetValue(s.splimit)
    effect::set_value_fn(d, e2, lua_fn(splimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // set target
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetOperation(s.tgop)
    effect::set_operation(d, e3, Some(operation(tgop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // atk.def
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e4, EFFECT_SET_ATTACK_FINAL);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.adcon)
    effect::set_condition(d, e4, Some(condition(adcon)));
    // e4:SetValue(s.atkval)
    effect::set_value_fn(d, e4, effect_card_value(atkval));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_SET_DEFENSE_FINAL)
    effect::set_code(d, e5, EFFECT_SET_DEFENSE_FINAL);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCondition(s.adcon)
    effect::set_condition(d, e5, Some(condition(adcon)));
    // e5:SetValue(s.defval)
    effect::set_value_fn(d, e5, effect_card_value(defval));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // destroy
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e6, EFFECT_SELF_DESTROY);
    // e6:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e6, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCondition(s.sdcon)
    effect::set_condition(d, e6, Some(condition(sdcon)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,10992251),c:GetControler(),LOCATION_MZONE,0,1,nil)
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_code(d, x, &[10992251]))), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.splimit(e,se,sp,st,pos,top)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // if (pos&POS_FACEDOWN)~=0 then return false end
    let pos = a.int(4) as u32;
    if (pos & POS_FACEDOWN as u32) != 0 {
        return false;
    }
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,10992251),top,LOCATION_MZONE,0,1,nil)
    let top = a.p(5);
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_code(d, x, &[10992251]))), top, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsCode,10992251),tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::faceup_filter(|d, x| card::is_code(d, x, &[10992251]))), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // c:SetCardTarget(g:GetFirst())
    let first = group::get_first(d, &g).nil();
    card::set_card_target(d, c, first);
    // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_DISABLE,0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD_DISABLE, 0, 1, 0, 0);
}
/// `s.adcon(e)`
fn adcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFirstCardTarget()~=nil
    let h = effect::get_handler(d, a.e).nil();
    card::get_first_card_target(d, h).is_some()
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetFirstCardTarget():GetAttack()
    let t = card::get_first_card_target(d, c).nil();
    card::get_attack(d, t) as i64
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetFirstCardTarget():GetDefense()
    let t = card::get_first_card_target(d, c).nil();
    card::get_defense(d, t) as i64
}
/// `s.sdcon(e)`
fn sdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFirstCardTarget()==nil and e:GetHandler():GetFlagEffect(id)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_first_card_target(d, h).is_none() && card::get_flag_effect(d, h, ID) != 0
}
