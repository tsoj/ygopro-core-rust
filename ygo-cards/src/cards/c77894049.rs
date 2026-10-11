//! Vanquish Soul Rocks
//! ＶＳ ロックス
use super::prelude::*;

pub const ID: u32 = 77894049;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[28168628], listed_series: &[SET_VANQUISH_SOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 4 "Vanquish Soul" monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_VANQUISH_SOUL),4,2,s.ovfilter,aux.Stringid(id,0),Xyz.InfiniteMats,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_VANQUISH_SOUL as u16], sc, st, Some(tp)))),
        Some(4),
        2,
        Some(xyz::alter_filter(|d, c, tp, lc| ovfilter(d, c, tp, lc))),
        Some(aux::stringid(ID, 0)),
        Some(xyz::INFINITE_MATS),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // All monsters your opponent controls lose 800 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetCondition(s.attrcon(ATTRIBUTE_DARK))
    effect::set_condition(d, e1, Some(attrcon(ATTRIBUTE_DARK)));
    // e1:SetValue(-800)
    effect::set_value(d, e1, -800);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // "Vanquish Soul" monsters you control gain 1000 ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCondition(s.attrcon(ATTRIBUTE_FIRE))
    effect::set_condition(d, e2, Some(attrcon(ATTRIBUTE_FIRE)));
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_VANQUISH_SOUL))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16]))));
    // e2:SetValue(1000)
    effect::set_value(d, e2, 1000);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 card your opponent controls
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.attrcon(ATTRIBUTE_EARTH))
    effect::set_condition(d, e3, Some(attrcon(ATTRIBUTE_EARTH)));
    // e3:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Track battles involving "Vanquish Soul" monsters
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_BATTLED)
        effect::set_code(d, ge1, EVENT_BATTLED);
        // ge1:SetOperation(s.regop)
        effect::set_operation(d, ge1, Some(operation(regop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, _a: &EffArgs) {
    // local a,b=Duel.GetBattleMonster(0)
    let (ma, mb) = duel::get_battle_monster(d, 0);
    // if (a and a:IsSetCard(SET_VANQUISH_SOUL)) or (b and b:IsSetCard(SET_VANQUISH_SOUL)) then
    let a_vs = match ma {
        Some(x) => card::is_set_card(d, x, &[SET_VANQUISH_SOUL as u16]),
        None => false,
    };
    let b_vs = match mb {
        Some(x) => card::is_set_card(d, x, &[SET_VANQUISH_SOUL as u16]),
        None => false,
    };
    if a_vs || b_vs {
        // Duel.RegisterFlagEffect(0,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, 0, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
// function s.ovfilter(c,tp,lc)
//     return (c:IsSetCard(SET_VANQUISH_SOUL,lc,SUMMON_TYPE_XYZ,tp) or c:IsCode(28168628)) and c:IsFaceup()
// end
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    (card::is_set_card_ctx(d, c, &[SET_VANQUISH_SOUL as u16], Some(lc), SUMMON_TYPE_XYZ as u64, Some(tp))
        || card::is_code(d, c, &[28168628]))
        && card::is_faceup(d, c)
}
// function s.xyzop(e,tp,chk)
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return not Duel.HasFlagEffect(tp,id) and Duel.HasFlagEffect(0,id+1) end
    if a.int(2) == 0 {
        return !duel::has_flag_effect(d, tp, ID, None) && duel::has_flag_effect(d, 0, ID + 1, None);
    }
    // return Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0);
    true
}
// function s.attrcon(attr)
//     return function(e) return e:GetHandler():GetOverlayGroup():IsExists(Card.IsAttribute,1,nil,attr) end
// end
fn attrcon(attr: u32) -> LuaFn {
    lua_fn(move |d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        let g = card::get_overlay_group(d, h);
        group::is_exists(d, &g, &|d, c| card::is_attribute(d, c, attr), 1, NO_EX)
    })
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
