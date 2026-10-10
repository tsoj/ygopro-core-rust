//! Saryuja Skull Dread
use super::prelude::*;

pub const ID: u32 = 74997493;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    link::add_procedure(d, c, None, 2, None, Some(group_check(lcheck)), None, None);
    card::enable_revive_limit(d, c);
    // effect gain
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(regcon)));
    effect::set_operation(d, e1, Some(operation(regop)));
    card::register_effect(d, c, e1, false);
    // draw
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 2));
    effect::set_category(d, e2, CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(drcon)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop)));
    card::register_effect(d, c, e2, false);
}
fn lcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    // g:CheckDifferentProperty(Card.GetCode,lc,sumtype,tp)
    group::check_different_property(d, g, &|d, c| card::get_code(d, c).into_iter().map(i64::from).collect())
}
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetMaterialCount()>=2 then
    if card::get_material_count(d, c) >= 2 {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, aux::stringid(ID, 0));
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
        effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
        effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
        effect::set_range(d, e1, LOCATION_MZONE);
        // aux.zptcon(Card.IsFaceup)
        effect::set_condition(d, e1, Some(aux::zptcon(Some(|d: &mut Duel, c: CardId, _tp: u8| card::is_faceup(d, c)))));
        effect::set_operation(d, e1, Some(operation(atkop)));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
        card::register_effect(d, c, e2, false);
        // if c:GetMaterialCount()==2 then
        if card::get_material_count(d, c) == 2 {
            card::register_flag_effect(d, c, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 3));
        }
    }
    // if c:GetMaterialCount()>=3 then
    if card::get_material_count(d, c) >= 3 {
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        effect::set_description(d, e3, aux::stringid(ID, 1));
        effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
        effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
        effect::set_range(d, e3, LOCATION_MZONE);
        effect::set_count_limit(d, e3, 1, 0, 0, 0);
        effect::set_target(d, e3, Some(cost_target(sptg)));
        effect::set_operation(d, e3, Some(operation(spop)));
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e3, false);
        if card::get_material_count(d, c) == 3 {
            card::register_flag_effect(d, c, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 4));
        } else {
            card::register_flag_effect(d, c, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 5));
        }
    }
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lg=c:GetLinkedGroup()
    // if not lg then return end  (a group is never nil in Lua, so this never returns)
    let _lg = card::get_linked_group(d, c);
    // local g=aux.zptgroup(eg,Card.IsFaceup,c)  (Lua passes no tp: Card.IsFaceup ignores it)
    let eg = a.eg();
    let g = aux::zptgroup(d, &eg, Some(&|d, c, _tp| card::is_faceup(d, c)), c, a.tp);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 300);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        card::register_effect(d, tc, e2, false);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    // return c:IsLinkSummoned() and c:GetMaterialCount()==4
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c) && card::get_material_count(d, c) == 4
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,4) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 4);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(4)
    duel::set_target_param(d, 4);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,4)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 4);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, a.tp, 3);
    true
}
fn drop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let n = info[1].int() as u32;
    // if Duel.Draw(p,d,REASON_EFFECT)==4 then
    if duel::draw(d, p, n, REASON_EFFECT) == 4 {
        // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,p,LOCATION_HAND,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), p, LOCATION_HAND, 0, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
        // local sg=g:Select(p,3,3,nil)
        let sg = group::select(d, &g, p, 3, 3, false, NO_EX).nil();
        // Duel.SendtoDeck(sg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, sg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        // Duel.SortDeckbottom(p,p,3)
        duel::sort_deckbottom(d, p, p, 3);
    }
}
