//! Constellar Siat
use super::prelude::*;

pub const ID: u32 = 44635489;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CONSTELLAR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a| spcon(d, a))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // lv change
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // xyzlimit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CANNOT_BE_XYZ_MATERIAL)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_XYZ_MATERIAL);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetValue(s.xyzlimit)
    effect::set_value_fn(d, e3, lua_fn(|d, a| xyzlimit(d, a)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args<'_>) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.GetFieldGroupCount(c:GetControler(),LOCATION_MZONE,0,nil)==0
    //   and Duel.GetFieldGroupCount(c:GetControler(),0,LOCATION_MZONE,nil)>0
    //   and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let p = card::get_controler(d, c);
    duel::get_field_group_count(d, p, LOCATION_MZONE, 0) == 0
        && duel::get_field_group_count(d, p, 0, LOCATION_MZONE) > 0
        && duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.filter(c,clv)`
fn filter(d: &mut Duel, c: CardId, clv: u32) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return c:IsSetCard(SET_CONSTELLAR) and lv~=0 and lv~=clv
    //   and ((c:IsLocation(LOCATION_MZONE) and c:IsFaceup()) or c:IsLocation(LOCATION_GRAVE))
    card::is_set_card(d, c, &[SET_CONSTELLAR as u16])
        && lv != 0
        && lv != clv
        && ((card::is_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)) || card::is_location(d, c, LOCATION_GRAVE))
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.filter(chkc,e:GetHandler():GetLevel()) end
    if let Some(chkc) = a.chkc() {
        if !card::is_controler(d, chkc, tp) || !card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) {
            return false;
        }
        let h = effect::get_handler(d, a.e).nil();
        let lv = card::get_level(d, h);
        return filter(d, chkc, lv);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,e:GetHandler(),e:GetHandler():GetLevel()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let lv = card::get_level(d, h);
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, lv)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,e:GetHandler(),e:GetHandler():GetLevel())
    let h = effect::get_handler(d, a.e).nil();
    let lv = card::get_level(d, h);
    duel::select_target(d, tp, Some(&|d, c| filter(d, c, lv)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, h.into());
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc:IsRelateToEffect(e)
    //   and (not tc:IsLocation(LOCATION_MZONE) or tc:IsFaceup()) then
    if !card::is_relate_to_effect(d, c, a.e) || !card::is_faceup(d, c) {
        return;
    }
    let tc = tc.nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    if card::is_location(d, tc, LOCATION_MZONE) && !card::is_faceup(d, tc) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetValue(tc:GetLevel())
    let lv = card::get_level(d, tc);
    effect::set_value(d, e1, lv as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // e:GetHandler():RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.xyzlimit(e,c)`
fn xyzlimit(d: &mut Duel, a: Args<'_>) -> bool {
    // if not c then return false end
    let Some(c) = a.card(1) else {
        return false;
    };
    // return not c:IsSetCard(SET_CONSTELLAR)
    !card::is_set_card(d, c, &[SET_CONSTELLAR as u16])
}
