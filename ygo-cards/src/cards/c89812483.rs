//! Max Metalmorph
use super::prelude::*;

pub const ID: u32 = 89812483;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.self_equip_trap=true
    self_equip_trap: true,
    // s.listed_names={CARD_MAX_METALMORPH}
    listed_names: &[CARD_MAX_METALMORPH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 monster that cannot be Normal Summoned/Set and mentions "Max Metalmorph" from your hand/Deck/GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMING_BATTLE_START|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c,e,tp)`
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp,c:GetLevel(),c:GetRace())
    if !card::is_faceup(d, c) || duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 <= 0 {
        return false;
    }
    let cost_lv = card::get_level(d, c) as i64;
    let cost_race = card::get_race(d, c);
    duel::is_existing_matching_card(
        d,
        Some(&|d, x| spfilter(d, x, e, tp, Some(cost_lv), Some(cost_race))),
        tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        NO_EX,
    )
}
/// `s.spfilter(c,e,tp,cost_lv,cost_race)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, cost_lv: Option<i64>, cost_race: Option<u64>) -> bool {
    // if not (c:IsMonster() and not c:IsSummonableCard() and c:ListsCode(CARD_MAX_METALMORPH)) then return false end
    if !(card::is_monster(d, c) && !card::is_summonable_card(d, c) && card::lists_code(d, c, &[CARD_MAX_METALMORPH])) {
        return false;
    }
    // if c:IsCanBeSpecialSummoned(e,0,tp,false,false) or c.max_metalmorph_stats==nil then return true end
    let stats = crate::card_info(d, c).max_metalmorph_stats;
    if card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) || stats.is_none() {
        return true;
    }
    // if not (c:IsCanBeSpecialSummoned(e,0,tp,true,true) and cost_lv and cost_race) then return false end
    if !(card::is_can_be_special_summoned(d, c, e, 0, tp, true, true, None, None, None) && cost_lv.is_some() && cost_race.is_some()) {
        return false;
    }
    // local lv,race=table.unpack(c.max_metalmorph_stats)
    let (lv, race) = stats.unwrap();
    // return cost_lv>=lv and cost_race&race>0
    cost_lv.unwrap() >= lv as i64 && cost_race.unwrap() & race > 0
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil,e,tp)
        let res = duel::check_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
        // if res then e:SetLabel(1) end
        if res {
            effect::set_label(d, e, &[1]);
        }
        // return res
        return res;
    }
    // local rc=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil,e,tp):GetFirst()
    let sel = duel::select_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    let rc = group::get_first(d, &sel).nil();
    // e:SetLabel(rc:GetLevel(),rc:GetRace())
    let lv = card::get_level(d, rc) as i64;
    let race = card::get_race(d, rc) as i64;
    effect::set_label(d, e, &[lv, race]);
    // Duel.Release(rc,REASON_COST)
    duel::release(d, rc, REASON_COST, None);
    true
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=e:GetLabel()==1 or (Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp))
        let res = effect::get_label(d, e) == 1
            || (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && duel::is_existing_matching_card(
                    d,
                    Some(&|d, x| spfilter(d, x, e, tp, None, None)),
                    tp,
                    LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
                    0,
                    1,
                    NO_EX,
                ));
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return res
        return res;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, Some(h.into()), 1, tp, 0);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local cost_lv,cost_race=e:GetLabel()
    let labels = effect::get_labels(d, e);
    let cost_lv = labels.first().copied();
    let cost_race = labels.get(1).map(|&r| r as u64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp,cost_lv,cost_race):GetFirst()
    let sel = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp, cost_lv, cost_race))),
        tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,true,true,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, tp, tp, true, true, POS_FACEUP as u8, None) > 0 {
            // tc:CompleteProcedure()
            card::complete_procedure(d, tc);
            // if not (c:IsRelateToEffect(e) and Duel.SelectYesNo(tp,aux.Stringid(id,1))) then return end
            if !(card::is_relate_to_effect(d, c, e) && duel::select_yes_no(d, tp, aux::stringid(ID, 1))) {
                return;
            }
            // c:CancelToGrave(true)
            card::cancel_to_grave(d, c, true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if not tc:EquipByEffectAndLimitRegister(e,tp,c,nil,true) then return end
            if !card::equip_by_effect_and_limit_register(d, tc, e, tp, c, None, Some(true)) {
                return;
            }
            // Equip limit
            // local e0=Effect.CreateEffect(c)
            let e0 = effect::create_effect(d, c);
            // e0:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
            // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e0:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
            // e0:SetValue(function(e,c) return c==tc end)
            effect::set_value_fn(d, e0, lua_fn(move |_d, x: Args| x.card(1) == Some(tc)));
            // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e0)
            card::register_effect(d, c, e0, false);
            // The equipped monster gains 400 ATK/DEF
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_EQUIP)
            effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(400)
            effect::set_value(d, e1, 400);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_UPDATE_DEFENSE)
            effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
            // The equipped monster cannot be destroyed by monster and Spell effects
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_EQUIP)
            effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
            // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
            effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
            // e3:SetValue(function(e,re,rc,c) return re:IsMonsterEffect() or re:IsSpellEffect() end)
            effect::set_value_fn(d, e3, lua_fn(|d, x: Args| {
                let re = x.e(1);
                effect::is_monster_effect(d, re) || effect::is_spell_effect(d, re)
            }));
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e3)
            card::register_effect(d, c, e3, false);
            // Your opponent cannot target the monster with monster and Spell effects
            // local e4=e3:Clone()
            let e4 = effect::clone(d, e3);
            // e4:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e4, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e4:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
            effect::set_code(d, e4, EFFECT_CANNOT_BE_EFFECT_TARGET);
            // e4:SetValue(function(e,re,rp) return rp==1-e:GetHandlerPlayer() and (re:IsMonsterEffect() or re:IsSpellEffect()) end)
            effect::set_value_fn(d, e4, lua_fn(|d, x: Args| {
                let re = x.e(1);
                x.int(2) == 1 - effect::get_handler_player(d, x.e(0)) as i64 && (effect::is_monster_effect(d, re) || effect::is_spell_effect(d, re))
            }));
            // c:RegisterEffect(e4)
            card::register_effect(d, c, e4, false);
        }
    }
}
