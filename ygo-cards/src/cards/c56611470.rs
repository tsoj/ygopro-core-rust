//! Artifacts Unleashed
use super::prelude::*;

pub const ID: u32 = 56611470;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARTIFACT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon an Xyz Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // draw
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(s.drcon)
    effect::set_condition(d, e2, Some(condition(drcon)));
    // e2:SetCost(s.drcost)
    effect::set_cost(d, e2, Some(cost_target(drcost)));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ARTIFACT) and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ARTIFACT as u16]) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.xyzfilter(c,mg,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, mg: &CardSet) -> bool {
    // return c:IsXyzSummonable(nil,mg,2,2)
    card::is_xyz_summonable(d, c, None, Some(mg.clone()), 2, 2)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local mg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil,e)
    let mg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e)), tp, LOCATION_MZONE, 0, NO_EX);
    // local exg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil,mg)
    let exg = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, &mg)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if chk==0 then return #exg>0 end
    if a.chk == Some(0) {
        return !exg.is_empty();
    }
    // local sg=aux.SelectUnselectGroup(mg,e,tp,2,2,s.rescon(exg),1,tp,HINTMSG_XMATERIAL)
    // function s.rescon(exg) return function(sg) return #sg==2 and exg:IsExists(Card.IsXyzSummonable,1,nil,nil,sg,2,2) end end
    let rescon = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| -> (bool, bool) {
        (
            sg.len() == 2 && group::is_exists(d, &exg, &|d, c| card::is_xyz_summonable(d, c, None, Some(sg.clone()), 2, 2), 1, NO_EX),
            false,
        )
    };
    let sg = aux::select_unselect_group(d, &mg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_XMATERIAL as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        // e1:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        // e1:SetTarget(s.attg)
        effect::set_target(d, e1, Some(effect_card_fn(attg)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
    // local g=Duel.GetTargetCards(e):Match(Card.IsFaceup,nil)
    let mut g = duel::get_target_cards(d, e).nil();
    group::match_(d, &mut g, &|d, c| card::is_faceup(d, c), NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local xyzg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil,g,tp)
    let xyzg = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, &g)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #xyzg>0 then
    if !xyzg.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local xyz=xyzg:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &xyzg, tp, 1, 1, false, NO_EX).nil();
        let xyz = group::get_first(d, &sel).nil();
        // Duel.XyzSummon(tp,xyz,g,nil,2,2)
        duel::xyz_summon(d, tp, xyz, Some(g.clone()), None, 2, 2);
    }
}
/// `s.attg(e,c)`
fn attg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_ARTIFACT)
    !card::is_set_card(d, c, &[SET_ARTIFACT as u16])
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and e:GetHandler():IsPreviousControler(tp)
    let h = effect::get_handler(d, a.e).nil();
    a.rp == 1 - a.tp && card::is_previous_controler(d, h, a.tp)
}
/// `s.cffilter(c)`
fn cffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsLevel(5) and not c:IsPublic()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_level(d, c, &[5]) && !card::is_public(d, c)
}
/// `s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cffilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cffilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cffilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&cffilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    true
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
