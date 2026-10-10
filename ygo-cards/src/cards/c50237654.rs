//! The Dark Magicians
use super::prelude::*;

pub const ID: u32 = 50237654;

pub const INFO: ScriptInfo = ScriptInfo {
    material: &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL],
    material_setcode: &[SET_DARK_MAGICIAN, SET_MAGICIAN_GIRL, SET_DARK_MAGICIAN_GIRL],
    listed_names: &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,{CARD_DARK_MAGICIAN,CARD_DARK_MAGICIAN_GIRL},aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Any(vec![fusion::Mat::Code(CARD_DARK_MAGICIAN), fusion::Mat::Code(CARD_DARK_MAGICIAN_GIRL)]),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_SPELLCASTER, Some(fc), st, Some(tp)))),
        ],
    );
    // Draw 1 card and Set it if it is a Spell/Trap card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_SET);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.drcon)
    effect::set_condition(d, e1, Some(condition(drcon)));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Dark Magician" and 1 "Dark Magician Girl"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsSpellTrapEffect()
    effect::is_spell_trap_effect(d, a.re.nil())
}
/// s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let count = info[1].int() as u32;
    // if Duel.Draw(p,d,REASON_EFFECT)~=0 then
    if duel::draw(d, p, count, REASON_EFFECT) != 0 {
        // local tc=Duel.GetOperatedGroup():GetFirst()
        let og = duel::get_operated_group(d);
        let tc = group::get_first(d, &og);
        // if tc:IsSpellTrap() and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if card::is_spell_trap(d, tc.nil()) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
            let tc = tc.nil();
            // Duel.SSet(tp,tc,tp,false)
            duel::s_set(d, a.tp, CardSet::from_card(tc), Some(a.tp), false);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // if tc:IsQuickPlaySpell() then
            if card::is_quick_play_spell(d, tc) {
                // e1:SetCode(EFFECT_QP_ACT_IN_SET_TURN)
                effect::set_code(d, e1, EFFECT_QP_ACT_IN_SET_TURN);
            // elseif tc:IsTrap() then
            } else if card::is_trap(d, tc) {
                // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
                effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
            }
            // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
            effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetDescription(aux.Stringid(id,3))
            effect::set_description(d, e1, aux::stringid(ID, 3));
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// s.spfilter(c,e,tp,code)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>1
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp,CARD_DARK_MAGICIAN)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp,CARD_DARK_MAGICIAN_GIRL) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, CARD_DARK_MAGICIAN)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, CARD_DARK_MAGICIAN_GIRL)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// s.rescon(sg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsCode,1,nil,CARD_DARK_MAGICIAN) and sg:IsExists(Card.IsCode,1,nil,CARD_DARK_MAGICIAN_GIRL)
    let res = group::is_exists(d, sg, &|d, x| card::is_code(d, x, &[CARD_DARK_MAGICIAN]), 1, NO_EX)
        && group::is_exists(d, sg, &|d, x| card::is_code(d, x, &[CARD_DARK_MAGICIAN_GIRL]), 1, NO_EX);
    (res, false)
}
/// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     or Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g1=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp,CARD_DARK_MAGICIAN)
    let g1 = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, a.tp, CARD_DARK_MAGICIAN))),
        a.tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        NO_EX,
    );
    // local g2=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp,CARD_DARK_MAGICIAN_GIRL)
    let g2 = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, a.tp, CARD_DARK_MAGICIAN_GIRL))),
        a.tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        NO_EX,
    );
    // if #g1>0 and #g2>0 then
    if !g1.is_empty() && !g2.is_empty() {
        // local sg=aux.SelectUnselectGroup(g1+g2,e,tp,2,2,s.rescon,1,tp,HINTMSG_SPSUMMON)
        let mut merged = g1.clone();
        group::add_card(&mut merged, &g2);
        let sg = aux::select_unselect_group(d, &merged, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,true,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
        }
    }
}
