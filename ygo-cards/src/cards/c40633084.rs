//! 騎甲虫歩兵分隊 (Beetrooper Squad)
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 40633084;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[64213018], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, TIMINGS_CHECK_MONSTER_E, None);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // if chk==0 then return true end
    true
}
/// `s.costfilter(c,tp)`
fn costfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsRace(RACE_INSECT) and not c:IsType(TYPE_TOKEN) and c:GetBaseAttack()>=1000
    //     and Duel.GetMZoneCount(tp,c)>0
    card::is_monster(d, c)
        && card::is_race(d, c, RACE_INSECT)
        && !card::is_type(d, c, TYPE_TOKEN)
        && card::get_base_attack(d, c) >= 1000
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=0 then return false end
        if effect::get_label(d, a.e) != 0 {
            return false;
        }
        // return Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil,tp)
        //     and Duel.IsPlayerCanSpecialSummonMonster(tp,64213018,SET_BEETROOPER,TYPES_TOKEN,1000,1000,3,RACE_INSECT,ATTRIBUTE_EARTH)
        return aux::check_release_group_cost(d, a.tp, Some(&|d, c| costfilter(d, c, a.tp)), 1, 1, false, None, NO_EX) && duel::is_player_can_special_summon_monster(d, a.tp, 64213018, token_data(), None, None, 0);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // local tg=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil,tp)
    let tg = aux::select_release_group_cost(d, a.tp, Some(&|d, c| costfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    // e:SetLabel(tg:GetFirst():GetBaseAttack()//1000)
    let first = group::get_first(d, &tg).nil();
    let lv = card::get_base_attack(d, first) as i64 / 1000;
    effect::set_label(d, a.e, &[lv]);
    // Duel.Release(tg,REASON_COST)
    duel::release(d, tg, REASON_COST, None);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if not Duel.IsPlayerCanSpecialSummonMonster(tp,64213018,SET_BEETROOPER,TYPES_TOKEN,1000,1000,3,RACE_INSECT,ATTRIBUTE_EARTH) then return end
    if !duel::is_player_can_special_summon_monster(d, a.tp, 64213018, token_data(), None, None, 0) {
        return;
    }
    // ft=math.min(ft,e:GetLabel())
    ft = ft.min(effect::get_label(d, a.e) as i32);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // if ft>1 then ft=Duel.AnnounceNumberRange(tp,1,ft) end
    if ft > 1 {
        ft = duel::announce_number_range(d, a.tp, Some(1), Some(ft as u32), &[]).nil().0 as i32;
    }
    // local sg=Group.CreateGroup()
    let mut sg = CardSet::new();
    // for i=1,ft do
    for _ in 0..ft.max(0) {
        // local token=Duel.CreateToken(tp,64213018)
        let token = duel::create_token(d, a.tp, 64213018).nil();
        // sg:AddCard(token)
        group::add_card(&mut sg, &CardSet::from_card(token));
    }
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
/// The Beetrooper Token's data (`Duel.IsPlayerCanSpecialSummonMonster(..., 64213018, SET_BEETROOPER, TYPES_TOKEN, 1000, 1000, 3, RACE_INSECT, ATTRIBUTE_EARTH)`)
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_BEETROOPER as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(1000),
        defense: Some(1000),
        level: Some(3),
        race: Some(RACE_INSECT),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
