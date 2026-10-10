//! Epigonen, the Impersonation Invader
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 51611041;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[51611042],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon self and Token
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
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
/// `s.spcostfilter(c,tp)`
fn spcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_EFFECT) and Duel.GetMZoneCount(tp,c)>1
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,c:GetOriginalRace(),c:GetOriginalAttribute())
    card::is_type(d, c, TYPE_EFFECT)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 1
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(Some(card::get_original_race(d, c)), Some(card::get_original_attribute(d, c))), None, None, 0)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.spcostfilter,1,false,nil,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::check_release_group_cost(d, tp, Some(&|d, c| spcostfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // local cc=Duel.SelectReleaseGroupCost(tp,s.spcostfilter,1,1,false,nil,nil,tp):GetFirst()
    let tp = a.tp;
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| spcostfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    let cc = group::get_first(d, &g).nil();
    // e:SetLabel(cc:GetOriginalRace(),cc:GetOriginalAttribute())
    let race = card::get_original_race(d, cc) as i64;
    let attribute = card::get_original_attribute(d, cc) as i64;
    effect::set_label(d, a.e, &[race, attribute]);
    // Duel.Release(cc,REASON_COST)
    duel::release(d, CardSet::from_card(cc), REASON_COST, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,c:GetLocation())
    let loc = card::get_location(d, c) as i32;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, loc);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    true
}
/// The token of `s.spop`: `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,race,attribute)`
fn token_data(race: Option<u64>, attribute: Option<u32>) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race,
        attribute,
    }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local race,attribute=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let race = labels.first().map(|&r| r as u64);
    let attribute = labels.get(1).map(|&x| x as u32);
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,1,tp,tp,false,false,POS_FACEUP)>0
    //     and ft>0 and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,race,attribute) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, CardSet::from_card(c), 1, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && ft > 0
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(race, attribute), None, None, 0)
    {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // if Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Change Type and Attribute
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_RACE)
            effect::set_code(d, e1, EFFECT_CHANGE_RACE);
            // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
            effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetValue(race)
            effect::set_value(d, e1, race.map_or(0, |r| r as i64));
            // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
            // token:RegisterEffect(e1,true)
            card::register_effect(d, token, e1, true);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_CHANGE_ATTRIBUTE)
            effect::set_code(d, e2, EFFECT_CHANGE_ATTRIBUTE);
            // e2:SetValue(attribute)
            effect::set_value(d, e2, attribute.map_or(0, |x| x as i64));
            // token:RegisterEffect(e2,true)
            card::register_effect(d, token, e2, true);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
