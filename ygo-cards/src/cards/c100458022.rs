//! Raise Moon Sky Cielo - No More Bets
use super::prelude::*;

pub const ID: u32 = 100458022;

// (The Lua's `s.listed_seriess={SET_RAISE_MOON}` misspells `listed_series`: it declares nothing, so there is no INFO.)

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 7 "Raise Moon" monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_RAISE_MOON),7,2)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, _sc, _st, _tp| card::is_set_card(d, c, &[SET_RAISE_MOON as u16]))),
        Some(7),
        2,
        None,
        None,
        None,
        None,
        false,
        None,
    );
    // Cannot be used as material for a Fusion, Xyz, or Link Summon
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_CANNOT_BE_MATERIAL)
    effect::set_code(d, e0, EFFECT_CANNOT_BE_MATERIAL);
    // e0:SetValue(aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK))
    effect::set_value_fn(d, e0, aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK]));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // If this card in your possession is sent to your GY by an opponent's card: You can draw 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.drcon)
    effect::set_condition(d, e1, Some(condition(drcon)));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(draw_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains these effects while in the Extra Monster Zone
    // ● Unaffected by other cards' effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetRange(LOCATION_EMZONE)
    effect::set_range(d, e2, LOCATION_EMZONE);
    // e2:SetValue(function(e,te) return te:GetOwner()~=e:GetOwner() end)
    effect::set_value_fn(d, e2, lua_fn(|d, a| {
        let te_owner = effect::get_owner(d, a.e(1));
        let e_owner = effect::get_owner(d, a.e(0));
        te_owner != e_owner
    }));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // ● When a card or effect activated by your opponent resolves that includes an effect that adds a card(s) from the Deck to the hand, you can make that effect become "Draw 1 card"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    // e3:SetRange(LOCATION_EMZONE)
    effect::set_range(d, e3, LOCATION_EMZONE);
    // e3:SetCondition(s.changecon)
    effect::set_condition(d, e3, Some(condition(changecon)));
    // e3:SetOperation(s.changeop)
    effect::set_operation(d, e3, Some(operation(changeop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // ● Once per turn (Quick Effect): You can detach 1 material from this card; draw 1 card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e4, CATEGORY_DRAW);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_EMZONE)
    effect::set_range(d, e4, LOCATION_EMZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e4, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e4:SetTarget(s.drtg)
    effect::set_target(d, e4, Some(cost_target(drtg)));
    // e4:SetOperation(s.drop)
    effect::set_operation(d, e4, Some(operation(draw_op)));
    // e4:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e4, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousControler(tp) and rp==1-tp
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_controler(d, h, a.tp) && a.rp == 1 - a.tp
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn draw_op(d: &mut Duel, a: &EffArgs) {
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, a.tp, 1, REASON_EFFECT);
}
/// `s.check(ev,re)(category,checkloc)`
fn check(d: &mut Duel, ev: u32, re: Option<EffectId>, category: u64, checkloc: bool) -> bool {
    // if not checkloc and re:IsHasCategory(category) then return true end
    if !checkloc && effect::is_has_category(d, re.nil(), category) {
        return true;
    }
    // local ex1,g1,gc1,dp1,dv1=Duel.GetOperationInfo(ev,category)
    let op1 = duel::get_operation_info(d, ev, category);
    // local ex2,g2,gc2,dp2,dv2=Duel.GetPossibleOperationInfo(ev,category)
    let op2 = duel::get_possible_operation_info(d, ev, category);
    // if not (ex1 or ex2) then return false end
    if op1.is_none() && op2.is_none() {
        return false;
    }
    // if category==CATEGORY_DRAW then return true end
    if category == CATEGORY_DRAW {
        return true;
    }
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // if g1 then g:Merge(g1) end
    if let Some(cards) = op1.as_ref().and_then(|o| o.op_cards.clone()) {
        group::add_card(&mut g, &cards);
    }
    // if g2 then g:Merge(g2) end
    if let Some(cards) = op2.as_ref().and_then(|o| o.op_cards.clone()) {
        group::add_card(&mut g, &cards);
    }
    // return (((dv1 or 0)|(dv2 or 0))&LOCATION_DECK)~=0 or (#g>0 and g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK))
    let dv1 = op1.as_ref().map_or(0, |o| o.op_param);
    let dv2 = op2.as_ref().map_or(0, |o| o.op_param);
    let dv = (dv1 | dv2) as u32;
    (dv & LOCATION_DECK) != 0 || (!g.is_empty() && group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX))
}
/// `s.changecon(e,tp,eg,ep,ev,re,r,rp)`
fn changecon(d: &mut Duel, a: &EffArgs) -> bool {
    // if ep==tp then return false end
    if a.ep == a.tp {
        return false;
    }
    // local checkfunc=s.check(ev,re)
    // return checkfunc(CATEGORY_TOHAND,true) or checkfunc(CATEGORY_DRAW,true) or checkfunc(CATEGORY_DRAW,false) or checkfunc(CATEGORY_SEARCH,false)
    check(d, a.ev, a.re, CATEGORY_TOHAND, true)
        || check(d, a.ev, a.re, CATEGORY_DRAW, true)
        || check(d, a.ev, a.re, CATEGORY_DRAW, false)
        || check(d, a.ev, a.re, CATEGORY_SEARCH, false)
}
/// `s.changeop(e,tp,eg,ep,ev,re,r,rp)`
fn changeop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerCanDraw(1-tp,1) and Duel.SelectEffectYesNo(tp,e:GetHandler(),aux.Stringid(id,2)) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::is_player_can_draw(d, 1 - a.tp, 1) && duel::select_effect_yes_no(d, a.tp, h, Some(aux::stringid(ID, 2))) {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // local g=Group.CreateGroup()
        // Duel.ChangeTargetCard(ev,g)
        duel::change_target_card(d, a.ev as u8, &CardSet::new());
        // Duel.ChangeChainOperation(ev,s.drop)
        duel::change_chain_operation(d, a.ev as u8, operation(draw_op));
    }
}
