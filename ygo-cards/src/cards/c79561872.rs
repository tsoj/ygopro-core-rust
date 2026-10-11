//! Melffys' Joyful Surprise
use super::prelude::*;

pub const ID: u32 = 79561872;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MELFFY], material_setcode: &[SET_MELFFY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_MELFFY),aux.FilterBoolFunctionEx(Card.IsRace,RACE_BEAST))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_MELFFY as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_BEAST as u64, Some(fc), st, Some(tp)))),
        ],
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_fusion_summoned(d, h)
    })));
    // e1:SetTarget(s.rthtg)
    effect::set_target(d, e1, Some(cost_target(rthtg)));
    // e1:SetOperation(s.rthop)
    effect::set_operation(d, e1, Some(operation(rthop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_LVCHANGE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2a, CATEGORY_LVCHANGE + CATEGORY_SPECIAL_SUMMON);
    // e2a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetCondition(function(e,tp,eg) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end)
    effect::set_condition(d, e2a, Some(condition(e2a_cond)));
    // e2a:SetTarget(s.sptg)
    effect::set_target(d, e2a, Some(cost_target(sptg)));
    // e2a:SetOperation(s.spop)
    effect::set_operation(d, e2a, Some(operation(spop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // local e2c=e2a:Clone()
    let e2c = effect::clone(d, e2a);
    // e2c:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e2c, EVENT_BE_BATTLE_TARGET);
    // e2c:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()==e:GetHandler() end)
    effect::set_condition(d, e2c, Some(condition(e2c_cond)));
    // c:RegisterEffect(e2c)
    card::register_effect(d, c, e2c, false);
}
/// `function(e,tp,eg) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end`
fn e2a_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX)
}
/// `function(e,tp) return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()==e:GetHandler() end`
fn e2c_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let attacker = duel::get_attacker(d).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_controler(d, attacker, 1 - tp) && duel::get_attack_target(d) == Some(h)
}
/// `s.rthandrescon(maxc)`: `function(sg,e,tp,mg)` returning `ct1==ct2, ct1>maxc or ct2>maxc`
fn rthandrescon(d: &mut Duel, sg: &CardSet, tp: u8, maxc: usize) -> (bool, bool) {
    let ct1 = group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX);
    let ct2 = sg.len() - ct1;
    (ct1 == ct2, ct1 > maxc || ct2 > maxc)
}
/// `s.rthfilter(c,opp)`
fn rthfilter(d: &mut Duel, c: CardId, opp: u8) -> bool {
    // return (c:IsControler(opp) or (c:IsSetCard(SET_MELFFY) and c:IsFaceup())) and c:IsAbleToHand()
    (card::is_controler(d, c, opp) || (card::is_set_card(d, c, &[SET_MELFFY as u16]) && card::is_faceup(d, c)))
        && card::is_able_to_hand(d, c, None)
}
/// `s.rthtg(e,tp,...,chk,chkc)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local opp=1-tp
    let opp = 1 - tp;
    // if chk==0 then return Duel.IsExistingTarget(s.rthfilter,tp,LOCATION_ONFIELD,0,1,nil,opp)
    //     and Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rthfilter(d, c, opp)), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetTargetGroup(s.rthfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil,opp)
    let g = duel::get_target_group(d, Some(&|d, c| rthfilter(d, c, opp)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // local ct=g:FilterCount(Card.IsControler,nil,tp)
    let ct = group::filter_count(d, &g, &|d, c| card::is_controler(d, c, tp), NO_EX);
    // local rescon=s.rthandrescon(math.min(ct,#g-ct))
    let maxc = ct.min(g.len() - ct);
    let rescon = move |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp2: u8, _mg: &CardSet, _c: Option<CardId>| rthandrescon(d, sg, tp2, maxc);
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,#g,rescon,1,tp,HINTMSG_RTOHAND,rescon)
    let tg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        tp,
        Some(2),
        Some(g.len()),
        Some(&rescon),
        1,
        tp,
        Some(HINTMSG_RTOHAND as u64),
        Some(&rescon),
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,#tg,tp,0)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), n, tp, 0);
    true
}
/// `s.rthop(e,tp,...)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_BEAST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_BEAST as u64) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,...,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return c:HasLevel() and c:IsLevelAbove(3) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return card::has_level(d, c)
            && card::is_level_above(d, c, 3)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,c,1,tp,-2)
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(CardSet::from_card(c)), 1, tp, -2);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsLevelAbove(3) and c:IsFaceup() and c:UpdateLevel(-2)==-2
    //     and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_level_above(d, c, 3)
        && card::is_faceup(d, c)
        && card::update_level(d, c, -2, None, None) == -2
        && card::is_relate_to_effect(d, tc.nil(), a.e)
    {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(tc.nil()), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
