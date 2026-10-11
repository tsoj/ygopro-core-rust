//! Soul of the Supreme Celestial King
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 76840111;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_ZARC],
    listed_series: &[SET_PENDULUM_DRAGON, SET_FUSION_DRAGON, SET_SYNCHRO_DRAGON, SET_XYZ_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=aux.FilterBoolFunction(Card.IsCode,CARD_ZARC),matfilter=Card.IsAbleToRemove,extrafil=s.fextra,extraop=Fusion.BanishMaterial,extratg=s.extratarget,stage2=s.stage2})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, x, _tp| card::is_code(d, x, &[CARD_ZARC]))),
            matfilter: Some(fusion::mat_filter_fn(|d, x, _e, tp, chk| card::is_able_to_remove(d, x, None, Some(tp), Some(chk as u32)))),
            extrafil: Some(Arc::new(fextra)),
            extraop: Some(fusion::banish_material()),
            extratg: Some(cost_target(extratarget)),
            stage2: Some(Arc::new(stage2)),
            ..Default::default()
        },
    );
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.fextra(e,tp,mg)
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // local loc=LOCATION_DECK|LOCATION_EXTRA
    let mut loc = LOCATION_DECK | LOCATION_EXTRA;
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        // loc=loc|LOCATION_GRAVE
        loc |= LOCATION_GRAVE;
    }
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,loc,0,nil)
    (
        Some(duel::get_matching_group(
            d,
            Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)),
            tp,
            loc,
            0,
            NO_EX,
        )),
        None,
    )
}
// s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_PENDULUM) and c:IsRace(RACE_SPELLCASTER) and c:GetBaseAttack()==2500
    card::is_type(d, c, TYPE_PENDULUM) && card::is_race(d, c, RACE_SPELLCASTER) && card::get_base_attack(d, c) == 2500
}
// s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&cfilter), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil)
    let g = duel::select_release_group_cost(d, tp, Some(&cfilter), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// s.extratarget(e,tp,eg,ep,ev,re,r,rp,chk)
fn extratarget(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE|LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(
        d,
        0,
        CATEGORY_REMOVE,
        None,
        0,
        a.tp,
        (LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE | LOCATION_DECK | LOCATION_EXTRA) as i32,
    );
    true
}
// s.chkfilter(set): function(c) return c:IsFaceup() and c:IsMonster() and c:IsSetCard(set) end
fn chkfilter(d: &mut Duel, c: CardId, set: u32) -> bool {
    card::is_faceup(d, c) && card::is_monster(d, c) && card::is_set_card(d, c, &[set as u16])
}
// s.stage2(e,tc,tp,sg,chk)
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==0 then
    if chk != 0 {
        return;
    }
    // local check=true
    let mut check = true;
    // local sets={SET_PENDULUM_DRAGON,SET_FUSION_DRAGON,SET_SYNCHRO_DRAGON,SET_XYZ_DRAGON}
    let sets = [SET_PENDULUM_DRAGON, SET_FUSION_DRAGON, SET_SYNCHRO_DRAGON, SET_XYZ_DRAGON];
    // for _,set in ipairs(sets) do
    for set in sets {
        // if not Duel.IsExistingMatchingCard(s.chkfilter(set),tp,LOCATION_REMOVED,0,1,nil) then check=false end
        if !duel::is_existing_matching_card(d, Some(&|d, c| chkfilter(d, c, set)), tp, LOCATION_REMOVED, 0, 1, NO_EX) {
            check = false;
        }
    }
    // if not check then tc:NegateEffects(e:GetHandler()) end
    if !check {
        let tc = tc.nil();
        let h = effect::get_handler(d, e).nil();
        card::negate_effects(d, tc, h, None, false, None);
    }
}
