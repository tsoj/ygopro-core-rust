//! Myutant Fusion
//! Scripted by edo9300
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 42577802;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MYUTANT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_MYUTANT),
    //     matfilter=Card.IsAbleToRemove,extrafil=s.extrafil,extraop=Fusion.BanishMaterial,extratg=s.extrtarget})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_MYUTANT as u16]))),
            matfilter: Some(fusion::mat_filter_fn(|d, c, _e, tp, chk| card::is_able_to_remove(d, c, None, Some(tp), Some(chk as u32)))),
            extrafil: Some(Arc::new(extrafil)),
            extraop: Some(fusion::banish_material()),
            extratg: Some(cost_target(extrtarget)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,aux.FALSE)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, aux_false());
}
/// `s.check(g1,g2)`: the returned function `function(tp,sg,fc)`
fn check(d: &mut Duel, g1: &CardSet, g2: &CardSet, sg: &CardSet) -> (bool, bool) {
    // local c1=#(sg&g1)
    // local c2=#(sg&g2)
    let c1 = group::iter(d, sg).into_iter().filter(|&x| group::is_contains(g1, x)).count();
    let c2 = group::iter(d, sg).into_iter().filter(|&x| group::is_contains(g2, x)).count();
    // return c1<=1 and c2<=1,c1>1 or c2>1
    (c1 <= 1 && c2 <= 1, c1 > 1 || c2 > 1)
}
/// `s.extrafil(e,tp,mg)`
fn extrafil(d: &mut Duel, e: EffectId, tp: u8, mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if Duel.GetCustomActivityCount(id,1-e:GetHandlerPlayer(),ACTIVITY_CHAIN)==0 then return nil end
    let hp = effect::get_handler_player(d, e);
    if duel::get_custom_activity_count(d, ID, 1 - hp, ACTIVITY_CHAIN) == 0 {
        return (None, None);
    }
    // local loc=LOCATION_DECK
    let mut loc = LOCATION_DECK;
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then loc=loc|LOCATION_GRAVE end
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() {
        loc |= LOCATION_GRAVE;
    }
    // local g=Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,loc,0,mg)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, loc, 0, mg.into());
    // return g,s.check(g:Split(Card.IsLocation,nil,LOCATION_GRAVE))
    let (g1, g2) = group::split(d, &g, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    (Some(g), Some(lua_fn(move |d: &mut Duel, a: Args| {
        let sg = a.g(1);
        let (r1, r2) = check(d, &g1, &g2, &sg);
        Value::Multi(vec![Value::Bool(r1), Value::Bool(r2)])
    })))
}
/// `s.extrtarget(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extrtarget(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, a.tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,0,tp,LOCATION_GRAVE|LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 0, a.tp, (LOCATION_GRAVE | LOCATION_DECK) as i32);
    true
}
