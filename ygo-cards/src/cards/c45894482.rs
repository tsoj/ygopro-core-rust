//! Gilasaurus
use super::prelude::*;

pub const ID: u32 = 45894482;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon procedure
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // Special Summon from the opponent's GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
/// `function(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SPECIAL+1
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SPECIAL + 1
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - a.tp, LOCATION_GRAVE as i32);
    true
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filter),1-tp,LOCATION_GRAVE,0,nil,e,1-tp)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| filter(d, c, e, 1 - tp))), 1 - tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g>0 and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
    if !g.is_empty()
        && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 1))
    {
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, 1 - tp, 1, 1, false, NO_EX).nil();
        duel::special_summon(d, sg, 0, 1 - tp, 1 - tp, false, false, POS_FACEUP as u8, None);
    }
}
