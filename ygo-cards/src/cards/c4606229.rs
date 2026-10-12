//! Numbers Last Hope
use super::prelude::*;

pub const ID: u32 = 4606229;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GALAXY_EYES, SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
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
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,math.floor(Duel.GetLP(tp)/2))
    let half = duel::get_lp(d, a.tp).div_euclid(2);
    duel::pay_lp_cost(d, a.tp, half as u32);
    true
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_effect_target(d, c, Some(e)) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.xyzfilter(c,mg)`
fn xyzfilter(d: &mut Duel, c: CardId, mg: &CardSet) -> bool {
    // return c:IsSetCard(SET_NUMBER) and c:IsXyzSummonable(nil,mg,2,2)
    card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_xyz_summonable(d, c, None, Some(mg.clone()), 2, 2)
}
/// `s.mfilter1(c,mg,exg)`
fn mfilter1(d: &mut Duel, c: CardId, mg: &CardSet, exg: &CardSet) -> bool {
    // return mg:IsExists(s.mfilter2,1,c,c,exg)
    group::is_exists(d, mg, &|d, x| mfilter2(d, x, Some(c), exg), 1, c.into())
}
/// `s.mfilter2(c,mc,exg)` (`mc` nil: `Group.FromCards` skips it)
fn mfilter2(d: &mut Duel, c: CardId, mc: Option<CardId>, exg: &CardSet) -> bool {
    // return exg:IsExists(Card.IsXyzSummonable,1,nil,nil,Group.FromCards(c,mc))
    let mut both = CardSet::from_card(c);
    if let Some(mc) = mc {
        both.insert(mc);
    }
    group::is_exists(d, exg, &|d, x| card::is_xyz_summonable(d, x, None, Some(both.clone()), 0, 0), 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let e = a.e;
    let tp = a.tp;
    // local mg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let mg = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // local exg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil,mg)
    let exg = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, &mg)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if chk==0 then return Duel.IsPlayerCanSpecialSummonCount(tp,2)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and #exg>0 end
    if a.chk == Some(0) {
        return duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && !exg.is_empty();
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg1=mg:FilterSelect(tp,s.mfilter1,1,1,nil,mg,exg)
    let sg1 = group::filter_select(d, &mg, tp, &|d, c| mfilter1(d, c, &mg, &exg), 1, 1, false, NO_EX).nil();
    // local tc1=sg1:GetFirst()
    // (nil when nothing could be selected, e.g. an activation the belief replay forced: Lua
    // carries on with it)
    let tc1 = group::get_first(d, &sg1);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg2=mg:FilterSelect(tp,s.mfilter2,1,1,tc1,tc1,exg)
    let ex = match tc1 {
        Some(tc1) => tc1.into(),
        None => NO_EX,
    };
    let sg2 = group::filter_select(d, &mg, tp, &|d, c| mfilter2(d, c, tc1, &exg), 1, 1, false, ex).nil();
    // sg1:Merge(sg2)
    let mut sg1 = sg1;
    group::add_card(&mut sg1, &sg2);
    // Duel.SetTargetCard(sg1)
    duel::set_target_card(d, sg1.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg1,3,tp,LOCATION_GRAVE|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg1), 3, tp, (LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    true
}
/// `s.filter2(c,e,tp)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRelateToEffect(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_relate_to_effect(d, c, e) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS):Filter(s.filter2,nil,e,tp)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let targets = info[0].as_group().cloned().nil();
    let g = group::filter(d, &targets, &|d, c| filter2(d, c, e, tp), NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // Negate their effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
    // if Duel.SpecialSummonComplete()~=2 then return end
    if duel::special_summon_complete(d) != 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local xyz=Duel.SelectMatchingCard(tp,s.xyzfilter,tp,LOCATION_EXTRA,0,1,1,nil,g):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| xyzfilter(d, x, &g)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let xyz = group::get_first(d, &sel);
    // if xyz then
    if let Some(xyz) = xyz {
        // Duel.XyzSummon(tp,xyz,g,nil,2,2)
        duel::xyz_summon(d, tp, xyz, Some(g.clone()), None, 2, 2);
        // xyz:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~RESET_TOFIELD,0,1)
        card::register_flag_effect(d, xyz, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 0, 1, 0, 0);
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // Check for a Special Summon from the Extra Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // e3:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsSummonLocation(LOCATION_EXTRA) and c:GetFlagEffect(id)==0
    card::is_summon_player(d, c, tp) && card::is_summon_location(d, c, LOCATION_EXTRA) && card::get_flag_effect(d, c, ID) == 0
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot Special Summon from the Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_,c) return c:IsLocation(LOCATION_EXTRA) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Clock Lizard check
    // aux.addTempLizardCheck(c,tp)
    aux::add_temp_lizard_check(d, c, tp, None, None, None, None, None);
}
