//! Secreterion Dragon
use super::prelude::*;

pub const ID: u32 = 89851827;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 Dragon monster + 1 Spellcaster monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON),aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_SPELLCASTER, Some(fc), st, Some(tp)))),
        ],
    );
    // Loses 100 ATK for each of your banished cards
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e) return -100*Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_REMOVED,0) end)
    effect::set_value_fn(d, e1, lua_fn(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // While you control this Fusion Summoned card, your opponent cannot activate the effects of Special Summoned Dragon and Spellcaster monsters they control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e2, Some(lua_fn(fscon)));
    // e2:SetValue(s.actlimval)
    effect::set_value_fn(d, e2, lua_fn(actlimval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 Dragon or 1 Spellcaster monster from your GY and place the other on the bottom of the Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.sptdtg)
    effect::set_target(d, e3, Some(cost_target(sptdtg)));
    // e3:SetOperation(s.sptdop)
    effect::set_operation(d, e3, Some(operation(sptdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return -100*Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_REMOVED,0) end`
fn atkval(d: &mut Duel, a: Args) -> i64 {
    let p = effect::get_handler_player(d, a.e(0));
    -100 * duel::get_field_group_count(d, p, LOCATION_REMOVED, 0) as i64
}
/// `function(e) return e:GetHandler():IsFusionSummoned() end`
fn fscon(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.actlimval(e,re,rp)`
fn actlimval(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re);
    // return re:IsMonsterEffect() and rc:IsSpecialSummoned() and rc:IsFaceup()
    //   and rc:IsRace(RACE_DRAGON|RACE_SPELLCASTER) and rc:IsLocation(LOCATION_MZONE)
    effect::is_monster_effect(d, re) && {
        let rc = rc.nil();
        card::is_special_summoned(d, rc) && card::is_faceup(d, rc) && card::is_race(d, rc, RACE_DRAGON | RACE_SPELLCASTER) && card::is_location(d, rc, LOCATION_MZONE)
    }
}
/// `s.sptdfilter(c,e,tp)`
fn sptdfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON|RACE_SPELLCASTER) and (c:IsCanBeSpecialSummoned(e,0,tp,false,false) or c:IsAbleToDeck())
    card::is_race(d, c, RACE_DRAGON | RACE_SPELLCASTER)
        && (card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) || card::is_able_to_deck(d, c))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsRace,1,nil,RACE_DRAGON) and sg:IsExists(Card.IsRace,1,nil,RACE_SPELLCASTER)
    //   and sg:IsExists(s.spchk,1,nil,e,tp,sg)
    let dragon = group::is_exists(d, sg, &|d, c| card::is_race(d, c, RACE_DRAGON), 1, NO_EX);
    let spellcaster = dragon && group::is_exists(d, sg, &|d, c| card::is_race(d, c, RACE_SPELLCASTER), 1, NO_EX);
    let r = spellcaster && group::is_exists(d, sg, &|d, c| spchk(d, c, e.nil(), tp, sg), 1, NO_EX);
    (r, false)
}
/// `s.spchk(c,e,tp,sg)`
fn spchk(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: &CardSet) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and (#sg==1 or sg:IsExists(Card.IsAbleToDeck,1,c))
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && (sg.len() == 1 || group::is_exists(d, sg, &|d, x| card::is_able_to_deck(d, x), 1, c.into()))
}
/// `s.sptdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.sptdfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| sptdfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && g.len() >= 2
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg.clone()), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg), 1, tp, 0);
    true
}
/// `s.sptdop(e,tp,eg,ep,ev,re,r,rp)`
fn sptdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if (#tg==0 or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0) then return end
    if tg.is_empty() || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=tg:FilterSelect(tp,s.spchk,1,1,nil,e,tp,tg)
    let sg = group::filter_select(d, &tg, tp, &|d, c| spchk(d, c, a.e, tp, &tg), 1, 1, false, NO_EX).nil();
    // if #sg>0 and Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)>0 and #tg==2 then
    if !sg.is_empty() && duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 && tg.len() == 2 {
        // local dg=tg-sg
        let mut dg = tg.clone();
        group::remove_card(&mut dg, &sg);
        // Duel.HintSelection(dg)
        duel::hint_selection(d, dg.clone(), true);
        // Duel.SendtoDeck(dg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, dg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
