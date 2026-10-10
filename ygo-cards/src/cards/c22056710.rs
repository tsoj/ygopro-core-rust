//! Vampire Genesis
use super::prelude::*;

pub const ID: u32 = 22056710;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[53839837], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    let _ = card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your hand) by banishing 1 "Vampire Lord" from your Monster Zone
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    effect::set_target(d, e1, Some(lua_fn(selfsptg)));
    effect::set_operation(d, e1, Some(lua_fn(selfspop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Zombie monster from your GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(gysptg)));
    effect::set_operation(d, e2, Some(operation(gyspop)));
    card::register_effect(d, c, e2, false);
}
/// `s.selfspconfilter(c)`
fn selfspconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(53839837) and c:IsFaceup() and c:IsAbleToRemoveAsCost()
    card::is_code(d, c, &[53839837]) && card::is_faceup(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.selfspcon(e,c)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.selfspconfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&selfspconfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return #g>0 and Duel.GetMZoneCount(tp,g)>0
    !g.is_empty() && duel::get_m_zone_count(d, tp, (&g).into(), None, None, None).0 > 0
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.selfspconfilter,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&selfspconfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE,nil,nil,true)
    let chkf = aux::chkf_mmz(1);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&chkf), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive() (groups are values in Rust)
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn selfspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup() (groups are values in Rust)
}
/// `s.gyspcostfilter(c,e,tp)`
fn gyspcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsDiscardable()
    //     and Duel.IsExistingTarget(s.gyspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,c:GetLevel()-1)
    card::is_race(d, c, RACE_ZOMBIE)
        && card::is_discardable(d, c, None, None)
        && {
            let lv = card::get_level(d, c) as i64 - 1;
            duel::is_existing_target(d, Some(&|d, cc| gyspfilter(d, cc, e, tp, lv)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        }
}
/// `s.gyspfilter(c,e,tp,lv)`
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    // return c:IsRace(RACE_ZOMBIE) and c:IsLevelBetween(1,lv) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_ZOMBIE)
        && is_level_between_one(d, c, lv)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `c:IsLevelBetween(1,lv)` (utility.lua: the levels are swapped if needed, then
/// `IsLevelAbove`/`IsLevelBelow` compare with unsigned numbers, so a negative `lv` never matches).
fn is_level_between_one(d: &mut Duel, c: CardId, lv: i64) -> bool {
    lv >= 0 && card::is_level_between(d, c, 1, lv as u32)
}
/// `s.gysptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn gysptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.gyspfilter(chkc,e,tp,e:GetLabel()) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp)
            && card::is_location(d, chkc, LOCATION_GRAVE)
            && gyspfilter(d, chkc, e, tp, effect::get_label(d, e));
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.gyspcostfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| gyspcostfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local dc=Duel.SelectMatchingCard(tp,s.gyspcostfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| gyspcostfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let dc = group::get_first(d, &sel).nil();
    // Duel.SendtoGrave(dc,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, dc, REASON_COST | REASON_DISCARD, None, None);
    // local lv=dc:GetLevel()-1
    let lv = card::get_level(d, dc) as i64 - 1;
    // e:SetLabel(lv)
    effect::set_label(d, e, &[lv]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tg=Duel.SelectTarget(tp,s.gyspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,lv)
    let tg = duel::select_target(d, tp, Some(&|d, c| gyspfilter(d, c, e, tp, lv)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, tg, 1, tp, 0);
    true
}
/// `s.gyspop(e,tp,eg,ep,ev,re,r,rp)`
fn gyspop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
