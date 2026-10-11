//! Wattuna
//! エレキハダマグロ
use super::prelude::*;

pub const ID: u32 = 10113611;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WATT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can attack directly
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon itself from the hand if you inflict battle damage to the opponent
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.HasFlagEffect(tp,id) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::has_flag_effect(d, a.tp, ID, None))));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Watt" Synchro Monster from your Extra Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EVENT_BATTLE_DAMAGE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.spsynccon)
    effect::set_condition(d, e3, Some(condition(spsynccon)));
    // e3:SetTarget(s.spsynctg)
    effect::set_target(d, e3, Some(cost_target(spsynctg)));
    // e3:SetOperation(s.spsyncop)
    effect::set_operation(d, e3, Some(operation(spsyncop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Check that a monster inflicted battle damage to the opponent
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_BATTLE_DAMAGE)
        effect::set_code(d, ge1, EVENT_BATTLE_DAMAGE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local player=eg:GetFirst():GetControler()
    let eg = a.eg.clone().nil();
    let player = card::get_controler(d, group::get_first(d, &eg).nil());
    // if ep==1-player then
    if a.ep == 1 - player {
        // Duel.RegisterFlagEffect(player,id,RESET_PHASE|PHASE_DAMAGE,0,1)
        duel::register_flag_effect(d, player, ID, RESET_PHASE | PHASE_DAMAGE, 0, 1, 0);
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.spsynccon(e,tp,eg,ep,ev,re,r,rp)`
fn spsynccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and Duel.GetAttackTarget()==nil
    a.ep == 1 - a.tp && duel::get_attack_target(d).is_none()
}
/// `s.relfilter(c)`
fn relfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasLevel() and not c:IsType(TYPE_TUNER) and c:IsReleasableByEffect()
    //     and (c:IsLocation(LOCATION_HAND) or c:IsFaceup())
    card::has_level(d, c)
        && !card::is_type(d, c, TYPE_TUNER)
        && card::is_releasable_by_effect(d, c, None, None)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
}
/// `s.spfilter(c,e,tp,matg,lv)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, matg: Option<&CardSet>, lv: i64) -> bool {
    // return c:IsSetCard(SET_WATT) and c:IsType(TYPE_SYNCHRO)
    //     and c:IsLevel(lv) and Duel.GetLocationCountFromEx(tp,tp,matg,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_WATT as u16])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_level(d, c, &[lv as u32])
        && duel::get_location_count_from_ex(d, tp, Some(tp), matg.map_or(NO_EX, |g| g.into()), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    // return sg:IsContains(e:GetHandler())
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,sg,sg:GetSum(Card.GetLevel))
    let h = effect::get_handler(d, e).nil();
    let contains = group::is_contains(sg, h);
    if !contains {
        return (false, false);
    }
    let lv = group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64);
    let ok = duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, Some(sg), lv)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    (ok, false)
}
/// `s.spsynctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spsynctg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(s.relfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil)
    let mut g = duel::get_matching_group(d, Some(&relfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // g:Merge(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // if chk==0 then return c:HasLevel() and #g>=2
    //     and aux.SelectUnselectGroup(g,e,tp,2,#g,s.rescon,0) end
    if a.chk == Some(0) {
        return card::has_level(d, c)
            && g.len() >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(g.len()), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_RELEASE,c,1,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, Some(CardSet::from_card(c)), 1, a.tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spsyncop(e,tp,eg,ep,ev,re,r,rp)`
fn spsyncop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.relfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil)
    let mut g = duel::get_matching_group(d, Some(&relfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // g:Merge(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,2,#g,s.rescon,1,tp,HINTMSG_RELEASE)
    let rg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(g.len()), Some(&rescon), 1, a.tp, Some(HINTMSG_RELEASE as u64), None, None, false)
        .unwrap_or_default();
    // if #rg<2 then return end
    if rg.len() < 2 {
        return;
    }
    // local lv=rg:GetSum(Card.GetLevel)
    let lv = group::get_sum(d, &rg, &|d, c| card::get_level(d, c) as i64);
    // if Duel.Release(rg,REASON_EFFECT)>0 then
    if duel::release(d, rg, REASON_EFFECT, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,nil,lv):GetFirst()
        let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, None, lv)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if sc then
        if let Some(sc) = group::get_first(d, &sel) {
            // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
