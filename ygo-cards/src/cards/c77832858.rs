//! Thestalos the Shadowfire Monarch
//! Scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 77832858;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute Summon by Tributing 1 monster the opponent controls and 1 Tribute Summoned monster you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.sumcon)
    effect::set_condition(d, e1, Some(lua_fn(sumcon)));
    // e1:SetTarget(s.sumtg)
    effect::set_target(d, e1, Some(lua_fn(sumtg)));
    // e1:SetOperation(s.sumop)
    effect::set_operation(d, e1, Some(lua_fn(sumop)));
    // e1:SetValue(SUMMON_TYPE_TRIBUTE)
    effect::set_value(d, e1, SUMMON_TYPE_TRIBUTE as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_SET_PROC)
    effect::set_code(d, e2, EFFECT_SET_PROC);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish 1 random card from the opponent's hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_REMOVE | CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(function(e) return e:GetHandler():IsTributeSummoned() end)
    effect::set_condition(d, e3, Some(lua_fn(|d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        card::is_tribute_summoned(d, h)
    })));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    // e4:SetValue(s.valcheck)
    effect::set_value_fn(d, e4, effect_card_value(valcheck));
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// The optional number argument of a Lua function (`nil` = `None`)
fn num(v: &Value) -> Option<u32> {
    if v.is_nil() { None } else { Some(v.int() as u32) }
}
/// `s.cfilter(c,relzone,tp)`
fn cfilter(d: &mut Duel, c: CardId, relzone: u32, tp: u8) -> bool {
    // return aux.IsZone(c,relzone,tp) and c:IsReleasable() and (c:IsTributeSummoned() or c:IsControler(1-tp))
    aux::is_zone(d, c, relzone, tp) && card::is_releasable(d, c, None, None) && (card::is_tribute_summoned(d, c) || card::is_controler(d, c, 1 - tp))
}
/// `Duel.GetMatchingGroup(Card.IsHasEffect,tp,LOCATION_MZONE,LOCATION_MZONE,nil,EFFECT_EXTRA_RELEASE)`
fn soul_ex_group(d: &mut Duel, tp: u8) -> CardSet {
    duel::get_matching_group(d, Some(&|d, x| !card::is_has_effect(d, x, EFFECT_EXTRA_RELEASE, None).is_empty()), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX)
}
/// `s.rescon(soul_ex_g,zone)`: the `rescon` of the Tribute group selection
fn rescon(soul: CardSet, zone: Option<u32>) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>| -> (bool, bool) {
        // return (#soul_ex_g==0 or sg&soul_ex_g==soul_ex_g) and Duel.GetMZoneCount(tp,sg,tp,LOCATION_REASON_TOFIELD,zone)>0
        //     and sg:FilterCount(Card.IsControler,nil,tp)==1
        let subset = soul.is_empty() || sg.iter().filter(|x| soul.contains(*x)).count() == soul.len();
        let res = subset
            && duel::get_m_zone_count(d, tp, sg.into(), Some(tp), Some(LOCATION_REASON_TOFIELD), zone).0 > 0
            && group::filter_count(d, sg, &|d, x| card::is_controler(d, x, tp), NO_EX) == 1;
        (res, false)
    }
}
/// `s.sumcon(e,c,minc,zone,relzone,exeff)`
fn sumcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // if minc>2 or c:IsLevelBelow(6) then return false end
    if a.int(2) > 2 || card::is_level_below(d, c, 6) {
        return false;
    }
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    let relzone = a.int(4) as u32;
    let zone = num(a.v(3));
    // local mg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,relzone,tp)
    let mg = duel::get_matching_group(d, Some(&|d, x| cfilter(d, x, relzone, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local soul_ex_g=Duel.GetMatchingGroup(Card.IsHasEffect,...)
    let soul = soul_ex_group(d, tp);
    // return #mg>=2 and aux.SelectUnselectGroup(mg,e,tp,2,2,s.rescon(soul_ex_g,zone),0)
    if mg.len() < 2 {
        return false;
    }
    let rs = rescon(soul, zone);
    matches!(aux::select_unselect_group(d, &mg, Some(e), tp, Some(2), Some(2), Some(&rs), 0, tp, None, None, None, false), Err(true))
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk,c,minc,zone,relzone,exeff)`
fn sumtg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let zone = num(a.v(11));
    let relzone = a.int(12) as u32;
    // local mg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,relzone,tp)
    let mg = duel::get_matching_group(d, Some(&|d, x| cfilter(d, x, relzone, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local soul_ex_g=Duel.GetMatchingGroup(Card.IsHasEffect,...)
    let soul = soul_ex_group(d, tp);
    // local g=aux.SelectUnselectGroup(mg,e,tp,2,2,s.rescon(soul_ex_g,zone),1,tp,HINTMSG_RELEASE,nil,nil,true)
    let rs = rescon(soul, zone);
    let g = aux::select_unselect_group(d, &mg, Some(e), tp, Some(2), Some(2), Some(&rs), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true).unwrap_or_default();
    // if g and #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp,c,minc,zone,relzone,exeff)`
fn sumop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(8);
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned().nil();
    // c:SetMaterial(g)
    card::set_material(d, c, g.clone());
    // Duel.Release(g,REASON_SUMMON|REASON_MATERIAL)
    duel::release(d, g, REASON_SUMMON | REASON_MATERIAL, None);
    // g:DeleteGroup()
}
/// `s.rmtg(e,tp,...,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_HAND,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1000);
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,PLAYER_ALL,LOCATION_ONFIELD)
        duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    }
    true
}
/// `s.rmop(e,tp,...)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_HAND,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_HAND, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local rg=g:RandomSelect(tp,1)
    let rg = group::random_select(d, &g, tp, 1);
    // if #rg>0 and Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)>0 and Duel.Damage(1-tp,1000,REASON_EFFECT)>0
    //     and Duel.GetPossibleOperationInfo(0,CATEGORY_REMOVE)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !rg.is_empty()
        && duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && duel::damage(d, 1 - tp, 1000, REASON_EFFECT, false, None) > 0
        && duel::get_possible_operation_info(d, 0, CATEGORY_REMOVE).is_some()
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local rc=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        let rc = group::get_first(d, &sel);
        // if rc and Duel.Remove(rc,POS_FACEUP,REASON_EFFECT)>0 and rc:IsAttribute(ATTRIBUTE_FIRE|ATTRIBUTE_DARK) and rc:HasLevel() then
        if let Some(rc) = rc {
            if duel::remove(d, rc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
                && card::is_attribute(d, rc, ATTRIBUTE_FIRE | ATTRIBUTE_DARK)
                && card::has_level(d, rc)
            {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Damage(1-tp,rc:GetLevel()*200,REASON_EFFECT)
                let lv = card::get_level(d, rc) as i64;
                duel::damage(d, 1 - tp, lv * 200, REASON_EFFECT, false, None);
            }
        }
    }
}
/// `s.valfilter(c)`
fn valfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(8) and c:IsTributeSummoned()
    card::is_level_above(d, c, 8) && card::is_tribute_summoned(d, c)
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    let lo = effect::get_label_object(d, e).as_effect().nil();
    // if g:IsExists(s.valfilter,1,nil) then
    if group::is_exists(d, &g, &|d, x| valfilter(d, x), 1, NO_EX) {
        // e:GetLabelObject():SetLabel(1)
        effect::set_label(d, lo, &[1]);
    } else {
        // e:GetLabelObject():SetLabel(0)
        effect::set_label(d, lo, &[0]);
    }
    0
}
