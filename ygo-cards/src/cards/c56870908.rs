//! Yum☆Yum☆Yummys
//! Ｙｕｍ☆Ｙｕｍ☆ヤミーズ
use super::prelude::*;

pub const ID: u32 = 56870908;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_YUMMY],
    listed_names: &[30581601],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Each time 2 or more "Yummy" monsters are Special Summoned ... gain 500 LP ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(s.lpcon)
    effect::set_condition(d, e1, Some(condition(lpcon)));
    // e1:SetOperation(s.lpop)
    effect::set_operation(d, e1, Some(operation(lpop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send 2 face-up monsters, including "Yummy★Snatchy", ... Synchro Monster ...
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.PayLP(500))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    // e2:SetTarget(s.syncsptg)
    effect::set_target(d, e2, Some(cost_target(syncsptg)));
    // e2:SetOperation(s.syncspop)
    effect::set_operation(d, e2, Some(operation(syncspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Immediately after this effect resolves, Link Summon 1 "Yummy" Link Monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end)
    effect::set_condition(d, e3, Some(condition(|_d, a| a.rp == 1 - a.tp)));
    // e3:SetCost(Cost.PayLP(100))
    effect::set_cost(d, e3, Some(cost::pay_lp(cost::PayLp::Amount(100))));
    // e3:SetTarget(s.linksptg)
    effect::set_target(d, e3, Some(cost_target(linksptg)));
    // e3:SetOperation(s.linkspop)
    effect::set_operation(d, e3, Some(operation(linkspop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.lpconfilter(c,tp)`
fn lpconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_YUMMY) and c:IsFaceup() and c:IsControler(tp)
    card::is_set_card(d, c, &[SET_YUMMY as u16]) && card::is_faceup(d, c) && card::is_controler(d, c, tp)
}
/// `s.lpcon(e,tp,eg,ep,ev,re,r,rp)`
fn lpcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.lpconfilter,2,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| lpconfilter(d, c, a.tp), 2, NO_EX)
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // if Duel.Recover(tp,500,REASON_EFFECT)>0 then Duel.PayLPCost(1-tp,500) end
        if duel::recover(d, a.tp, 500, REASON_EFFECT, false, None) > 0 {
            duel::pay_lp_cost(d, 1 - a.tp, 500);
        }
    } else {
        let c = c.nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
        // e1:SetRange(LOCATION_SZONE)
        effect::set_range(d, e1, LOCATION_SZONE);
        // e1:SetOperation(function(e,tp) ... end)
        effect::set_operation(
            d,
            e1,
            Some(operation(|d, a| {
                // Duel.Hint(HINT_CARD,0,id)
                duel::hint(d, HINT_CARD as u8, 0, ID as u64);
                // if Duel.Recover(tp,500,REASON_EFFECT)>0 then Duel.PayLPCost(1-tp,500) end
                if duel::recover(d, a.tp, 500, REASON_EFFECT, false, None) > 0 {
                    duel::pay_lp_cost(d, 1 - a.tp, 500);
                }
            })),
        );
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CHAIN)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
        // e2:SetOperation(function() e1:Reset() end)
        effect::set_operation(d, e2, Some(operation(move |d, _a| effect::reset(d, e1))));
        // e2:SetReset(RESET_CHAIN)
        effect::set_reset(d, e2, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
}
/// `s.tgfilter(c,e)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsCanBeEffectTarget(e) and c:IsAbleToGrave()
    card::is_faceup(d, c) && card::is_can_be_effect_target(d, c, Some(e)) && card::is_able_to_grave(d, c, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsCode,1,nil,30581601) and Duel.IsExistingMatchingCard(s.syncfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,sg)
    let res = group::is_exists(d, sg, &|d, c| card::is_code(d, c, &[30581601]), 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| syncfilter(d, c, e.nil(), tp, Some(sg))), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    (res, false)
}
/// `s.syncfilter(c,e,tp,sg)`
fn syncfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: Option<&CardSet>) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,c,nil,REASON_SYNCHRO)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, Some(c), None, REASON_SYNCHRO);
    // return #pg<=0 and c:IsSetCard(SET_YUMMY) and c:IsSynchroMonster() and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    //   and Duel.GetLocationCountFromEx(tp,tp,sg,c)>0
    let ex = match sg {
        Some(g) => g.into(),
        None => NO_EX,
    };
    pg.is_empty()
        && card::is_set_card(d, c, &[SET_YUMMY as u16])
        && card::is_synchro_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO as u32, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.syncsptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn syncsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, a.e)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TOGRAVE)
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(tg), 2, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.syncspop(e,tp,eg,ep,ev,re,r,rp)`
fn syncspop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==2 and Duel.SendtoGrave(tg,REASON_EFFECT)==2 and tg:IsExists(Card.IsLocation,2,nil,LOCATION_GRAVE) then
    if tg.len() == 2
        && duel::send_to_grave(d, tg.clone(), REASON_EFFECT, None, None) == 2
        && group::is_exists(d, &tg, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 2, NO_EX)
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.syncfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
        let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| syncfilter(d, c, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        let Some(sc) = group::get_first(d, &sel) else { return };
        // sc:SetMaterial(nil)
        card::set_material(d, sc, CardSet::new());
        // if Duel.SpecialSummon(sc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)>0 then
        if duel::special_summon(d, sc, SUMMON_TYPE_SYNCHRO as u32, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
        }
    }
}
/// `s.linkfilter(c)`
fn linkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YUMMY) and c:IsLinkSummonable()
    card::is_set_card(d, c, &[SET_YUMMY as u16]) && card::is_link_summonable(d, c, None, None, 0, 0)
}
/// `s.linksptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn linksptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.linkfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&linkfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.linkspop(e,tp,eg,ep,ev,re,r,rp)`
fn linkspop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.linkfilter,tp,LOCATION_EXTRA,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&linkfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if sc then Duel.LinkSummon(tp,sc) end
    if let Some(sc) = group::get_first(d, &sel) {
        duel::link_summon(d, a.tp, sc, None, None, 0, 0);
    }
}
