//! Nekroz of Areadbhair
//! Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 39468724;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEKROZ], mat_filter: Some(mat_filter), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must be ritual summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.ritlimit)
    effect::set_value_fn(d, e1, aux::ritlimit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Tribute up to 2 "Nekroz" monsters, send that many "Nekroz" cards from deck to GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e2, Some(cost::self_discard()));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Negate the activation of monster effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_NEGATE+CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_NEGATE + CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.negcon)
    effect::set_condition(d, e3, Some(condition(negcon)));
    // e3:SetCost(s.negcost)
    effect::set_cost(d, e3, Some(cost_target(negcost)));
    // e3:SetTarget(s.negtg)
    effect::set_target(d, e3, Some(cost_target(negtg)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// --Cannot include level 10 monsters for its ritual summon
// function s.mat_filter(c)
fn mat_filter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    // return c:GetLevel()~=10
    card::get_level(d, c) != 10
}
// --Check for "Nekroz" monsters to tribute
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NEKROZ) and c:IsMonster() and c:IsReleasableByEffect()
    card::is_set_card(d, c, &[SET_NEKROZ as u16]) && card::is_monster(d, c) && card::is_releasable_by_effect(d, c, None, None)
}
// --Check for "Nekroz" cards to send to GY
// function s.sendfilter(c)
fn sendfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NEKROZ) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_NEKROZ as u16]) && card::is_able_to_grave(d, c, None)
}
// --Activation legality
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.sendfilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.CheckReleaseGroupEx(tp,s.filter,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&sendfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::check_release_group_ex(d, tp, Some(&filter), 1, duel::ReleaseArgs::default(), h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
// --Tribute up to 2 "Nekroz" monsters, send that many "Nekroz" cards from deck to GY
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ac=Duel.GetMatchingGroupCount(s.sendfilter,tp,LOCATION_DECK,0,nil)
    let mut ac = duel::get_matching_group_count(d, Some(&sendfilter), tp, LOCATION_DECK, 0, NO_EX) as u16;
    // if ac==0 then return end
    if ac == 0 {
        return;
    }
    // if ac>2 then ac=2 end
    if ac > 2 {
        ac = 2;
    }
    // local g=Duel.SelectReleaseGroupEx(tp,s.filter,1,ac,nil)
    let g = duel::select_release_group_ex(d, tp, Some(&filter), 1, ac, duel::ReleaseArgs::default(), NO_EX);
    // if g and #g>0 then
    if let Some(g) = g {
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // local rct=Duel.Release(g,REASON_EFFECT)
            let rct = duel::release(d, g, REASON_EFFECT, None) as u16;
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
            // local sg=Duel.SelectMatchingCard(tp,s.sendfilter,tp,LOCATION_DECK,0,rct,rct,nil)
            let sg = duel::select_matching_card(d, tp, Some(&sendfilter), tp, LOCATION_DECK, 0, rct, rct, false, NO_EX).nil();
            // Duel.SendtoGrave(sg,REASON_EFFECT)
            duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
        }
    }
}
// --Monster effect activated
// function s.negcon(e,tp,eg,ep,ev,re,r,rp)
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return re:IsMonsterEffect() and not c:IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)
    let re = a.re.nil();
    effect::is_monster_effect(d, re) && !card::is_status(d, c, STATUS_BATTLE_DESTROYED) && duel::is_chain_negatable(d, a.ev as u8)
}
// --Tribute 1 monster from hand or field as cost
// function s.negcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,1,true,nil,nil) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, tp, None, 1, 1, true, None, NO_EX);
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,nil,1,1,true,nil,nil)
    let sg = aux::select_release_group_cost(d, tp, None, 1, 1, true, None, NO_EX);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    true
}
// --Activation legality
// function s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // local relation=rc:IsRelateToEffect(re)
    let relation = card::is_relate_to_effect(d, rc, re);
    // if chk==0 then return rc:IsAbleToRemove(tp)
    //     or (not relation and Duel.IsPlayerCanRemove(tp)) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, rc, Some(tp), None, None) || (!relation && duel::is_player_can_remove(d, tp, None));
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    if relation {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rc,1,rc:GetControler(),rc:GetLocation())
        let p = card::get_controler(d, rc);
        let loc = card::get_location(d, rc);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, p, loc as i32);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,0,rc:GetPreviousLocation())
        let loc = card::get_previous_location(d, rc);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, 0, loc as i32);
    }
    true
}
// --Negate the activation of monster effect, and if you do, banish that card
// function s.negop(e,tp,eg,ep,ev,re,r,rp)
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if Duel.NegateActivation(ev) and rc:IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, rc, re) {
        // Duel.Remove(rc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, rc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
