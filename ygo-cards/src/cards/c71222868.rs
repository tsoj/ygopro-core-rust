//! Raidraptor - Rising Rebellion Falcon
//! ＲＲ－ライジング・リベリオン・ファルコン
use super::prelude::*;

pub const ID: u32 = 71222868;

// s.listed_series={SET_RAIDRAPTOR}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RAIDRAPTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_WINGEDBEAST),13,5)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_WINGEDBEAST, sc, st, Some(tp)))),
        Some(13),
        5,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // Destroy as many cards your opponent controls as possible
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Unaffected by other cards' effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(function(e,te) return te:GetOwner()~=e:GetOwner() end)
    effect::set_value_fn(
        d,
        e2,
        lua_fn(|d, a: Args| {
            let e = a.e(0);
            let te = a.e(1);
            let owner_te = effect::get_owner(d, te).nil();
            let owner_e = effect::get_owner(d, e).nil();
            owner_te != owner_e
        }),
    );
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Copy the effects of a "Raidraptor" Xyz Monster in your GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(Cost.DetachFromSelf(3,3,nil))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(3), Some(cost::Count::N(3)), None)));
    // e3:SetTarget(s.copytg)
    effect::set_target(d, e3, Some(cost_target(copytg)));
    // e3:SetOperation(s.copyop)
    effect::set_operation(d, e3, Some(operation(copyop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():IsXyzSummoned() end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 0);
    true
}
/// `s.dmgfilter(c)`
fn dmgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:GetBaseAttack()>0
    card::is_previous_location(d, c, LOCATION_MZONE) && card::get_base_attack(d, c) > 0
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsSetCard(SET_RAIDRAPTOR)
    card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16])
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // if Duel.Destroy(g,REASON_EFFECT)==0 then return end
    if duel::destroy(d, g, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local og=Duel.GetOperatedGroup():Filter(s.dmgfilter,nil)
    let operated = duel::get_operated_group(d);
    let og = group::filter(d, &operated, &dmgfilter, NO_EX);
    // if #og==0 then return end
    if og.is_empty() {
        return;
    }
    // local dam=og:GetSum(Card.GetBaseAttack)
    let dam = group::get_sum(d, &og, &|d, c| card::get_base_attack(d, c) as i64);
    // local mg=c:GetOverlayGroup():Filter(s.matfilter,nil)
    let ov = card::get_overlay_group(d, c);
    let mg = group::filter(d, &ov, &matfilter, NO_EX);
    // if mg:GetClassCount(Card.GetCode)>=3 then
    if group::get_class_count(d, &mg, &|d, c| card::get_code(d, c)[0] as i64) >= 3 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,dam,REASON_EFFECT)
        duel::damage(d, 1 - tp, dam, REASON_EFFECT, false, None);
    }
}
/// `s.copyfilter(c)`
fn copyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsSetCard(SET_RAIDRAPTOR)
    card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16])
}
/// `s.copytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn copytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.copyfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && copyfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.copyfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&copyfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.copyfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&copyfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.copyop(e,tp,eg,ep,ev,re,r,rp)`
fn copyop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // and tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, e) {
            // local code=tc:GetOriginalCodeRule()
            let code = card::get_original_code_rule(d, tc)[0];
            // c:CopyEffect(code,RESETS_STANDARD_PHASE_END,1)
            card::copy_effect(d, c, code, RESETS_STANDARD_PHASE_END, 1);
        }
    }
}
