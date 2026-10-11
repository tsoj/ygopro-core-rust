//! Exosister Irene
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 79858629;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[5352328], listed_series: &[SET_EXOSISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.drcost)
    effect::set_cost(d, e1, Some(cost_target(drcost)));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon Xyz
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_LEAVE_GRAVE)
    effect::set_code(d, e2, EVENT_LEAVE_GRAVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(_,tp,_,_,_,_,_,rp)return rp==1-tp end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(_,tp,_,_,_,_,_,rp) return rp==1-tp end` (of e2)
fn e2_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp
}
/// `s.drcostfilter(c)`
fn drcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EXOSISTER) and c:IsAbleToDeckAsCost()
    card::is_set_card(d, c, &[SET_EXOSISTER as u16]) && card::is_able_to_deck_as_cost(d, c)
}
/// `s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.drcostfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&drcostfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.drcostfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&drcostfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, g.clone());
    // Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_COST)
    duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_COST, None);
    true
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
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
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dr = info[1].int() as u32;
    // if Duel.Draw(p,d,REASON_EFFECT)==d
    if duel::draw(d, p, dr, REASON_EFFECT) == dr as i32
        // and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,5352328),tp,LOCATION_ONFIELD,0,1,nil) then
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[5352328])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Recover(tp,800,REASON_EFFECT)
        duel::recover(d, a.tp, 800, REASON_EFFECT, false, None);
    }
}
/// `s.hspfilter`-like filter of the Xyz summon (`s.spfilter(c,e,tp,mc)`)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return c:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp) and c:IsSetCard(SET_EXOSISTER) and mc:IsCanBeXyzMaterial(c,tp)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0 and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_type_ctx(d, c, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_set_card(d, c, &[SET_EXOSISTER as u16])
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
        let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::from_card(c)), a.tp, None, None, REASON_XYZ);
        // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
        return (pg.is_empty() || (pg.len() == 1 && pg.contains(c)))
            && duel::is_existing_matching_card(d, Some(&|d, sc| spfilter(d, sc, a.e, a.tp, c)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) or c:IsControler(1-tp) or c:IsImmuneToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) || card::is_controler(d, c, 1 - a.tp) || card::is_immune_to_effect(d, c, a.e) {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::from_card(c)), a.tp, None, None, REASON_XYZ);
    // if #pg>1 or (#pg==1 and not pg:IsContains(c)) then return end
    if pg.len() > 1 || (pg.len() == 1 && !pg.contains(c)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,c):GetFirst()
    let sg = duel::select_matching_card(d, a.tp, Some(&|d, sc| spfilter(d, sc, a.e, a.tp, c)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sg);
    // if sc then
    if let Some(sc) = sc {
        // local mg=Group.FromCards(c)
        let mg = CardSet::from_card(c);
        // sc:SetMaterial(mg)
        card::set_material(d, sc, mg.clone());
        // Duel.Overlay(sc,mg)
        duel::overlay(d, sc, mg, false);
        // if Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
        if duel::special_summon(d, sc, SUMMON_TYPE_XYZ, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
        }
    }
}
