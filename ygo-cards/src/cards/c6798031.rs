//! Ryzeal Cross
use super::prelude::*;

pub const ID: u32 = 6798031;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RYZEAL], listed_names: &[ID], ..ScriptInfo::DEFAULT };

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
    // You cannot Xyz Summon monsters with the same name as a card you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.xyzlimit)
    effect::set_target(d, e1, Some(lua_fn(|d, a| xyzlimit(d, a))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 2 "Ryzeal" cards from your GY on the bottom of the Deck then draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Detach 1 material from a "Ryzeal" Xyz Monster you control, and if you do, negate an opponent's activated monster effect when it resolves
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCondition(s.negcon)
    effect::set_condition(d, e3, Some(condition(negcon)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.xyzlimit(e,c,sump,sumtyp)
fn xyzlimit(d: &mut Duel, a: Args<'_>) -> bool {
    // return (sumtyp&SUMMON_TYPE_XYZ)==SUMMON_TYPE_XYZ and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,c:GetCode()),sump,LOCATION_ONFIELD,0,1,nil)
    let c = a.c(1);
    let sump = a.p(2);
    let sumtyp = a.int(3) as u64;
    let xyz = SUMMON_TYPE_XYZ as u64;
    if (sumtyp & xyz) != xyz {
        return false;
    }
    let codes = card::get_code(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &codes)), sump, LOCATION_ONFIELD, 0, 1, NO_EX)
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RYZEAL) and c:IsAbleToDeck() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_RYZEAL as u16]) && card::is_able_to_deck(d, c) && !card::is_code(d, c, &[ID])
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1) && duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE,0,2,2,nil)
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 2, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 or Duel.SendtoDeck(tg,nil,SEQ_DECKBOTTOM,REASON_EFFECT)==0 then return end
    if tg.is_empty() || duel::send_to_deck(d, tg, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) == 0 {
        return;
    }
    // local dg=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_DECK|LOCATION_EXTRA)
    let op = duel::get_operated_group(d);
    let dg = group::filter(d, &op, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    // if #dg==0 then return end
    if dg.is_empty() {
        return;
    }
    // local ct=dg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)
    let ct = group::filter_count(d, &dg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // if ct>1 then
    if ct > 1 {
        // Duel.SortDeckbottom(tp,tp,ct)
        duel::sort_deckbottom(d, tp, tp, ct as u16);
    }
    // if Duel.IsPlayerCanDraw(tp) then
    if duel::is_player_can_draw(d, tp, 0) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
// function s.xyzfilter(c)
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RYZEAL) and c:IsType(TYPE_XYZ) and c:GetOverlayCount()>0 and c:IsFaceup()
    card::is_set_card(d, c, &[SET_RYZEAL as u16]) && card::is_type(d, c, TYPE_XYZ) && card::get_overlay_count(d, c) > 0 && card::is_faceup(d, c)
}
// function s.negcon(e,tp,eg,ep,ev,re,r,rp)
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&xyzfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return rp==1-tp and re:IsMonsterEffect() and Duel.IsChainDisablable(ev) and not e:GetHandler():HasFlagEffect(id)
    //     and #g>0 and Duel.CheckRemoveOverlayCard(tp,1,0,1,REASON_EFFECT,g)
    a.rp == 1 - tp
        && effect::is_monster_effect(d, a.re.nil())
        && duel::is_chain_disablable(d, a.ev as u8)
        && !card::has_flag_effect(d, effect::get_handler(d, a.e).nil(), ID, None)
        && !g.is_empty()
        && duel::check_remove_overlay_card(d, tp, 1, 0, 1, REASON_EFFECT, Some(&g))
}
// function s.negop(e,tp,eg,ep,ev,re,r,rp)
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&xyzfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local cid=Duel.GetChainInfo(ev,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID])[0].int();
    // if not (Duel.GetFlagEffectLabel(tp,id)~=cid and #g>0 and Duel.SelectEffectYesNo(tp,c)) then return end
    let label = duel::get_flag_effect_label(d, tp, ID).first().copied();
    if !(label != Some(cid) && !g.is_empty() && duel::select_effect_yes_no(d, tp, c, None)) {
        return;
    }
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.RemoveOverlayCard(tp,1,0,1,1,REASON_EFFECT,g)
    duel::remove_overlay_card(d, tp, 1, 0, 1, 1, REASON_EFFECT, Some(&g));
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
    // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1,cid)
    duel::register_flag_effect(d, tp, ID, RESET_CHAIN, 0, 1, cid as u32);
}
