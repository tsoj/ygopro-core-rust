//! Avida, Rebuilder of Worlds
use super::prelude::*;

pub const ID: u32 = 17469113;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon procedure
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND)
    effect::set_range(d, e0, LOCATION_HAND);
    // e0:SetCondition(s.spcon)
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot negate its Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_CANNOT_DISABLE_SPSUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_DISABLE_SPSUMMON);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Check/apply Special Summon restriction
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EFFECT_SPSUMMON_COST)
    effect::set_code(d, e3, EFFECT_SPSUMMON_COST);
    // e3:SetCost(function(_,_,tp) return Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)==0 end)
    effect::set_cost(d, e3, Some(lua_fn(|d, a| duel::get_activity_count(d, a.p(2), &[ACTIVITY_SPSUMMON]).first() == Some(&0))));
    // e3:SetOperation(s.spcostop)
    effect::set_operation(d, e3, Some(operation(spcostop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Shuffle all other monsters that are banished, on the field, and in the GYs into the Deck
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e4, CATEGORY_TODECK);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetTarget(s.tdtg)
    effect::set_target(d, e4, Some(cost_target(tdtg)));
    // e4:SetOperation(s.tdop)
    effect::set_operation(d, e4, Some(operation(tdop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // if Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)<=0 then return false end
    let tp = card::get_controler(d, c);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return false;
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsLinkMonster),0,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(
        d,
        Some(&|d, x| card::is_faceup(d, x) && card::is_link_monster(d, x)),
        0,
        LOCATION_MZONE | LOCATION_GRAVE,
        LOCATION_MZONE | LOCATION_GRAVE,
        NO_EX,
    );
    // return g:GetClassCount(Card.GetCode)>=8
    group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64) >= 8
}
// function s.spcostop(e,tp,eg,ep,ev,re,r,rp)
fn spcostop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and (c:IsFaceup() or not c:IsLocation(LOCATION_REMOVED))
    card::is_monster(d, c) && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local LOCATION_MZONE_GRAVE_REMOVED=LOCATION_MZONE|LOCATION_GRAVE|LOCATION_REMOVED
    let loc = LOCATION_MZONE | LOCATION_GRAVE | LOCATION_REMOVED;
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_MZONE_GRAVE_REMOVED,LOCATION_MZONE_GRAVE_REMOVED,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&tdfilter), tp, loc, loc, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, tp, 0);
    // Duel.SetChainLimit(aux.FALSE)
    duel::set_chain_limit(d, aux_false());
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp,chk)
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local LOCATION_MZONE_GRAVE_REMOVED=LOCATION_MZONE|LOCATION_GRAVE|LOCATION_REMOVED
    let loc = LOCATION_MZONE | LOCATION_GRAVE | LOCATION_REMOVED;
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_MZONE_GRAVE_REMOVED,LOCATION_MZONE_GRAVE_REMOVED,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&tdfilter), tp, loc, loc, h.into());
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
