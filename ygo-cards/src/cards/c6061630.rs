//! Magical Exemplar
use super::prelude::*;

pub const ID: u32 = 6061630;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[COUNTER_SPELL as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // add counter
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_CHAINING)
    effect::set_code(d, e0, EVENT_CHAINING);
    // e0:SetRange(LOCATION_MZONE)
    effect::set_range(d, e0, LOCATION_MZONE);
    // e0:SetOperation(aux.chainreg)
    effect::set_operation(d, e0, Some(aux::chainreg()));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.acop)
    effect::set_operation(d, e1, Some(operation(acop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.acop(e,tp,eg,ep,ev,re,r,rp)`
fn acop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and e:GetHandler():GetFlagEffect(1)>0 then
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) {
        let h = effect::get_handler(d, a.e).nil();
        if card::get_flag_effect(d, h, 1) > 0 {
            // e:GetHandler():AddCounter(COUNTER_SPELL,2)
            let h = effect::get_handler(d, a.e).nil();
            card::add_counter(d, h, COUNTER_SPELL as u16, 2, false);
        }
    }
}
/// `s.filter(c,cc,e,tp)`
fn filter(d: &mut Duel, c: CardId, cc: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and c:HasLevel() and cc:IsCanRemoveCounter(tp,COUNTER_SPELL,c:GetLevel(),REASON_COST)
    card::is_race(d, c, RACE_SPELLCASTER as u64)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::has_level(d, c)
        && {
            let lv = card::get_level(d, c) as u16;
            card::is_can_remove_counter(d, cc, tp, COUNTER_SPELL as u16, lv, REASON_COST)
        }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let chk = a.int(8);
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e:GetHandler(),e,tp) end
    if chk == 0 {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| filter(d, c, h, e, tp)),
                tp,
                LOCATION_HAND | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            );
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND|LOCATION_GRAVE,0,nil,e:GetHandler(),e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, h, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
    // local lvt={}
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    //     local tlv=tc:GetLevel()
    //     lvt[tlv]=tlv
    // end
    // Levels of the candidates, sorted and without duplicates: the same list the Lua
    // compaction loop (`for i=1,12`) builds for `table.unpack(lvt)`.
    let mut lvt: Vec<u64> = Vec::new();
    for tc in group::iter(d, &g) {
        let tlv = card::get_level(d, tc) as u64;
        lvt.push(tlv);
    }
    lvt.sort_unstable();
    lvt.dedup();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local lv=Duel.AnnounceNumber(tp,table.unpack(lvt))
    let lv = duel::announce_number(d, tp, &lvt).map_or(0, |(n, _)| n);
    // e:GetHandler():RemoveCounter(tp,COUNTER_SPELL,lv,REASON_COST)
    let h = effect::get_handler(d, e).nil();
    card::remove_counter(d, h, tp, COUNTER_SPELL as u16, lv as u16, REASON_COST);
    // e:SetLabel(lv)
    effect::set_label(d, e, &[lv as i64]);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.sfilter(c,lv,e,tp)`
fn sfilter(d: &mut Duel, c: CardId, lv: u64, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:GetLevel()==lv
    card::is_race(d, c, RACE_SPELLCASTER as u64)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::get_level(d, c) as u64 == lv
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let tp = a.p(1);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local lv=e:GetLabel()
    let lv = effect::get_label(d, e) as u64;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.sfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,lv,e,tp)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| sfilter(d, c, lv, e, tp))),
        tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
