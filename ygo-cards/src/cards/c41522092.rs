//! Number F0: Utopic Future Zexal
//! ＦＮｏ．０ 未来皇ホープ・ゼアル
use super::prelude::*;

pub const ID: u32 = 41522092;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(0), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Xyz Monsters with the same Rank
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_XYZ),nil,2,nil,nil,nil,nil,false,s.xyzcheck)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_XYZ, sc, st, Some(tp)))),
        None,
        2,
        None,
        None,
        None,
        None,
        false,
        Some(std::sync::Arc::new(xyzcheck)),
    );
    // Gains ATK/DEF equal to the total Ranks of all Xyz Monsters you control and in your opponent's GY x 500
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsType,TYPE_XYZ),e:GetHandlerPlayer(),LOCATION_MZONE,LOCATION_GRAVE,nil):GetSum(Card.GetRank)*500 end)
    effect::set_value_fn(d, e1, effect_card_value(atk_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Your opponent's monsters cannot target monsters for attacks, except this one
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetValue(function(e,c) return c~=e:GetHandler() end)
    effect::set_value_fn(d, e3, lua_fn(e3_value));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Your opponent cannot target target other cards on the field with card effects
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e4, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e4:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(LOCATION_ONFIELD,LOCATION_ONFIELD)
    effect::set_target_range(d, e4, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // e4:SetTarget(function(e,c) return c~=e:GetHandler() end)
    effect::set_target(d, e4, Some(effect_card_fn(|d, e, c| effect::get_handler(d, e) != Some(c))));
    // e4:SetValue(aux.tgoval)
    effect::set_value_fn(d, e4, aux::tgoval());
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Take control of 1 monster your opponent controls
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e5, CATEGORY_CONTROL);
    // e5:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e5, EFFECT_TYPE_QUICK_O);
    // e5:SetCode(EVENT_CHAINING)
    effect::set_code(d, e5, EVENT_CHAINING);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetCondition(s.ctcon)
    effect::set_condition(d, e5, Some(condition(ctcon)));
    // e5:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e5, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e5:SetTarget(s.cttg)
    effect::set_target(d, e5, Some(cost_target(cttg)));
    // e5:SetOperation(s.ctop)
    effect::set_operation(d, e5, Some(operation(ctop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `function(e,c) return Duel.GetMatchingGroup(...):GetSum(Card.GetRank)*500 end` (e1's value)
fn atk_value(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let hp = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, x| card::is_type(d, x, TYPE_XYZ))), hp, LOCATION_MZONE, LOCATION_GRAVE, NO_EX);
    group::get_sum(d, &g, &|d, x| card::get_rank(d, x) as i64) * 500
}
/// `function(e,c) return c~=e:GetHandler() end` (e3's value)
fn e3_value(d: &mut Duel, a: Args) -> bool {
    a.card(1) != effect::get_handler(d, a.e(0))
}
/// `s.xyzcheck(g,tp,xyz)`
fn xyzcheck(d: &mut Duel, g: &CardSet, _tp: u8, _xyz: CardId) -> bool {
    // local mg=g:Filter(function(c) return not c:IsHasEffect(EFFECT_EQUIP_SPELL_XYZ_MAT) end,nil)
    let mg = group::filter(d, g, &|d, c| card::is_has_effect(d, c, EFFECT_EQUIP_SPELL_XYZ_MAT, None).is_empty(), NO_EX);
    // return mg:GetClassCount(Card.GetRank)==1
    group::get_class_count(d, &mg, &|d, c| card::get_rank(d, c) as i64) == 1
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local trig_loc,trig_p=Duel.GetChainInfo(0,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_PLAYER)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_PLAYER]);
    let trig_loc = info[0].int();
    let trig_p = info[1].int();
    // return trig_p==1-tp and (trig_loc&LOCATION_ONFIELD)>0
    trig_p == (1 - a.tp) as i64 && (trig_loc & LOCATION_ONFIELD as i64) > 0
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_controler_can_be_changed(d, x, false, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, tp, 0);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_controler_can_be_changed(d, x, false, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.GetControl(g,tp)
        duel::get_control(d, g, tp, 0, 0, None, None);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // This card cannot be destroyed by battle or card effects this turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3008)
        effect::set_description(d, e1, 3008);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
