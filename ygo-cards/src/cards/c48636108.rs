//! U.A. Man of the Match
//! Ｕ．Ａ．マン・オブ・ザ・マッチ
use super::prelude::*;

pub const ID: u32 = 48636108;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_FA, SET_UA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon when destroying by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1, EVENT_BATTLED);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon when inflicting battle damage
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    // e2:SetCondition(s.dmcon)
    effect::set_condition(d, e2, Some(condition(dmcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let attacker = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    // if not d then return false end
    let Some(dd) = duel::get_attack_target(d) else {
        return false;
    };
    // if d:IsControler(tp) then a,d=d,a end
    let (aa, dd2) = if card::is_controler(d, dd, a.tp) { (Some(dd), attacker) } else { (attacker, Some(dd)) };
    // return (a:IsSetCard(SET_FA) or a:IsSetCard(SET_UA)) and not a:IsStatus(STATUS_BATTLE_DESTROYED)
    //     and d:IsStatus(STATUS_BATTLE_DESTROYED)
    (card::is_set_card(d, aa.nil(), &[SET_FA as u16]) || card::is_set_card(d, aa.nil(), &[SET_UA as u16]))
        && !card::is_status(d, aa.nil(), STATUS_BATTLE_DESTROYED)
        && card::is_status(d, dd2.nil(), STATUS_BATTLE_DESTROYED)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) and (c:IsSetCard(SET_FA) or c:IsSetCard(SET_UA))
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && (card::is_set_card(d, c, &[SET_FA as u16]) || card::is_set_card(d, c, &[SET_UA as u16]))
}
fn sprescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetCode)>=#sg
    let res = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) >= sg.len();
    (res, false)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE|LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,LOCATION_GRAVE|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, (LOCATION_GRAVE | LOCATION_HAND) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    // local tg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE|LOCATION_HAND,0,nil,e,tp)
    let tg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_HAND, 0, NO_EX);
    // if #tg==0 or ft<=0 then return end
    if tg.is_empty() || ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local g=aux.SelectUnselectGroup(tg,e,tp,1,ft,s.sprescon,1,tp,HINTMSG_SPSUMMON)
    let g = aux::select_unselect_group(d, &tg, Some(a.e), a.tp, Some(1), Some(ft as usize), Some(&sprescon), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
fn dmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    // return ep==1-tp and tc:IsControler(tp) and (tc:IsSetCard(SET_FA) or tc:IsSetCard(SET_UA))
    a.ep == 1 - a.tp
        && card::is_controler(d, tc.nil(), a.tp)
        && (card::is_set_card(d, tc.nil(), &[SET_FA as u16]) || card::is_set_card(d, tc.nil(), &[SET_UA as u16]))
}
