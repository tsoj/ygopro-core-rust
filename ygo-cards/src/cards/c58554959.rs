//! Evil HERO Malicious Edge
use super::prelude::*;

pub const ID: u32 = 58554959;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 1 tribute
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    effect::set_condition(d, e1, Some(lua_fn(otcon)));
    effect::set_operation(d, e1, Some(lua_fn(otop)));
    effect::set_value(d, e1, SUMMON_TYPE_TRIBUTE as i64);
    card::register_effect(d, c, e1, false);
    // pierce
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_PIERCE);
    card::register_effect(d, c, e2, false);
}
/// `s.otcon(e,c,minc)`
fn otcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return Duel.GetFieldGroupCount(c:GetControler(),0,LOCATION_MZONE)>0 and c:GetLevel()>6 and minc<=1 and Duel.CheckTribute(c,1)
    let tp = card::get_controler(d, c);
    duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
        && card::get_level(d, c) > 6
        && a.int(2) <= 1
        && duel::check_tribute(d, c, 1, None, None, None, None)
}
/// `s.otop(e,tp,eg,ep,ev,re,r,rp,c)`
fn otop(d: &mut Duel, a: Args) {
    let (tp, c) = (a.p(1), a.c(8));
    // local sg=Duel.SelectTribute(tp,c,1,1)
    let sg = duel::select_tribute(d, tp, c, 1, 1, None, None, None, false).nil();
    // c:SetMaterial(sg)
    card::set_material(d, c, sg.clone());
    // Duel.Release(sg,REASON_SUMMON|REASON_MATERIAL)
    duel::release(d, sg, REASON_SUMMON | REASON_MATERIAL, None);
}
