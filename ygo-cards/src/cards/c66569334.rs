//! Nightmell the Dark Bonder
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 66569334;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// The values `s.getprops(c)` returns, in order: `GetSummonLocation, GetOriginalRace,
/// GetOriginalAttribute, GetOriginalLevel, GetTextAttack, GetTextDefense, GetOriginalCode`.
struct Props {
    sumloc: u32,
    race: u64,
    att: u32,
    lv: u32,
    atk: i32,
    def: i32,
    code: u32,
}
/// `s.getprops(c)`
fn getprops(d: &mut Duel, c: CardId) -> Props {
    // return c:GetSummonLocation(),c:GetOriginalRace(),c:GetOriginalAttribute(),c:GetOriginalLevel(),c:GetTextAttack(),c:GetTextDefense(),c:GetOriginalCode()
    Props {
        sumloc: card::get_summon_location(d, c),
        race: card::get_original_race(d, c),
        att: card::get_original_attribute(d, c),
        lv: card::get_original_level(d, c),
        atk: card::get_text_attack(d, c),
        def: card::get_text_defense(d, c),
        code: card::get_original_code(d, c),
    }
}
/// `s.spfilter(c,e,tp,cc,sumloc,race,att,lv,atk,def,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, cc: Option<CardId>, p: &Props) -> bool {
    // if c:IsLocation(LOCATION_EXTRA) and (sumloc~=LOCATION_EXTRA or Duel.GetLocationCountFromEx(tp,tp,cc,c)<1) then return false end
    if card::is_location(d, c, LOCATION_EXTRA) {
        let ex = cc.map_or(NO_EX, |x| x.into());
        if p.sumloc != LOCATION_EXTRA || duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 < 1 {
            return false;
        }
    }
    // return c:HasLevel() and not c:IsOriginalCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and c:IsOriginalRace(race) and c:IsOriginalAttribute(att)
    //     and c:GetOriginalLevel()==lv and c:GetTextAttack()==atk and c:GetTextDefense()==def
    card::has_level(d, c)
        && !card::is_original_code(d, c, &[p.code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_original_race(d, c, p.race)
        && card::is_original_attribute(d, c, p.att)
        && card::get_original_level(d, c) == p.lv
        && card::get_text_attack(d, c) == p.atk
        && card::get_text_defense(d, c) == p.def
}
/// `s.spcostfilter(c,e,tp)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local loc=Duel.GetMZoneCount(tp,c)>0 and LOCATION_DECK|LOCATION_EXTRA or LOCATION_EXTRA
    let loc = if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0 { LOCATION_DECK | LOCATION_EXTRA } else { LOCATION_EXTRA };
    // return c:HasLevel() and Duel.IsExistingMatchingCard(s.spfilter,tp,loc,0,1,nil,e,tp,c,s.getprops(c))
    if !card::has_level(d, c) {
        return false;
    }
    let p = getprops(d, c);
    duel::is_existing_matching_card(d, Some(&|d, sc| spfilter(d, sc, e, tp, Some(c), &p)), tp, loc, 0, 1, NO_EX)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.spcostfilter,1,false,nil,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| spcostfilter(d, c, a.e, a.tp)), 1, 1, false, None, NO_EX);
    }
    // local rc=Duel.SelectReleaseGroupCost(tp,s.spcostfilter,1,1,false,nil,nil,e,tp):GetFirst()
    let rg = duel::select_release_group_cost(d, a.tp, Some(&|d, c| spcostfilter(d, c, a.e, a.tp)), 1, 1, false, None, NO_EX);
    let rc = group::get_first(d, &rg).nil();
    // e:SetLabelObject(rc)
    effect::set_label_object(d, a.e, Value::Card(rc));
    // Duel.Release(rc,REASON_COST)
    duel::release(d, rc, REASON_COST, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local rc=e:GetLabelObject()
    let rc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e,tp,nil,s.getprops(rc))
    let p = getprops(d, rc);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, None, &p)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, &g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
