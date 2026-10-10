//! Number 90: Galaxy-Eyes Photon Lord
use super::prelude::*;

pub const ID: u32 = 8165596;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PHOTON, SET_GALAXY],
    xyz_number: Some(90),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,8,2)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, None, None, false, None);
    // If this card has a "Photon" card as material, it cannot be destroyed by card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return e:GetHandler():GetOverlayGroup():IsExists(Card.IsSetCard,1,nil,SET_PHOTON) end)
    effect::set_condition(d, e1, Some(condition(indes_cond)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate an opponent's monster effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp,chk) return rp==1-tp and re:IsMonsterEffect() and Duel.IsChainDisablable(ev) end)
    effect::set_condition(d, e2, Some(condition(negate_cond)));
    // e2:SetCost(Cost.DetachFromSelf(1,1,function(e,og) e:SetLabel(og:GetFirst():IsSetCard(SET_GALAXY) and 1 or 0) end))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), Some(std::sync::Arc::new(detach_op)))));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Take 1 "Photon" or "Galaxy" card from your Deck, and either add it to your hand or attach it to this card as material
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e3, Some(condition(thmat_cond)));
    // e3:SetTarget(s.thmattg)
    effect::set_target(d, e3, Some(cost_target(thmattg)));
    // e3:SetOperation(s.thmatop)
    effect::set_operation(d, e3, Some(operation(thmatop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():GetOverlayGroup():IsExists(Card.IsSetCard,1,nil,SET_PHOTON) end`
fn indes_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_overlay_group(d, h);
    group::is_exists(d, &g, &|d, c| card::is_set_card(d, c, &[SET_PHOTON as u16]), 1, NO_EX)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) return rp==1-tp and re:IsMonsterEffect() and Duel.IsChainDisablable(ev) end`
fn negate_cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && duel::is_chain_disablable(d, a.ev as u8)
}
/// `function(e,og) e:SetLabel(og:GetFirst():IsSetCard(SET_GALAXY) and 1 or 0) end`
fn detach_op(d: &mut Duel, e: EffectId, og: &CardSet) {
    let tc = group::get_first(d, og).nil();
    let v = card::is_set_card(d, tc, &[SET_GALAXY as u16]);
    effect::set_label(d, e, &[v as i64]);
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) and e:GetLabel()==1 then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, a.re.nil()) && effect::get_label(d, a.e) == 1 {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) and e:GetLabel()==1 then
    if duel::negate_effect(d, a.ev as u8) {
        let rh = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rh, a.re.nil()) && effect::get_label(d, a.e) == 1 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) end`
fn thmat_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.thmatfilter(c,hc,tp,relation_chk)`
fn thmatfilter(d: &mut Duel, c: CardId, hc: CardId, tp: u8, relation_chk: bool) -> bool {
    // return c:IsSetCard({SET_PHOTON,SET_GALAXY}) and (c:IsAbleToHand() or (relation_chk and c:IsCanBeXyzMaterial(hc,tp,REASON_EFFECT)))
    card::is_set_card(d, c, &[SET_PHOTON as u16, SET_GALAXY as u16])
        && (card::is_able_to_hand(d, c, None) || (relation_chk && card::is_can_be_xyz_material(d, c, Some(hc), Some(tp), Some(REASON_EFFECT))))
}
/// `s.thmattg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thmattg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thmatfilter,tp,LOCATION_DECK,0,1,nil,e:GetHandler(),tp,true) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| thmatfilter(d, c, h, a.tp, true)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thmatop(e,tp,eg,ep,ev,re,r,rp)`
fn thmatop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local relation_chk=c:IsRelateToEffect(e)
    let relation_chk = card::is_relate_to_effect(d, c, a.e);
    // local desc=relation_chk and aux.Stringid(id,2) or HINTMSG_ATOHAND
    let desc = if relation_chk { aux::stringid(ID, 2) } else { HINTMSG_ATOHAND as u64 };
    // Duel.Hint(HINT_SELECTMSG,tp,desc)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, desc);
    // local sc=Duel.SelectMatchingCard(tp,s.thmatfilter,tp,LOCATION_DECK,0,1,1,nil,c,tp,relation_chk):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| thmatfilter(d, x, c, a.tp, relation_chk)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if not sc then return end
    let Some(sc) = sc else {
        return;
    };
    // aux.ToHandOrElse(sc,tp,
    //     function() return relation_chk and sc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT) end,
    //     function() Duel.Overlay(c,sc) end,
    //     aux.Stringid(id,3)
    // )
    aux::to_hand_or_else(
        d,
        sc,
        a.tp,
        Some(&|d, _x| relation_chk && card::is_can_be_xyz_material(d, sc, Some(c), Some(a.tp), Some(REASON_EFFECT))),
        Some(&|d, _g| {
            duel::overlay(d, c, sc, false);
            0
        }),
        Some(aux::stringid(ID, 3)),
    );
}
