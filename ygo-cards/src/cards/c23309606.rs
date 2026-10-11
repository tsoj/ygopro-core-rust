//! Infernal Incinerator
use super::prelude::*;

pub const ID: u32 = 23309606;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_LIMIT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_LIMIT_SUMMON_PROC);
    // e1:SetCondition(s.otcon)
    effect::set_condition(d, e1, Some(lua_fn(otcon)));
    // e1:SetTarget(s.ottg)
    effect::set_target(d, e1, Some(lua_fn(ottg)));
    // e1:SetOperation(s.otop)
    effect::set_operation(d, e1, Some(lua_fn(otop)));
    // e1:SetValue(SUMMON_TYPE_TRIBUTE)
    effect::set_value(d, e1, SUMMON_TYPE_TRIBUTE as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_LIMIT_SET_PROC)
    effect::set_code(d, e2, EFFECT_LIMIT_SET_PROC);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atk
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetValue(s.val)
    effect::set_value_fn(d, e3, effect_card_value(val));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.otfilter(c,tp)`
fn otfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttackAbove(2000) and (c:IsControler(tp) or c:IsFaceup())
    card::is_attack_above(d, c, 2000) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.otcon(e,c,minc)`
fn otcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    let minc = a.v(2).int();
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local hg=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let mut hg = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    // hg:RemoveCard(c)
    group::remove_card(&mut hg, &CardSet::from_card(c));
    // local mg=Duel.GetMatchingGroup(s.otfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp)
    let mg = duel::get_matching_group(d, Some(&|d, x| otfilter(d, x, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // return #hg>0 and minc<=1 and Duel.CheckTribute(c,1,1,mg)
    //     and hg:FilterCount(Card.IsAbleToGraveAsCost,nil)==#hg
    !hg.is_empty()
        && minc <= 1
        && duel::check_tribute(d, c, 1, Some(1), Some(&mg), None, None)
        && group::filter_count(d, &hg, &|d, x| card::is_able_to_grave_as_cost(d, x), NO_EX) == hg.len()
}
/// `s.ottg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn ottg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    let c = a.c(9);
    // local mg=Duel.GetMatchingGroup(s.otfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tp)
    let mg = duel::get_matching_group(d, Some(&|d, x| otfilter(d, x, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local sg=Duel.SelectTribute(tp,c,1,1,mg,nil,nil,true)
    let sg = duel::select_tribute(d, tp, c, 1, 1, Some(&mg), None, None, true);
    // if sg and #sg>0 then
    if let Some(sg) = sg {
        if !sg.is_empty() {
            // sg:KeepAlive() (groups are values in Rust)
            // e:SetLabelObject(sg)
            effect::set_label_object(d, e, Value::Group(sg));
            // return true
            return true;
        }
    }
    // return false
    false
}
/// `s.otop(e,tp,eg,ep,ev,re,r,rp,c)`
fn otop(d: &mut Duel, a: Args) {
    let (e, tp) = (a.e(0), a.p(1));
    let c = a.c(8);
    // local sg=e:GetLabelObject()
    // if not sg then return end
    let Some(sg) = effect::get_label_object(d, e).as_group().cloned() else { return };
    // local hg=Duel.GetFieldGroup(tp,LOCATION_HAND,0)-c
    let mut hg = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    group::remove_card(&mut hg, &CardSet::from_card(c));
    // Duel.SendtoGrave(hg,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, hg, REASON_COST | REASON_DISCARD, None, None);
    // c:SetMaterial(sg)
    card::set_material(d, c, sg.clone());
    // Duel.Release(sg,REASON_SUMMON|REASON_MATERIAL)
    duel::release(d, &sg, REASON_SUMMON | REASON_MATERIAL, None);
    // sg:DeleteGroup() (groups are values in Rust)
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)*-500+500+Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)*200
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) as i64 * -500 + 500 + duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) as i64 * 200
}
