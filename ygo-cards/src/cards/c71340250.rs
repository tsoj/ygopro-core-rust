//! Mixeroid
use super::prelude::*;

pub const ID: u32 = 71340250;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 non-WIND "roid" monster from your Deck
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
    // Special Summon 1 "roid" Fusion Monster from your Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.exspcost)
    effect::set_cost(d, e2, Some(cost_target(exspcost)));
    // e2:SetTarget(s.exsptg)
    effect::set_target(d, e2, Some(cost_target(exsptg)));
    // e2:SetOperation(s.exspop)
    effect::set_operation(d, e2, Some(operation(exspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spcostfilter(c,tp)
fn spcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_MACHINE) and Duel.GetMZoneCount(tp,c)>0
    card::is_race(d, c, RACE_MACHINE) && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.spcostfilter,1,false,nil,nil,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| spcostfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,s.spcostfilter,1,1,false,nil,nil,tp)
    let sg = duel::select_release_group_cost(d, a.tp, Some(&|d, c| spcostfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    true
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ROID) and c:IsAttributeExcept(ATTRIBUTE_WIND) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ROID as u16])
        && card::is_attribute_except(d, c, ATTRIBUTE_WIND, None, 0, None)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.exspcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn exspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsAbleToRemoveAsCost()
    card::is_race(d, c, RACE_MACHINE) && card::is_able_to_remove_as_cost(d, c, None)
}
// function s.exspfilter(c,e,tp,lv)
fn exspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    // return c:IsSetCard(SET_ROID) and c:IsType(TYPE_FUSION) and c:IsLevelBelow(lv) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_ROID as u16])
        && card::is_type(d, c, TYPE_FUSION)
        && card::is_level_below(d, c, lv as u32)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
// function s.exsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn exsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // local cg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE,0,nil)
        let cg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
        let n = cg.len() as i64;
        // return c:IsAbleToRemoveAsCost() and Duel.IsExistingMatchingCard(s.exspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,#cg)
        return card::is_able_to_remove_as_cost(d, c, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| exspfilter(d, x, a.e, a.tp, n)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.PayLPCost(tp,math.floor(Duel.GetLP(tp)/2))
    let half = duel::get_lp(d, a.tp).div_euclid(2);
    duel::pay_lp_cost(d, a.tp, half as u32);
    // local cg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE,0,nil)
    let cg = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    let n = cg.len() as i64;
    // local tg=Duel.GetMatchingGroup(s.exspfilter,tp,LOCATION_EXTRA,0,nil,e,tp,#cg)
    let tg = duel::get_matching_group(d, Some(&|d, x| exspfilter(d, x, a.e, a.tp, n)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // local lvt={}
    // local tc=tg:GetFirst()
    let _ = group::get_first(d, &tg);
    // for tc in aux.Next(tg) do
    //     local tlv=0
    //     tlv=tlv+tc:GetLevel()
    //     lvt[tlv]=tlv
    // end
    let mut levels: Vec<u64> = Vec::new();
    for tc in group::iter(d, &tg) {
        let tlv = card::get_level(d, tc) as u64;
        levels.push(tlv);
    }
    // local pc=1
    // for i=1,12 do
    //     if lvt[i] then lvt[i]=nil lvt[pc]=i pc=pc+1 end
    // end
    // lvt[pc]=nil
    // (the levels 1 to 12 that are present, in order)
    let lvt: Vec<u64> = (1..=12u64).filter(|i| levels.contains(i)).collect();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local lv=Duel.AnnounceNumber(tp,table.unpack(lvt))
    let lv = duel::announce_number(d, a.tp, &lvt).map_or(0, |x| x.0) as i64;
    // local rg1=Group.CreateGroup()
    let mut rg1 = CardSet::new();
    // if lv>1 then
    if lv > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local rg2=cg:Select(tp,lv-1,lv-1,c)
        let rg2 = group::select(d, &cg, a.tp, (lv - 1) as u16, (lv - 1) as u16, false, c.into()).nil();
        // rg1:Merge(rg2)
        group::add_card(&mut rg1, &rg2);
    }
    // rg1:AddCard(c)
    group::add_card(&mut rg1, &CardSet::from_card(c));
    // Duel.Remove(rg1,POS_FACEUP,REASON_COST)
    duel::remove(d, rg1, POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(lv)
    effect::set_label(d, a.e, &[lv]);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.sfilter(c,e,tp,lv)
fn sfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    // return c:IsSetCard(SET_ROID) and c:IsType(TYPE_FUSION) and c:GetLevel()==lv and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_ROID as u16])
        && card::is_type(d, c, TYPE_FUSION)
        && card::get_level(d, c) as i64 == lv
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
// function s.exspop(e,tp,eg,ep,ev,re,r,rp)
fn exspop(d: &mut Duel, a: &EffArgs) {
    // local lv=e:GetLabel()
    let lv = effect::get_label(d, a.e);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.sfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,lv)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| sfilter(d, c, a.e, a.tp, lv)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,true,false,POS_FACEUP)~=0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None) != 0 {
            let h = effect::get_handler(d, a.e).nil();
            // local fid=e:GetHandler():GetFieldID()
            let fid = card::get_field_id(d, h);
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e1:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetLabel(fid)
            effect::set_label(d, e1, &[fid as i64]);
            // e1:SetLabelObject(tc)
            effect::set_label_object(d, e1, Value::Card(tc));
            // e1:SetCondition(s.descon)
            effect::set_condition(d, e1, Some(condition(descon)));
            // e1:SetOperation(s.desop)
            effect::set_operation(d, e1, Some(operation(desop)));
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
        }
    }
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffectLabel(id)==e:GetLabel() then
    if card::get_flag_effect_label(d, tc, ID).first().copied() == Some(effect::get_label(d, a.e)) {
        // return true
        return true;
    }
    // e:Reset()
    effect::reset(d, a.e);
    // return false
    false
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Destroy(tc,REASON_EFFECT)
    duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
}
